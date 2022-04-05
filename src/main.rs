use gst::prelude::*;

fn main() {
    gst::init().expect("gstreamer init error");

    let config_path = env!("CARGO_MANIFEST_DIR");
    let uri = String::from("file://") + config_path + "/test.mp3";
    let pipeline =
        gst::parse_launch(&format!("playbin uri={}", uri)).expect("gstreamer parse launch error");
    // Start playing
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");
    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}
