#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;
use bindings::*;

use std::ffi::{CString, CStr};
use hound::{WavWriter, WavSpec, SampleFormat};
use rodio::{Decoder, OutputStream, source::Source, Sink};
use std::io::{Cursor, BufWriter};

pub struct FliteWav {
    samples: Vec<i16>,
    sample_rate: i32,
    num_channels: i32, 
}

impl FliteWav {
    #![allow(clippy::missing_safety_doc)]
    // TODO: add safety doc
    pub unsafe fn new(raw_wav: *mut cst_wave) -> Self {
       Self {
            samples: std::slice::from_raw_parts((*raw_wav).samples, (*raw_wav).num_samples as usize).to_vec(),
            sample_rate: (*raw_wav).sample_rate,
            num_channels: (*raw_wav).num_channels
        } 
    }
    pub fn spec(&self) -> WavSpec {
        WavSpec {
            channels: self.num_channels as u16,
            sample_rate: self.sample_rate as u32,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int
        }
    }
    pub fn play(&self) -> Sink {
        // TODO: play returns a sink, so that user could
        // pause/play/stop audio output stream.
        todo!()
    }
    pub fn play_without_control(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut wav_data: Vec<u8> = Vec::new();
        let mut wav_writer = WavWriter::new(BufWriter::new(Cursor::new(&mut wav_data)), self.spec()).expect("failed to create wav writer");
        self.samples.iter().copied().for_each(|sample| wav_writer.write_sample(sample).expect("failed to write sample"));
        wav_writer.finalize().expect("Failed to finalized wav");
        let decoder = Decoder::new(Cursor::new(wav_data)).unwrap();
        let duration = std::time::Duration::from_secs_f32(self.samples.len() as f32 / self.sample_rate as f32);
        stream_handle.play_raw(decoder.convert_samples()).expect("failed to play");
        std::thread::sleep(duration);

    }
}

pub fn tts(text: &str) -> FliteWav {
    unsafe {
        let c_text = CString::new(text).expect("failed to construct ctext");
        let voice = register_cmu_us_slt(std::ptr::null());
        let raw_wav = flite_text_to_wave(c_text.as_ptr(), voice);
        let wav = FliteWav::new(raw_wav);
        cst_free(raw_wav as *mut _);
        wav
    }
}

