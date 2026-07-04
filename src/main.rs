// On déclare notre nouveau module pour que Rust l'intègre à la compilation
mod canvas;

use canvas::Canvas;

fn main() {
    println!("--- Démarrage de Limix Studio ---");

    // 1. Définition de l'espace de travail (ex: un canvas Full HD)
    let mut workspace = Canvas::new(1920, 1080);
    println!("Document initialisé : {}x{} pixels", workspace.width, workspace.height);

    // 2. Création de l'architecture de calques de base
    workspace.add_layer("Arrière-plan");
    workspace.add_layer("Tracé Principal");

    // 3. Vérification de l'état du système
    println!("Moteur prêt. Nombre de calques actifs : {}", workspace.layers.len());
}