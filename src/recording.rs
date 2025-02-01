use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use std::sync::{Arc, Mutex};
use std::thread;
use device_query::{DeviceQuery, DeviceState, Keycode};

pub fn start_recording(filename: &str) {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device found");
    let config = device.default_input_config().unwrap();
    let device_state = DeviceState::new();

    let spec = WavSpec {
        channels: config.channels() as u16,
        sample_rate: config.sample_rate().0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let writer = Arc::new(Mutex::new(Some(WavWriter::create(filename, spec).unwrap())));
    let writer_clone = writer.clone();

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                if let Some(ref mut writer) = *writer_clone.lock().unwrap() {
                    for &sample in data {
                        writer.write_sample(sample).unwrap();
                    }
                }
            },
            |err| eprintln!("Error: {}", err),
            None,
        )
        .unwrap();

    let mut recording = false;

    println!("Press Space to start recording...");

    loop {
        let keys = device_state.get_keys();
        
        if keys.contains(&Keycode::Space) {
            if !recording {
                println!("Recording started. Press Space again to stop...");
                stream.play().unwrap();
                recording = true;
            } else {
                println!("Recording stopped.");
                break;
            }
        }
        
        thread::sleep(std::time::Duration::from_millis(100));
    }

    drop(stream); // Stop the recording properly

    let writer_to_finalize = {
        let mut lock = writer.lock().unwrap();
        lock.take()
    };

    if let Some(writer) = writer_to_finalize {
        writer.finalize().unwrap();
    }
}
