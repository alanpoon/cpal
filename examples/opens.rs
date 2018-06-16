extern crate opensles;
use opensles::bindings::*;
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
fn main() {
    let engineObject:SLObjectItf;
    let engineEngine:SLEngineItf;    
    let outputMixObject:SLObjectItf;

    // buffer queue player interfaces
    let bqPlayerObject:SLObjectItf;
    let bqPlayerPlay:SLPlayItf;
    let bqPlayerBufferQueue:SLAndroidSimpleBufferQueueItf;
    let bqPlayerMuteSolo:SLMuteSoloItf;
    let bqPlayerVolume:SLVolumeItf;
    let bqPlayerBufSize =0;
    let buffer:[[u16;2];512];
    let curBuffer =0;

}
fn bqPlayerCallback(bq:SLAndroidSimpleBufferQueueItf, context as *mut _ as *mut std::os::raw::c_void) {
    assert(bqPlayerBufferQueue,bq);

}