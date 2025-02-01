mod recording;
mod transcription;
mod config;
mod execute;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "audio.wav";
    recording::start_recording(file_path);
    let audio_url = transcription::upload_audio(file_path)?;
    let transcription_id = transcription::request_transcription(audio_url)?;
    let transcription = transcription::get_transcription(transcription_id)?;
    
    // Remove punctuations and convert to lowercase
    let cleaned_transcription: String = transcription.chars()
        .filter(|c| !".,?!".contains(*c))
        .collect::<String>()
        .to_lowercase();
    
    execute::cmd_execute(&cleaned_transcription);
    Ok(())
}