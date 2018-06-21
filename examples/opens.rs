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
pub const SL_DATALOCATOR_OUTPUTMIX:SLuint32=0x00000004;
pub const SL_BYTEORDER_LITTLEENDIAN:SLuint32 = 0x00000002;
pub const SL_PLAYSTATE_PLAYING:SLuint32 = 0x00000003;
fn main() {
    let engineObject:SLObjectItf;
    let mut engineEngine:SLEngineItf;    
    let outputMixObject:SLObjectItf;

    // buffer queue player interfaces
    let bqPlayerObject:SLObjectItf;
    let mut bqPlayerPlay:SLPlayItf;
    let mut bqPlayerBufferQueue:SLAndroidSimpleBufferQueueItf;
    let mut bqPlayerMuteSolo:SLMuteSoloItf;
    let mut bqPlayerVolume:SLVolumeItf;
    let bqPlayerBufSize =0;
    let mut curBuffer:usize =0;
    let mut context:Context;
    context.curBuffer =0;
}
pub struct Context{
    buffer:[[u16;2];512],
    bqPlayerBufferQueue:SLAndroidSimpleBufferQueueItf,
    curBuffer:usize
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
}

fn OpenSLWrap_Init(engineObject:&mut SLObjectItf,engineEngine:&mut SLEngineItf,outputMixObject:&mut SLObjectItf,bqPlayerObject:&mut SLObjectItf,
bqPlayerPlay:&mut SLPlayItf,
bqPlayerVolume:&mut SLVolumeItf,context:&mut Context)->bool{
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
        outputMix:*outputMixObject
    };
    let loc_outmix_ptr:*mut c_void = &mut loc_outmix as *mut _ as *mut c_void;
    let pformatnull:*mut c_void = ptr::null_mut();
    let audioSnk = SLDataSink{
        pLocator:loc_outmix_ptr,
        pFormat:pformatnull
    };
    let bqPlayerPlay_ptr:*mut c_void = bqPlayerPlay as *mut _ as *mut c_void;
    let bqPlayerBufferQueue_ptr:*mut c_void = context.bqPlayerBufferQueue as *mut c_void;
    let register_null:*mut c_void = ptr::null_mut();
    let slvolume_ptr:*mut c_void = bqPlayerVolume as *mut _ as *mut c_void;
    unsafe{
        let ids:[SLInterfaceID;2]=[SL_IID_BUFFERQUEUE,SL_IID_VOLUME];
        let req:[SLboolean;2]=[SL_BOOLEAN_TRUE,SL_BOOLEAN_TRUE];
        let result = (***bqPlayerObject).Realize.unwrap()(*bqPlayerObject,SL_BOOLEAN_FALSE);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (***bqPlayerObject).GetInterface.unwrap()(*bqPlayerObject,SL_IID_PLAY,bqPlayerPlay_ptr);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (***bqPlayerObject).GetInterface.unwrap()(*bqPlayerObject,SL_IID_BUFFERQUEUE,bqPlayerBufferQueue_ptr);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (**context.bqPlayerBufferQueue).RegisterCallback.unwrap()(context.bqPlayerBufferQueue,Some(bqPlayerCallback2),register_null);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (***bqPlayerObject).GetInterface.unwrap()(*bqPlayerObject, SL_IID_VOLUME, slvolume_ptr);
        assert_eq!(result,SL_RESULT_SUCCESS);
        let result = (***bqPlayerPlay).SetPlayState.unwrap()(*bqPlayerPlay,SL_PLAYSTATE_PLAYING);
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
fn OpenSLWrap_Shutdown(bqPlayerObject:&mut SLObjectItf,bqPlayerPlay:&mut SLPlayItf,
bqPlayerMuteSolo:&mut SLMuteSoloItf,bqPlayerVolume:&mut SLVolumeItf,
engineEngine:&mut SLObjectItf,
engineObject:&mut SLObjectItf,
outputMixObject:&mut SLObjectItf,
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