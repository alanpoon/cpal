#![allow(dead_code)]
extern crate libc;
use std::marker::PhantomData;
use opensles::bindings::*;
use CreationError;
use DefaultFormatError;
use Format;
use FormatsEnumerationError;
use StreamData;
use SupportedFormat;
use std::{cmp, ffi, iter, mem, ptr};
use opensles::bindings::{SLAndroidSimpleBufferQueueItf};
use libc::{c_int};
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;
pub struct EventLoop{
    active_callbacks: Arc<ActiveCallbacks>,
    streams: Mutex<Vec<Option<StreamInner>>>,
}
fn nextIndex(index:i32, increment:i32)->i32 {
  // Handle potential integer overflow.
  return (std::i32::MAX - index >= increment) ? index + increment; : 0;
}
pub struct OPENSL_STREAM {
    callback: Box<FnMut(StreamId, StreamData) + Send>,
    sample_rate:i32,
    input_channels: i32,
    output_channels:i32,
    callback_buffer_size:i32,
    
}
struct ActiveCallbacks {
    // Whenever the `run()` method is called with a callback, this callback is put in this list.
    callbacks: Mutex<Vec<&'static mut (FnMut(StreamId, StreamData) + Send)>>,
}

extern "C" fn c_render_callback(queue: SLAndroidSimpleBufferQueueItf, void_context: *mut std::os::raw::c_void) {
 let closure = *void_context.closure;
}
extern "C" fn c_record_callback(bq: SLAndroidSimpleBufferQueueItf, context: *mut std::os::raw::c_void) {
  c_int STARTUP_INTERVALS=8;
  OPENSL_STREAM *p = (OPENSL_STREAM *) context;
  if (p->outputChannels) {
    if (p->inputIntervals < STARTUP_INTERVALS) {
      updateIntervals(&p->inputTime, p->thresholdMillis, &p->inputIntervals,
          &p->inputOffset, &p->previousInputIndex, p->inputIndex);
    }
  } else {
    p->callback(p->context, p->sampleRate, p->callbackBufferFrames,
        p->inputChannels, p->inputBuffer +
        (p->inputIndex % p->inputBufferFrames) * p->inputChannels,
        0, NULL);
  }
  __sync_bool_compare_and_swap(&p->inputIndex, p->inputIndex,
      nextIndex(p->inputIndex, p->callbackBufferFrames));
  (*bq)->Enqueue(bq, p->inputBuffer +
      (p->inputIndex % p->inputBufferFrames) * p->inputChannels,
      p->callbackBufferFrames * p->inputChannels * sizeof(short));
      p->callback(&p.streamid,&p.streamdata);
}
impl EventLoop {
    #[inline]
    pub fn new() -> EventLoop {
        EventLoop {
            active_callbacks: Arc::new(ActiveCallbacks { callbacks: Mutex::new(Vec::new()) }),
            streams: Mutex::new(Vec::new()),
        }
    }

    #[inline]
    pub fn run<F>(&mut self, callback: F) -> !
        where F: FnMut(StreamId, StreamData) + Send
    {
        let engineMixIIDs = SL_IID_ENGINE;
        let mut result = slCreateEngine(engineMixIIDs,0,None,1,true);

        let callback: &mut (FnMut(StreamId, StreamData) + Send) = &mut callback;
        self.active_callbacks
            .callbacks
            .lock()
            .unwrap()
            .push(unsafe { mem::transmute(callback) });

        loop {
            // So the loop does not get optimised out in --release
            thread::sleep(Duration::new(1u64, 0u32));
        }
    }

    #[inline]
    pub fn build_input_stream(&self, device: &Device, _: &Format) -> Result<StreamId, CreationError> {
        if (self.inputChannels < 0 || self.inputChannels > 2) {
            return Err(CreationError::DeviceNotAvailable);
        }
        let loc_dev:SLDataLocator_IODevice_ = SLDataLocator_IODevice_{
            locatorType:SL_IID_3DLOCATION,
            deviceType:SL_IODEVICE_AUDIOINPUT,
            deviceID:SL_DEFAULTDEVICEID_AUDIOINPUT,
            device:None
        };
        let audioSrc:SLDataSource_ = SLDataSource_{
            pLocator:&loc_dev, 
            pFormat:None};  // source: microphone

        let mut mics=0;
        if (self.inputChannels > 1) {
            // Yes, we're using speaker macros for mic config.  It's okay, really.
            mics = SL_SPEAKER_FRONT_LEFT | SL_SPEAKER_FRONT_RIGHT;
        } else {
            mics = SL_SPEAKER_FRONT_CENTER;
        }
        let loc_bq =SL_IID_ANDROIDSIMPLEBUFFERQUEUE;
        let format_pcm = SLDataFormat_PCM{
            formatType:SL_DATAFORMAT_PCM,
            numChannels:self.inputChannels,
            samplesPerSec:sr,
            bitsPerSample:ANDROID_KEY_PCMFORMAT_BITSPERSAMPLE,
            containerSize:ANDROID_KEY_PCMFORMAT_CONTAINERSIZE,
            channelMask:mics,
            endianness:ANDROID_KEY_PCMFORMAT_ENDIANNESS
        };

        let audioSnk = SLDataSink{&loc_bq, &format_pcm};  // sink: buffer queue

        // create audio recorder (requires the RECORD_AUDIO permission)
        let id = SL_IID_ANDROIDSIMPLEBUFFERQUEUE;
        let req = SL_BOOLEAN_TRUE;

        let mut result = self.engineEngine.CreateAudioRecorder(
            self.engineEngine, &self.recorderObject, &audioSrc, &audioSnk, 1, id, req);
        if (SL_RESULT_SUCCESS != result) return Err(CreationError::DeviceNotAvailable);

        let result = self.recorderObject.Realize(self.recorderObject, SL_BOOLEAN_FALSE);
        if (SL_RESULT_SUCCESS != result) return Err(CreationError::DeviceNotAvailable);

        let result = self.recorderObject.GetInterface(self.recorderObject,
            SL_IID_RECORD, &p->recorderRecord);
        if (SL_RESULT_SUCCESS != result) return Err(CreationError::DeviceNotAvailable);

        let result = self.recorderObject.GetInterface(
            self.recorderObject, SL_IID_ANDROIDSIMPLEBUFFERQUEUE,
            &self.recorderBufferQueue);
        if (SL_RESULT_SUCCESS != result) return Err(CreationError::DeviceNotAvailable);

        let result = self.recorderBufferQueue.RegisterCallback(
            self.recorderBufferQueue, c_record_callback, self);
        let stream_id = self.next_stream_id();
        let active_callbacks = self.active_callbacks.clone();
        Ok(StreamId(stream_id))
    }

