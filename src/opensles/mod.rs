#![allow(dead_code)]

use std::marker::PhantomData;
use opensles;
use opensles::bindings::*;
use CreationError;
use DefaultFormatError;
use Format;
use FormatsEnumerationError;
use StreamData;
use SupportedFormat;
use std::{cmp, ffi, iter, mem, ptr};
pub struct EventLoop(SLObjectItf);

impl EventLoop {
    #[inline]
    pub fn new() -> EventLoop {
        let o = SLObjectItf();
        EventLoop(o)
    }

    #[inline]
    pub fn run<F>(&mut self, _callback: F) -> !
        where F: FnMut(StreamId, StreamData)
    {
        let engineMixIIDs = SL_IID_ENGINE;
        let mut result = slCreateEngine(self.0,0,None,1,true);
        let bu = SLBufferQueueItf_();
        let bqPlayerBufferQueue;
        result = bu.RegisterCallback(bqPlayerBufferQueue,Some())
    }

    #[inline]
    pub fn build_input_stream(&self, _: &Device, _: &Format) -> Result<StreamId, CreationError> {
        
    }

    #[inline]
    pub fn build_output_stream(&self, device: &Device, format: &Format) -> Result<StreamId, CreationError> {
        unsafe{
            let mut playback_handle = mem::uninitialized();
            let bu = SLBufferQueueItf_();
            let res = match bu.RegisterCallback(bqPlayerBufferQueue,playback_handle).unwrap(){
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
