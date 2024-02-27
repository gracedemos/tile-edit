mod app;

use eframe::egui;
use app::App;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tile Edit",
        options,
        Box::new(|cc| {
            Box::<App>::default()
        })
    )
}
