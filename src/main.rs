mod core;
mod ui;
mod app;
mod tools;
mod filters;

use eframe::NativeOptions;
use ui::LimixApp;

fn main() -> eframe::Result<()> {
    println!("--- Lancement de l'interface Limix Studio ---");
    
    let mut options = NativeOptions::default();
    
    // C'est ici qu'on verrouille l'architecture de la fenêtre !
    options.viewport = eframe::egui::ViewportBuilder::default()
        .with_title("Limix Studio")
        .with_inner_size([1280.0, 720.0])      // Ta taille par défaut
        .with_min_inner_size([1000.0, 600.0]); // LE BOUCLIER ANTI-ÉCRASEMENT (Triangles rouges)

    // Démarrage de la boucle graphique
    eframe::run_native(
        "Limix Studio",
        options,
        Box::new(|cc| Box::new(LimixApp::new(cc))),
    )
}
