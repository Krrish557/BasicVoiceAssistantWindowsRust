use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
// use crate::config::{API_KEY, UPLOAD_URL, TRANSCRIBE_URL};

#[derive(Serialize)]
struct TranscriptionRequest {
    audio_url: String,
}

#[derive(Deserialize)]
struct TranscriptionResponse {
    id: String,
}

pub fn upload_audio(file_path: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let audio_data = fs::read(file_path)?;
    let response: Response = client
        .post(UPLOAD_URL)
        .header("Authorization", API_KEY)
        .header("Content-Type", "application/octet-stream")
        .body(audio_data)
        .send()?;
    let json_resp: serde_json::Value = response.json()?;
    Ok(json_resp["upload_url"].as_str().unwrap().to_string())
}

pub fn request_transcription(audio_url: String) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response: TranscriptionResponse = client
        .post(TRANSCRIBE_URL)
        .header("Authorization", API_KEY)
        .header("Content-Type", "application/json")
        .json(&TranscriptionRequest { audio_url })
        .send()?
        .json()?;
    Ok(response.id)
}

pub fn get_transcription(transcription_id: String) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/{}", TRANSCRIBE_URL, transcription_id);
    loop {
        let response: Response = client.get(&url).header("Authorization", API_KEY).send()?;
        let json_resp: serde_json::Value = response.json()?;
        match json_resp["status"].as_str() {
            Some("completed") => return Ok(json_resp["text"].as_str().unwrap().to_string()),
            Some("failed") => return Err("Transcription failed".into()),
            _ => {
                println!("Waiting for transcription...");
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        }
    }
}