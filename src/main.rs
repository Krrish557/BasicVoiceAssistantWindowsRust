mod recording;
mod transcription;
mod execute;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./audio/audio.wav";

    // Start Recording
    recording::start_recording(file_path);

    // Get Offline Transcription
    let transcription = transcription::get_transcription(file_path)?;

    // Clean Text (remove punctuation, lowercase)
    let cleaned_transcription: String = transcription.chars()
        .filter(|c| !".,?!".contains(*c))
        .collect::<String>()
        .to_lowercase();

    // Execute the Command
    execute::cmd_execute(&cleaned_transcription);

    Ok(())
}
