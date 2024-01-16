use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::Read, path::PathBuf};

pub struct Player {
    pub mp3_folder: PathBuf,
    pub songs: Vec<PathBuf>,
    pub current_song_index: usize,
    pub stream: Option<OutputStream>,
    pub sink: Option<Sink>,
    pub is_playing: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            mp3_folder: PathBuf::from("./mp3"),
            songs: vec![],
            current_song_index: 0,
            stream: None,
            sink: None,
            is_playing: false,
        }
    }
}

impl Player {
    pub fn scan_mp3s(&mut self) {
        if let Ok(songs) = std::fs::read_dir(&self.mp3_folder) {
            self.songs = songs.map(|f| f.unwrap().path()).collect();
        }
    }

    pub fn setup_audio(&mut self) {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        self.stream = Some(stream);
        self.sink = Some(Sink::try_new(&stream_handle).unwrap());
    }

    pub fn read_file(&self, file_path: &PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn play_song(&mut self, song: &PathBuf) {
        let buffer = self.read_file(song).unwrap();
        let source = Decoder::new(std::io::Cursor::new(buffer)).unwrap();
        self.sink.as_mut().unwrap().clear();
        self.sink.as_mut().unwrap().append(source);
        self.sink.as_mut().unwrap().play();
        self.is_playing = true;
    }

    pub fn pause_resume(&mut self) {
        if let Some(sink) = self.sink.as_mut() {
            if self.is_playing {
                sink.pause();
            } else {
                sink.play();
            }
            self.is_playing = !self.is_playing;
        }
    }

    pub fn play_next_song(&mut self) {
        if !self.songs.is_empty() {
            self.current_song_index = (self.current_song_index + 1) % self.songs.len();
            let next_song = self.songs[self.current_song_index].clone();
            self.play_song(&next_song);
        }
    }

    pub fn play_previous_song(&mut self) {
        if !self.songs.is_empty() {
            if self.current_song_index == 0 {
                self.current_song_index = self.songs.len() - 1;
            } else {
                self.current_song_index -= 1;
            }

            let previous_song = self.songs[self.current_song_index].clone();
            self.play_song(&previous_song);
        }
    }
}
