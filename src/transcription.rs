use vosk::{Model, Recognizer};
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use byteorder::{LittleEndian, ReadBytesExt};
use serde_json::Value;

pub fn get_transcription(file_path: &str) -> Result<String, Box<dyn Error>> {
    let model_path = "./models/vosk-model-small-en-us-0.15";
    let model = Model::new(model_path).expect("Failed to load Vosk model");
    let mut recognizer = Recognizer::new(&model, 16000.0).unwrap();

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    
    // Read 16-bit audio samples
    let mut buffer = Vec::new();
    while let Ok(sample) = reader.read_i16::<LittleEndian>() {
        buffer.push(sample);
    }

    recognizer.accept_waveform(&buffer)?;

    // Get the final result
    let result = recognizer.final_result();

    // Convert result to a JSON string
    let result_str = serde_json::to_string(&result).expect("Failed to serialize result");

    // Parse JSON correctly
    let json: Value = serde_json::from_str(&result_str)?;
    let transcription = json["text"].as_str().unwrap_or("").to_string();

    Ok(transcription)
}

