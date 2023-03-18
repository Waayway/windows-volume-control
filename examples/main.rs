use testing_rust::AudioController;

fn main() {
    unsafe {
        let mut controller = AudioController::init();
        controller.GetSessions();
        controller.GetDefaultAudioEnpointVolumeControl();
        controller.GetAllProcessSessions();
        
    }
}
