use eframe::egui;

use crate::player::Player;

impl eframe::App for Player {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

                if let Some(_sink) = self.sink.as_ref() {
                    let label = if self.is_playing { "Pause" } else { "Resume" };
                    if ui.button(label).clicked() {
                        self.pause_resume();
                    }
                }

                ui.horizontal(|ui| {
                    if ui.button("Previous").clicked() {
                        self.play_previous_song();
                    }
                    if ui.button("Next").clicked() {
                        self.play_next_song();
                    }
                });
            });
        }
    }
}
