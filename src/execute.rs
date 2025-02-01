use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

#[derive(Deserialize)]
struct Commands {
    commands: HashMap<String, String>,
}

pub fn cmd_execute(transcription: &str) {
    let data = fs::read_to_string("commands.json").expect("Unable to read file");
    let commands: Commands = serde_json::from_str(&data).expect("JSON was not well-formed");

    if let Some(ps_command) = commands.commands.get(transcription) {
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(ps_command)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            println!("Command executed successfully!");
        } else {
            eprintln!("Command execution failed.");
        }
    } else {
        println!("Unknown command: {}", transcription);
    }
}
