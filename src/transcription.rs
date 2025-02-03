use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use vosk::{Model, Recognizer};

#[derive(Serialize)]
#[allow(dead_code)]
struct TranscriptionRequest {
    audio_url: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct TranscriptionResponse {
    id: String,
}

pub fn transcribe_audio(file_path: &str) -> Result<String, Box<dyn Error>> {
    // Load the Vosk model
    let model_path = "models/vosk-model-small-en-us-0.15";
    let model = Model::new(model_path).expect("Could not create Vosk model");

    // Load the audio file
    let audio_file = File::open(file_path).expect("Could not open audio file");
    let mut reader = BufReader::new(audio_file);

    // Create recognizer
    let mut recognizer = Recognizer::new(&model, 16000.0).expect("Could not create recognizer");

    // Create a buffer to hold audio data
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).expect("Error reading audio file");

    // Process the audio file
    // Vosk requires raw audio data in the form of 16-bit mono PCM (i16)
    let audio_data: Vec<i16> = buffer
        .chunks(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    // Accept the waveform for transcription
    recognizer.accept_waveform(&audio_data).expect("Error processing audio");

    // Get the final transcription result
    let result = recognizer.final_result();

    if let vosk::CompleteResult::Single(result) = result {
        Ok(result.text.to_string())  // Convert &str to String
    } else {
        Err("Failed to get a valid result".into())
    }

}

pub fn get_transcription(file_path: &str) -> Result<String, Box<dyn Error>> {
    let transcription = transcribe_audio(file_path)?;
    Ok(transcription)
}
