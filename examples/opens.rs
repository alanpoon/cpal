extern crate opensles;
use opensles::bindings::*;
use std::os::raw::c_void;
use std::ptr;
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
pub const SL_BYTEORDER_LITTLEENDIAN:SLuint32 = 0x00000002;
fn main() {
    let engineObject:SLObjectItf;
    let mut engineEngine:SLEngineItf;    
    let outputMixObject:SLObjectItf;

    // buffer queue player interfaces
    let bqPlayerObject:SLObjectItf;
    let bqPlayerPlay:SLPlayItf;
    let mut bqPlayerBufferQueue:SLAndroidSimpleBufferQueueItf;
    let bqPlayerMuteSolo:SLMuteSoloItf;
    let bqPlayerVolume:SLVolumeItf;
    let bqPlayerBufSize =0;
    let buffer:[[u16;2];512];
    let mut curBuffer:usize =0;

}
fn bqPlayerCallback(bq:SLAndroidSimpleBufferQueueItf, 
//context as *mut _ as *mut std::os::raw::c_void,
bqPlayerBufferQueue:&mut SLAndroidSimpleBufferQueueItf,
buffer:&mut [[u16;2];512],curBuffer:&mut usize ) {
    assert_eq!(*bqPlayerBufferQueue,bq);
    let mut next_buffer = buffer[curBuffer.clone()];
    let next_size = buffer[0].len() as u32;
    let state_ptr: *mut c_void = &mut next_buffer as *mut _ as *mut c_void;
    unsafe{
        let result = (***bqPlayerBufferQueue).Enqueue.unwrap()(*bqPlayerBufferQueue,state_ptr, next_size);
        assert_eq!(result,SL_RESULT_SUCCESS);
    }
    *curBuffer ^= 1;
}
extern "C" fn OpenSLWrap_Init(engineObject:&mut SLObjectItf,engineEngine:&mut SLEngineItf,outputMixObject:&mut SLObjectItf){
    let optionnull:*const SLEngineOption = ptr::null();
    let pinterfaceidnull:*const SLInterfaceID = ptr::null();
    let pInterfaceRequirednull:*const SLboolean = ptr::null();
    unsafe{
        let mut result = slCreateEngine(engineObject,0,optionnull,0, pinterfaceidnull,pInterfaceRequirednull);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (***engineObject).Realize.unwrap()(*engineObject,SL_BOOLEAN_FALSE);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let engine_ptr: *mut c_void = engineEngine as *mut _ as *mut c_void;
        let result = (***engineObject).GetInterface.unwrap()(*engineObject,SL_IID_ENGINE,engine_ptr);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (***engineEngine).CreateOutputMix.unwrap()(*engineEngine,outputMixObject,0,pinterfaceidnull,&0);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (***outputMixObject).Realize.unwrap()(*outputMixObject,SL_BOOLEAN_FALSE);
        assert_eq!(result,SL_RESULT_SUCCESS);
    }
    let loc_bufq:SLDataLocator_AndroidSimpleBufferQueue = SLDataLocator_AndroidSimpleBufferQueue{
        locatorType:SL_IID_ANDROIDSIMPLEBUFFERQUEUE,
        numBuffers:2
    };
    let format_pcm = SLDataFormat_PCM{
        formatType:SL_DATAFORMAT_PCM,
        numChannels:2,
        samplesPerSec:SL_SAMPLINGRATE_44_1,
        bitsPerSample:SL_PCMSAMPLEFORMAT_FIXED_16,
        containerSize:SL_PCMSAMPLEFORMAT_FIXED_16,
        channelMask:SL_SPEAKER_FRONT_LEFT,
        endianness:SL_BYTEORDER_LITTLEENDIAN
    };
    let audioSrc = SLDataSource_{
        pLocator:&loc_bufq,
        pFormat:&format_pcm
    };
}