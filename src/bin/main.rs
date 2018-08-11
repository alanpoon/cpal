#[macro_use] extern crate conrod;
extern crate android_glue;
extern crate glium;
extern crate image;
extern crate rand;
extern crate rusttype;
extern crate alto;
#[macro_use] extern crate log;
extern crate android_logger;
use log::Level;
use android_logger::Filter;
use std::sync::Arc;
use alto::*;
use conrod::{widget, color, Widget};
use conrod::backend::glium::glium::{Surface};
mod app;
mod assets;
fn native_activity_create() {
    android_logger::init_once(Filter::default().with_min_level(Level::Trace), None);
}
pub fn main() {
    native_activity_create();
    trace!("start");
     std::thread::spawn(move || {
            let alto = if let Ok(alto) = Alto::load_default() {
		alto
	} else {
		trace!("No OpenAL implementation present!");
		return;
	};

	trace!("Using output: {:?}", alto.default_output().unwrap());
	let dev = alto.open(None).unwrap();
	let ctx = dev.new_context(None).unwrap();

	let mut slot = if dev.is_extension_present(alto::ext::Alc::Efx) {
		trace!("Using EFX reverb");
		if let Ok(slot) = (|| -> AltoResult<_> {
			let mut slot = ctx.new_aux_effect_slot()?;
			let mut reverb: efx::EaxReverbEffect = ctx.new_effect()?;
			reverb.set_preset(&efx::REVERB_PRESET_GENERIC)?;
			slot.set_effect(&reverb)?;
			Ok(slot)
		})() {
			Some(slot)
		} else {
			trace!("Broken router detected; disabling EFX");
			None
		}
	} else {
		trace!("EFX not present");
		None
	};

	{
		let buf = ctx.new_buffer(SinWave::new(44_000 / 440, 0.25).render().take(44_000 / 440).collect::<Vec<_>>(), 44_000).unwrap();
		let buf = Arc::new(buf);

		let mut src = ctx.new_static_source().unwrap();
		src.set_buffer(buf).unwrap();
		src.set_looping(true);
		if let Some(ref mut slot) = slot {
			src.set_aux_send(0, slot).unwrap();
		}

		trace!("Playing static 440hz sine wave...");
		src.play();

		std::thread::sleep(std::time::Duration::new(2, 0));
	}

	std::thread::sleep(std::time::Duration::new(1, 0));

	{
		let mut wave = SinWave::new(44_000 / 220, 0.25);

		let mut src = ctx.new_streaming_source().unwrap();
		if let Some(ref mut slot) = slot {
			src.set_aux_send(0, slot).unwrap();
		}
		for _ in 0 .. 5 {
			let buf = ctx.new_buffer(wave.render().take(44_000 / 10).collect::<Vec<_>>(), 44_000).unwrap();
			src.queue_buffer(buf).unwrap();
		}

		trace!("Playing streaming 220hz sine wave...");
		src.play();

		for _ in 0 .. 15 {
			while src.buffers_processed() == 0 { }

			let mut buf = src.unqueue_buffer().unwrap();
			buf.set_data(wave.render().take(44_000 / 10).collect::<Vec<_>>(), 44_000).unwrap();
			src.queue_buffer(buf).unwrap();
		}

		while src.buffers_processed() < 5 { }
	}

	std::thread::sleep(std::time::Duration::new(1, 0));
        });
    let builder = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new()
        .with_gl(glium::glutin::GlRequest::Specific(glium::glutin::Api::OpenGlEs, (3, 0)));
    let mut events_loop = glium::glutin::EventsLoop::new();
    let display = glium::Display::new(builder, context, &events_loop).unwrap();

    let (w, h) = display.get_framebuffer_dimensions();
    let mut ui = conrod::UiBuilder::new([w as f64, h as f64]).theme(app::theme()).build();
    ui.fonts.insert(assets::load_font("LiberationSans-Regular.ttf"));

    let mut image_map: conrod::image::Map<glium::texture::Texture2d> = conrod::image::Map::new();
    let image_rgba = assets::load_image("rust.png").to_rgba();
    let dims = image_rgba.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image_rgba.into_raw(), dims);
    let texture = glium::texture::Texture2d::new(&display, raw_image).unwrap();
    let rust_logo = image_map.insert(texture);

    let mut demo_app = app::DemoApp::new(rust_logo);
    let ids = app::Ids::new(ui.widget_id_generator());

    let (event_tx, event_rx) = std::sync::mpsc::channel();
    let (render_tx, render_rx) = std::sync::mpsc::channel();
    let events_loop_proxy = events_loop.create_proxy();

    std::thread::spawn(move || {
        let mut needs_update = true;
        let mut first = true;
        loop {
            if !first {
                let mut events = Vec::new();
                while let Ok(event) = event_rx.try_recv() {
                    events.push(event);
                }

                if events.is_empty() || !needs_update {
                    match event_rx.recv() {
                        Ok(event) => events.push(event),
                        Err(_) => break
                    }
                }

                needs_update = false;

                for event in events {
                    ui.handle_event(event);
                    needs_update = true;
                }
            }
            else {
                first = false;
            }

            trace!("Drawing the GUI.");
            app::gui(&mut ui.set_widgets(), &ids, &mut demo_app);

            if let Some(primitives) = ui.draw_if_changed() {
                if render_tx.send(primitives.owned()).is_err() || events_loop_proxy.wakeup().is_err() {
                    break;
                }
            }
        }
    });

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let mut last_update = std::time::Instant::now();
    let mut closed = false;
    let mut first = true;
    while !closed {
        let primitives: Option<conrod::render::OwnedPrimitives>;

        if !first {
            // Don't loop more rapidly than 60Hz.
            let sixteen_ms = std::time::Duration::from_millis(16);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }

            events_loop.run_forever(|event| {
                if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                    event_tx.send(event).unwrap();
                }

                match event {
                    glium::glutin::Event::WindowEvent { event, .. } => match event {
                        glium::glutin::WindowEvent::CloseRequested => {
                            closed = true;
                            return glium::glutin::ControlFlow::Break;
                        },
                        glium::glutin::WindowEvent::Resized(..) => {
                            if let Some(primitives) = render_rx.iter().next() {
                                draw(&display, &mut renderer, &image_map, &primitives);
                            }
                        },
                        _ => {},
                    },
                    glium::glutin::Event::Awakened => return glium::glutin::ControlFlow::Break,
                    _ => (),
                }

                glium::glutin::ControlFlow::Continue
            });

            primitives = render_rx.try_iter().last();
        }
        else {
            first = false;
            primitives = render_rx.recv().ok();
        }

        if let Some(primitives) = primitives {
            trace!("Rendering.");
            draw(&display, &mut renderer, &image_map, &primitives);
        }

        last_update = std::time::Instant::now();
    }
}

