# Rust Language Programme

This programme was made to learn about the possibilities using the Rust language.

## Steps to Try This Program

1. **Create a config.rs File:**
    - Navigate to the `src` folder.
    - Create a file named `config.rs`.
    - Paste the following lines into `config.rs`:
    ```rust
    pub const API_KEY: &str = "<Your API Key>";
    pub const UPLOAD_URL: &str = "https://api.assemblyai.com/v2/upload";
    pub const TRANSCRIBE_URL: &str = "https://api.assemblyai.com/v2/transcript";
    ```

2. **Build and Run the Program:**
    - Open your terminal.
    - Run the following commands:
    ```sh
    cargo build
    cargo run
    ```
    - You will be prompted to press space. Do it.
    - State your command (listed in `commands.json`).
    - The stated command will be executed.

Enjoy experimenting with Rust!
