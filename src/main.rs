use eframe::egui;
use rodio::{Decoder, OutputStream, Sink};  
use std::{fs::File, io::Read, path::PathBuf, thread};

struct Player {
    mp3_folder: PathBuf,
    songs: Vec<PathBuf>,
    current_song: Option<PathBuf>,
    current_song_index: usize,
    stream: Option<OutputStream>,
    sink: Option<Sink>,
    is_playing: bool
}

impl Default for Player {
    fn default() -> Self {
        Self {
            mp3_folder: PathBuf::from("./mp3"),
            songs: vec![], 
            current_song: None,
            current_song_index: 0,
            stream: None,
            sink: None,
            is_playing: false,
        }
    }
}

impl Player {
    fn scan_mp3s(&mut self) {
        if let Ok(songs) = std::fs::read_dir(&self.mp3_folder) {
            self.songs = songs
                .map(|f| f.unwrap().path())
                .collect();
        }
    }

    fn setup_audio(&mut self) {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        self.stream = Some(stream);
        self.sink = Some(Sink::try_new(&stream_handle).unwrap());
    }

    fn read_file(&self, file_path: &PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
        
    fn play_song(&mut self, song: &PathBuf) {
        let buffer = self.read_file(song).unwrap();
        let source = Decoder::new(std::io::Cursor::new(buffer)).unwrap();
        self.sink.as_mut().unwrap().clear();
        self.sink.as_mut().unwrap().append(source);
        self.sink.as_mut().unwrap().play();
        self.is_playing = true;
    }

    fn pause_resume(&mut self) {
        if let Some(sink) = self.sink.as_mut() {
            if self.is_playing {
                sink.pause();
            } else {
                sink.play();
            }
            self.is_playing = !self.is_playing;
        }
    }
}

impl eframe::App for Player {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.stream.is_none() {
            self.setup_audio();
        }

        self.scan_mp3s();

        if let Some(selected_song) = self.songs.get(self.current_song_index).cloned() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Song List");

                    for (index, song) in self.songs.iter().enumerate() {
                        ui.selectable_value(
                            &mut self.current_song_index,
                            index,
                            song.file_name()
                                .map(|name| name.to_string_lossy().to_string())
                                .unwrap_or_else(|| String::from("Unknown")),
                        );
                    }
                });

                ui.label("Now Playing:");
                ui.label(selected_song.file_name().unwrap().to_string_lossy());
                if ui.button("Play").clicked() {
                    self.play_song(&selected_song);
                }

                if let Some(sink) = self.sink.as_ref() {
                    let label = if self.is_playing { "Pause" } else { "Resume" };
                    if ui.button(label).clicked() {
                        self.pause_resume();
                    }
                }
            });
        }
    }
}



fn main() {
    let native_options = eframe::NativeOptions::default();

    if let Err(err) = eframe::run_native("Music Player", native_options, Box::new(|cc| Box::new(Player::default()))) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}