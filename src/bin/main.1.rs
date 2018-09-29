#[macro_use] extern crate conrod;
extern crate android_glue;
extern crate glium;
extern crate image;
extern crate rand;
extern crate rusttype;
use conrod::{widget, color, Widget};
use conrod::backend::glium::glium::{Surface};
use std::thread;
extern crate opensles;
use opensles::bindings::*;
use std::os::raw::c_void;
use std::ptr;
use std::mem;
pub const SL_DATALOCATOR_IODEVICE:SLuint32 = 0x00000003;
pub const SL_IODEVICE_AUDIOINPUT:SLuint32 = 0x00000001;
pub const SL_DEFAULTDEVICEID_AUDIOINPUT:SLuint32 = 0xFFFFFFFF;
pub const SL_SPEAKER_FRONT_LEFT:SLuint32 = 0x00000001;
pub const SL_SPEAKER_FRONT_RIGHT:SLuint32 = 0x00000002;
pub const SL_SPEAKER_FRONT_CENTER:SLuint32 = 0x00000004;
pub const SL_PCMSAMPLEFORMAT_FIXED_16:SLuint32 = 0x0010;
pub const SL_SAMPLINGRATE_8:SLuint32 = 8000000;
pub const SL_SAMPLINGRATE_11_025:SLuint32 = 11025000;
pub const SL_SAMPLINGRATE_12:SLuint32 = 12000000;
pub const SL_SAMPLINGRATE_16:SLuint32 = 16000000;
pub const SL_SAMPLINGRATE_22_05:SLuint32 =22050000;
pub const SL_SAMPLINGRATE_24:SLuint32 =24000000;
pub const SL_SAMPLINGRATE_32:SLuint32 =32000000;
pub const SL_SAMPLINGRATE_44_1:SLuint32 =44100000;
pub const SL_SAMPLINGRATE_48:SLuint32 =48000000;
pub const SL_SAMPLINGRATE_64:SLuint32 =64000000;
pub const SL_SAMPLINGRATE_88_2:SLuint32 =88200000;
pub const SL_SAMPLINGRATE_96:SLuint32 = 96000000;
pub const SL_SAMPLINGRATE_192:SLuint32 = 192000000;
pub const SL_RESULT_SUCCESS:SLuint32 = 0x00000000;
pub const SL_BOOLEAN_TRUE:SLboolean = 0x00000001;
pub const SL_BOOLEAN_FALSE:SLboolean = 0x00000000;
pub const SL_DATALOCATOR_OUTPUTMIX:SLuint32=0x00000004;
pub const SL_BYTEORDER_LITTLEENDIAN:SLuint32 = 0x00000002;
pub const SL_PLAYSTATE_PLAYING:SLuint32 = 0x00000003;
pub const BUFFER_SIZE_IN_SAMPLES:usize = 256;
mod app;
mod assets;
pub fn main() {
     thread::spawn(move || {
            let bqPlayerBufSize =0;
            let mut curBuffer:usize =0;
            let mut context:Context = unsafe{mem::zeroed()};
            context.curBuffer =0;
            context.sample_rate =200.0;
            context.sample_clock=0.0;
            context.next_value=Box::new(|sample_clock:&mut f32,sample_rate:f32|->f32{
                *sample_clock = (*sample_clock + 1.0) % sample_rate;
                (*sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin()
            });

            loop{
                OpenSLWrap_Init(&mut context);
            }
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

            println!("Drawing the GUI.");
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
            println!("Rendering.");
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
pub struct Context{
    buffer:[[u16;2];512],
    bqPlayerBufferQueue:SLAndroidSimpleBufferQueueItf,
    curBuffer:usize,
    sample_clock:f32,
    sample_rate:f32,
    next_value:Box<Fn(&mut f32,f32)->f32>
}
pub struct ContextInit{
    engineEngine:SLEngineItf,
    engineObject:SLObjectItf,
    outputMixObject:SLObjectItf,
    bqPlayerObject:SLObjectItf,
    bqPlayerPlay:SLPlayItf,
    bqPlayerBufferQueue:SLAndroidSimpleBufferQueueItf,
    bqPlayerMuteSolo:SLMuteSoloItf,
    bqPlayerVolume:SLVolumeItf,
    mSimpleBufferQueueInterface
}
impl ContextInit{
    pub fn new()->ContextInit{
        ContextInit{
            engineEngine:unsafe{mem::zeroed()},
            engineObject:0,
            outputMixObject:0,
            bqPlayerObject:0,
            bqPlayerPlay:unsafe{mem::zeroed()},
            bqPlayerBufferQueue:unsafe{mem::zeroed()},
            bqPlayerMuteSolo:unsafe{mem::zeroed()},
            bqPlayerVolume:unsafe{mem::zeroed()},
            mSimpleBufferQueueInterface:unsafe{mem::zeroed()},
        }
    }
}
extern "C" fn bqPlayerCallback2(bq:SLAndroidSimpleBufferQueueItf,context:*mut c_void){
    let context_struct: &mut Context = unsafe { &mut *(context as *mut Context) };
    assert_eq!(context_struct.bqPlayerBufferQueue,bq);
    let mut next_buffer = context_struct.buffer[context_struct.curBuffer.clone()];
    let next_size = context_struct.buffer[0].len() as u32;
    let state_ptr: *mut c_void = &mut next_buffer as *mut _ as *mut c_void;
    unsafe{
        let result = (**context_struct.bqPlayerBufferQueue).Enqueue.unwrap()(context_struct.bqPlayerBufferQueue,state_ptr, next_size);
        assert_eq!(result,SL_RESULT_SUCCESS);
    }
    context_struct.curBuffer ^= 1;
    audio_callback(context_struct, BUFFER_SIZE_IN_SAMPLES);
}
fn OpenSLWrap_Init2(init_context:&mut ContextInit){
    unsafe{
        let result = slCreateEngine(&mut init_context.engineObject,0,0,0, 0,0);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (**engineObject).Realize.unwrap()(init_context.engineObject,SL_BOOLEAN_FALSE);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let engine_ptr: *mut c_void = engineEngine as *mut c_void;
        let result = (**engineObject).GetInterface.unwrap()(init_context.engineObject,SL_IID_ENGINE,init_context.mSimpleBufferQueueInterface);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (**engineEngine).CreateOutputMix.unwrap()(engineEngine,&mut outputMixObject,0,0,0);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (**outputMixObject).Realize.unwrap()(outputMixObject,SL_BOOLEAN_FALSE);
        assert_eq!(result,SL_RESULT_SUCCESS);
    }
}
fn OpenSLWrap_Init(context:&mut Context)->bool{
    let mut engineObject:SLObjectItf = unsafe{mem::zeroed()};
    let mut engineEngine:SLEngineItf= unsafe{mem::zeroed()};    
    let mut outputMixObject:SLObjectItf= unsafe{mem::zeroed()};
    let mut bqPlayerObject:SLObjectItf= unsafe{mem::zeroed()};
    let mut bqPlayerPlay:SLPlayItf= unsafe{mem::zeroed()};
    let mut bqPlayerBufferQueue:SLAndroidSimpleBufferQueueItf= unsafe{mem::zeroed()};
    let mut bqPlayerMuteSolo:SLMuteSoloItf= unsafe{mem::zeroed()};
    let mut bqPlayerVolume:SLVolumeItf= unsafe{mem::zeroed()};
    let optionnull:*const SLEngineOption = ptr::null();
    let pinterfaceidnull:*const SLInterfaceID = ptr::null();
    let pInterfaceRequirednull:*const SLboolean = ptr::null();
    unsafe{
        let result = slCreateEngine(&mut engineObject,0,optionnull,0, pinterfaceidnull,pInterfaceRequirednull);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (**engineObject).Realize.unwrap()(engineObject,SL_BOOLEAN_FALSE);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let engine_ptr: *mut c_void = engineEngine as *mut c_void;
        let result = (**engineObject).GetInterface.unwrap()(engineObject,SL_IID_ENGINE,engine_ptr);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (**engineEngine).CreateOutputMix.unwrap()(engineEngine,&mut outputMixObject,0,pinterfaceidnull,&0);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (**outputMixObject).Realize.unwrap()(outputMixObject,SL_BOOLEAN_FALSE);
        assert_eq!(result,SL_RESULT_SUCCESS);
    }

    let mut loc_bufq:SLDataLocator_AndroidSimpleBufferQueue = SLDataLocator_AndroidSimpleBufferQueue{
        locatorType:2,
        numBuffers:2
    };
    let mut format_pcm = SLDataFormat_PCM{
        formatType:2,
        numChannels:2,
        samplesPerSec:SL_SAMPLINGRATE_44_1,
        bitsPerSample:SL_PCMSAMPLEFORMAT_FIXED_16,
        containerSize:SL_PCMSAMPLEFORMAT_FIXED_16,
        channelMask:SL_SPEAKER_FRONT_LEFT,
        endianness:SL_BYTEORDER_LITTLEENDIAN
    };
    
    let loc_bufq_ptr:*mut c_void = &mut loc_bufq as *mut _ as *mut c_void;
    let p_format_ptr:*mut c_void = &mut format_pcm as *mut _ as *mut c_void;
    let audioSrc = SLDataSource_{
        pLocator:loc_bufq_ptr,
        pFormat:p_format_ptr
    };
    // configure audio sink
    let mut loc_outmix =SLDataLocator_OutputMix{
        locatorType:SL_DATALOCATOR_OUTPUTMIX,
        outputMix:outputMixObject
    };
    let loc_outmix_ptr:*mut c_void = &mut loc_outmix as *mut _ as *mut c_void;
    let pformatnull:*mut c_void = ptr::null_mut();
    let audioSnk = SLDataSink{
        pLocator:loc_outmix_ptr,
        pFormat:pformatnull
    };
    let bqPlayerPlay_ptr:*mut c_void = bqPlayerPlay as *mut c_void;
    let bqPlayerBufferQueue_ptr:*mut c_void = context.bqPlayerBufferQueue as *mut c_void;
    let register_null:*mut c_void = ptr::null_mut();
    let slvolume_ptr:*mut c_void = bqPlayerVolume as *mut c_void;
    unsafe{
        let ids:[SLInterfaceID;2]=[SL_IID_BUFFERQUEUE,SL_IID_VOLUME];
        let req:[SLboolean;2]=[SL_BOOLEAN_TRUE,SL_BOOLEAN_TRUE];
        let result = (**bqPlayerObject).Realize.unwrap()(bqPlayerObject,SL_BOOLEAN_FALSE);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (**bqPlayerObject).GetInterface.unwrap()(bqPlayerObject,SL_IID_PLAY,bqPlayerPlay_ptr);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (**bqPlayerObject).GetInterface.unwrap()(bqPlayerObject,SL_IID_BUFFERQUEUE,bqPlayerBufferQueue_ptr);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (**context.bqPlayerBufferQueue).RegisterCallback.unwrap()(context.bqPlayerBufferQueue,Some(bqPlayerCallback2),register_null);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (**bqPlayerObject).GetInterface.unwrap()(bqPlayerObject, SL_IID_VOLUME, slvolume_ptr);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (**bqPlayerPlay).SetPlayState.unwrap()(bqPlayerPlay,SL_PLAYSTATE_PLAYING);
        assert_eq!(result,SL_PLAYSTATE_PLAYING);
        context.curBuffer = 0;
        let buff = context.buffer[context.curBuffer];
        let buff_box = Box::new(buff);
        let buff_ptr = Box::into_raw(buff_box) as *const c_void;
        let result = (**context.bqPlayerBufferQueue).Enqueue.unwrap()(context.bqPlayerBufferQueue,buff_ptr, context.buffer[context.curBuffer].len() as u32);
        if SL_RESULT_SUCCESS != result {
            return false;
        }
        context.curBuffer ^= 1;
        return true;
    }
}

fn OpenSLWrap_Shutdown(engineObject:&mut SLObjectItf,engineEngine:&mut SLEngineItf,outputMixObject:&mut SLObjectItf,bqPlayerObject:&mut SLObjectItf,
bqPlayerPlay:&mut SLPlayItf,bqPlayerVolume:&mut SLVolumeItf,
bqPlayerMuteSolo:&mut SLMuteSoloItf,
context:&mut Context){
    if !bqPlayerObject.is_null(){
        *bqPlayerObject = ptr::null();
        *bqPlayerPlay = ptr::null();
        context.bqPlayerBufferQueue = ptr::null();
        *bqPlayerMuteSolo = ptr::null();
        *bqPlayerVolume = ptr::null();
    }
    if !outputMixObject.is_null(){
        *outputMixObject = ptr::null();
    }
    if !engineObject.is_null(){
        *engineObject = ptr::null();
        *engineEngine = ptr::null();
    }
    //I am converting some c++ code to rust. I can certainty use optional value. but I want to know what happens if I equal *bqPlayerObject = stdtr::null()
}
fn audio_callback(context_struct:&mut Context, num_samples:usize){
    let sample_rate = context_struct.sample_rate.clone();
    for sample in context_struct.buffer[context_struct.curBuffer].chunks_mut(num_samples){
        let value = (((*context_struct.next_value)(&mut context_struct.sample_clock,sample_rate) * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
    }
}