    #[inline]
    pub fn build_output_stream(&self, device: &Device, format: &Format) -> Result<StreamId, CreationError> {
        
        unsafe{
            let bu = SLBufferQueueItf_();
            let active_callbacks = self.active_callbacks.clone();
            let res = match bu.RegisterCallback.unwrap()(bu,Some(c_render_callback),content as *mut _ as *mut std::os::raw::c_void).unwrap(){
                SLresult::Success=>{
                    let new_stream_id = StreamId(self.next_stream_id.fetch_add(1, Ordering::Relaxed));
                    assert_ne!(new_stream_id.0, usize::max_value()); // check for overflows
                    let stream_inner = StreamInner {
                        id: new_stream_id.clone(),
                        audio_player: playback_handle,
                        sample_format: format.data_type,
                        num_descriptors: 2,
                        num_channels: format.channels as u16,
                        buffer_len: buffer_len,
                        period_len: period_len,
                    };

                    self.push_command(Command::NewStream(stream_inner));
                    Ok(new_stream_id)
                }
            }
        }
    }

    #[inline]
    pub fn destroy_stream(&self, _: StreamId) {
        unimplemented!()
    }

    #[inline]
    pub fn play_stream(&self, _: StreamId) {
        panic!()
    }

    #[inline]
    pub fn pause_stream(&self, _: StreamId) {
        panic!()
    }
}

enum Command {
    NewStream(StreamInner),
    PlayStream(StreamId),
    PauseStream(StreamId),
    DestroyStream(StreamId),
}

struct RunContext {
    // Descriptors to wait for. Always contains `pending_trigger.read_fd()` as first element.
    descriptors: Vec<libc::pollfd>,
    // List of streams that are written in `descriptors`.
    streams: Vec<StreamInner>,
}

struct StreamInner {
    // The id of the stream.
    id: StreamId,

    // The opensles channel.
    audio_player: *mut SLObjectItf,

    // When converting between file descriptors and `snd_pcm_t`, this is the number of
    // file descriptors that this `snd_pcm_t` uses.
    num_descriptors: usize,

    // Format of the samples.
    sample_format: SampleFormat,

    // Number of channels, ie. number of samples per frame.
    num_channels: u16,

    // Number of samples that can fit in the buffer.
    buffer_len: usize,

    // Minimum number of samples to put in the buffer.
    period_len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StreamId(usize);

#[derive(Default)]
pub struct Devices;

impl Iterator for Devices {
    type Item = Device;

    #[inline]
    fn next(&mut self) -> Option<Device> {
        None
    }
}

#[inline]
pub fn default_input_device() -> Option<Device> {
    None
}

#[inline]
pub fn default_output_device() -> Option<Device> {
    None
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device;

impl Device {
    #[inline]
    pub fn supported_input_formats(&self) -> Result<SupportedInputFormats, FormatsEnumerationError> {
        unimplemented!()
    }

    #[inline]
    pub fn supported_output_formats(&self) -> Result<SupportedOutputFormats, FormatsEnumerationError> {
        unimplemented!()
    }

    #[inline]
    pub fn default_input_format(&self) -> Result<Format, DefaultFormatError> {
        unimplemented!()
    }

    #[inline]
    pub fn default_output_format(&self) -> Result<Format, DefaultFormatError> {
        unimplemented!()
    }

    #[inline]
    pub fn name(&self) -> String {
        "null".to_owned()
    }
}

pub struct SupportedInputFormats;
pub struct SupportedOutputFormats;

impl Iterator for SupportedInputFormats {
    type Item = SupportedFormat;

    #[inline]
    fn next(&mut self) -> Option<SupportedFormat> {
        None
    }
}

impl Iterator for SupportedOutputFormats {
    type Item = SupportedFormat;

    #[inline]
    fn next(&mut self) -> Option<SupportedFormat> {
        None
    }
}

pub struct InputBuffer<'a, T: 'a> {
    marker: PhantomData<&'a T>,
}

pub struct OutputBuffer<'a, T: 'a> {
    marker: PhantomData<&'a mut T>,
}

impl<'a, T> InputBuffer<'a, T> {
    #[inline]
    pub fn buffer(&self) -> &[T] {
        unimplemented!()
    }

    #[inline]
    pub fn finish(self) {
    }
}

impl<'a, T> OutputBuffer<'a, T> {
    #[inline]
    pub fn buffer(&mut self) -> &mut [T] {
        unimplemented!()
    }

    #[inline]
    pub fn len(&self) -> usize {
        0
    }

    #[inline]
    pub fn finish(self) {
    }
}
