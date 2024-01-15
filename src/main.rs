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

fn play_mp3(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let buffer = read_file(file_path)?;

    // Create an output stream and a sink
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    // Decode the MP3 and append it to the sink
    let source = Decoder::new(std::io::Cursor::new(buffer))?;
    sink.append(source);

    // The sound plays in a separate thread
    sink.sleep_until_end();

    Ok(())
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
