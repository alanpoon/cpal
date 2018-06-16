use super::Device;
use opensles::bindings::{SLEngineItf,SLEngineItf_};
use opensles::bindings::{SLDataLocator_AndroidSimpleBufferQueue,SLDataFormat_PCM};
use opensles::bindings::{SL_DATALOCATOR_ANDROIDSIMPLEBUFFERQUEUE,SL_DATAFORMAT_PCM,SL_SAMPLINGRATE_8,
SL_PCMSAMPLEFORMAT_FIXED_16,SL_SPEAKER_FRONT_CENTER,SL_BYTEORDER_LITTLEENDIAN};
use opensles::bindings::{SLInterfaceID,SLboolean,SL_IID_BUFFERQUEUE, SL_IID_VOLUME, SL_IID_EFFECTSEND};
use opensles::bindings::SL_IID_ANDROIDSIMPLEBUFFERQUEUE;
use super::macrobinds::{SL_SAMPLINGRATE_8,SL_PCMSAMPLEFORMAT_FIXED_16,SL_SPEAKER_FRONT_CENTER,SL_BYTEORDER_LITTLEENDIAN
SL_BOOLEAN_FALSE};
pub struct Devices{
    object:SLObjectItf,
    deviceid: SLuint32,
    engineEngine:SLEngineItf
}

unsafe impl Send for Devices {
}
unsafe impl Sync for Devices {
}

impl Default for Devices {
    fn default() -> Devices {
        Devices{
            object:None,
            req: SLboolean
            engineEngine:SLEngineItf_{}
        }
    }
}
impl Iterator for Devices {
    type Item = Device;

    fn next(&mut self) -> Option<Device> {
        let loc_bufq = SLDataLocator_AndroidSimpleBufferQueue{
            locatorType:SL_DATALOCATOR_ANDROIDSIMPLEBUFFERQUEUE, 
            numBuffers:2};
        let format_pcm = SLDataFormat_PCM{
            formatType:SL_DATAFORMAT_PCM,
            numChannels:1,
            samplesPerSec:SL_SAMPLINGRATE_8,
            bitsPerSample:SL_PCMSAMPLEFORMAT_FIXED_16,
            containerSize:SL_PCMSAMPLEFORMAT_FIXED_16,
            channelMask:SL_SPEAKER_FRONT_CENTER,
            endianness:SL_BYTEORDER_LITTLEENDIAN};
        let audioSrc = SLDataSource{
            pLocator:&loc_bufq,
            pFormat: &format_pcm};
        let ids = [SL_IID_ANDROIDSIMPLEBUFFERQUEUE];    
        let bqPlayerObject: Option<SLObjectItf> = None;
        let mut result = self.object.CreateAudioPlayer(self.engineEngine, &bqPlayerObject, &audioSrc, &audioSnk,
                bqPlayerSampleRate? 2 : 3, ids, self.req);
        assert(SL_RESULT_SUCCESS,result);

        // realize the player
        let result = self.object.Realize(bqPlayerObject, SL_BOOLEAN_FALSE);
        assert(SL_RESULT_SUCCESS,result);

        // get the play interface
        let result = self.object.GetInterface(bqPlayerObject, SL_IID_PLAY, &bqPlayerPlay);
        assert(SL_RESULT_SUCCESS,result);

        // get the buffer queue interface
        let result = self.object.GetInterface(bqPlayerObject, SL_IID_BUFFERQUEUE,
                &bqPlayerBufferQueue);
        assert(SL_RESULT_SUCCESS,result);
       
    }
}
