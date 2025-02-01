This programme was made to learn about the possiblities using the rust language.
To try this program make a config.rs file in the src folder and paste these lines:
  pub const API_KEY: &str = "<Your API Key>";
  pub const UPLOAD_URL: &str = "https://api.assemblyai.com/v2/upload";
  pub const TRANSCRIBE_URL: &str = "https://api.assemblyai.com/v2/transcript";
Then run the command in your terminal 
  cargo build
  cargo run
you will be prompted to press space, do it state your command (listed in commands.json)
the stated command will be executed.
