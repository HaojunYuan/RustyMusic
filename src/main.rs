use eframe::NativeOptions;

mod player;
use player::Player;

mod ui;

fn main() {
    let native_options = NativeOptions::default();

    if let Err(err) = eframe::run_native(
        "Music Player",
        native_options,
        Box::new(|_cc| Box::new(Player::default())),
    ) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
