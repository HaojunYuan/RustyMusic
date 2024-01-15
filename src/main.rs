use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use rodio::{Decoder, OutputStream, Sink};

fn read_file(file_path: &PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <mp3_file>", args[0]);
        std::process::exit(1);
    }

    let mp3_file = PathBuf::from(&args[1]);

    if !mp3_file.exists() {
        eprintln!("File not found: {:?}", mp3_file);
        std::process::exit(1);
    }

    play_mp3(&mp3_file)?;

    // Sleep to keep the program alive while the audio plays
    thread::sleep(Duration::from_secs(10));

    Ok(())
}
