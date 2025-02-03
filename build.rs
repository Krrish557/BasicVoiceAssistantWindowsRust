fn main() {
    println!("cargo:rustc-link-search=native=C:/vosk/vosk-api-0.3.50/vosk-api-0.3.50");
    println!("cargo:rustc-link-lib=dylib=vosk");
}