fn draw(display: &glium::Display,
        renderer: &mut conrod::backend::glium::Renderer,
        image_map: &conrod::image::Map<glium::Texture2d>,
        primitives: &conrod::render::OwnedPrimitives) {
    renderer.fill(display, primitives.walk(), &image_map);
    let mut target = display.draw();
    target.clear_color(1.0, 1.0, 1.0, 1.0);
    renderer.draw(display, &mut target, &image_map).unwrap();
    target.finish().unwrap();
}

struct SinWave {
	len: i32,
	vol: f32,
	cursor: i32,
}

struct SinWaveRenderer<'w>(&'w mut SinWave);


impl SinWave {
	pub fn new(len: i32, vol: f32) -> SinWave {
		SinWave{len: len, vol: vol, cursor: 0}
	}


	pub fn render(&mut self) -> SinWaveRenderer {
		SinWaveRenderer(self)
	}
}


impl<'w> Iterator for SinWaveRenderer<'w> {
	type Item = Mono<i16>;

	fn next(&mut self) -> Option<Mono<i16>> {
		let cursor = self.0.cursor;
		self.0.cursor += 1;
		if self.0.cursor == self.0.len { self.0.cursor = 0 }

		Some(Mono{center: ((cursor as f32 / self.0.len as f32 * 2.0 * std::f32::consts::PI).sin() * self.0.vol * std::i16::MAX as f32) as i16})
	}
}