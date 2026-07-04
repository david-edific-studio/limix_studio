mod core;
mod ui;
mod app;
mod tools;
mod filters;

use eframe::NativeOptions;
use ui::LimixApp;

fn main() -> eframe::Result<()> {
    println!("--- Lancement de l'interface Limix Studio ---");
    
    // Configuration de la fenêtre système (dimensions, icône, etc.)
    let mut options = NativeOptions::default();
    options.viewport.inner_size = Some(eframe::egui::vec2(1280.0, 720.0));
    options.viewport.title = Some("Limix Studio".to_owned());

    // Démarrage de la boucle graphique
    eframe::run_native(
        "Limix Studio",
        options,
        Box::new(|cc| Box::new(LimixApp::new(cc))),
    )
}
