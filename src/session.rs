use windows::core::GUID;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::ISimpleAudioVolume;

use std::process::exit;
pub trait Session {
    unsafe fn getName(&self) -> String;
    unsafe fn getVolume(&self) -> f32;
    unsafe fn setVolume(&self, vol: f32);
}

pub struct EndPointSession {
    simple_audio_volume: IAudioEndpointVolume,
    name: String,
    guid: GUID,
}
impl EndPointSession {
    pub fn new(simple_audio_volume: IAudioEndpointVolume, name: String) -> Self {
        let guid = GUID::new().unwrap_or_else(|err| {
            eprintln!("ERROR: Couldn't generate GUID {err}");
            exit(1);
        });

        Self {
            simple_audio_volume: simple_audio_volume,
            name: name,
            guid: guid,
        }
    }
}

impl Session for EndPointSession {
    unsafe fn getName(&self) -> String {
        self.name.clone()
    }
    unsafe fn getVolume(&self) -> f32 {
        self.simple_audio_volume
            .GetMasterVolumeLevel()
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't get volume {err}");
                0.0
            })
    }
    unsafe fn setVolume(&self, vol: f32) {
        self.simple_audio_volume
            .SetMasterVolumeLevel(vol, &self.guid)
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't set volume: {err}");
            });
    }
}

pub struct ApplicationSession {
    simple_audio_volume: ISimpleAudioVolume,
    name: String,
    guid: GUID,
}

impl ApplicationSession {
    pub fn new(simple_audio_volume: ISimpleAudioVolume, name: String) -> Self {
        let guid = GUID::new().unwrap_or_else(|err| {
            eprintln!("ERROR: Couldn't generate GUID {err}");
            exit(1);
        });
        
        Self {
            simple_audio_volume: simple_audio_volume,
            name: name,
            guid: guid,
        }
    }
}

impl Session for ApplicationSession {
    unsafe fn getName(&self) -> String {
        self.name.clone()
    }
    unsafe fn getVolume(&self) -> f32 {
        self.simple_audio_volume
            .GetMasterVolume()
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't get volume {err}");
                0.0
            })
    }
    unsafe fn setVolume(&self, vol: f32) {
        self.simple_audio_volume
            .SetMasterVolume(vol, &self.guid)
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't set volume: {err}");
            });
    }
}
