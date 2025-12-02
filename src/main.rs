use std::i16;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound;
use std::sync::Mutex;

fn main() {
    // Set up audio host and input device
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");
    println!("Using input device: {}", device.name().unwrap());

    // Get default input config
    let config = device.default_input_config().unwrap();
    println!("Default input format: {:?}", config);

    // Create WAV writer
    let spec = hound::WavSpec {
        channels: config.channels() as u16,
        sample_rate: config.sample_rate().0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let writer = hound::WavWriter::create("recording.wav", spec).unwrap();

    // Wrap in Arc<Mutex<>> so we can share across threads
    let writer = std::sync::Arc::new(Mutex::new(Some(writer))); // <- store Option<WavWriter>

    // Build input stream
    let writer_clone = writer.clone();

    let stream = device
        .build_input_stream(
            &config.config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut guard = writer_clone.lock().unwrap();
                if let Some(ref mut writer) = *guard {
                    for &sample in data {
                        let amplitude = (sample * i16::MAX as f32) as i16;
                        writer.write_sample(amplitude).unwrap();
                    }
                }
            },
            move |err| {
                eprintln!("Stream error: {}", err);
            },
            None,
        )
        .unwrap();
    // Start recording
    stream.play().unwrap();
    println!("Recording for 5 seconds...");

    std::thread::sleep(std::time::Duration::from_secs(5));

    // Finalize safely
    let mut guard = writer.lock().unwrap();
    if let Some(writer) = guard.take() {
        writer.finalize().unwrap();
    }
    println!("Saved recording to recording.wav");
}
