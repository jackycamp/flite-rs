use fliters;
use rodio::{OutputStream, Sink};

fn main() {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    let fwav = fliters::tts("play this audio with some stream controls");
    let decoder = fwav.get_decoder();
    sink.append(decoder);
    sink.sleep_until_end();
}
