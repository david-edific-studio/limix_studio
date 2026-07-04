mod canvas;

use canvas::{Canvas, Rgba};

fn main() {
    println!("--- Démarrage de Limix Studio ---");

    let mut workspace = Canvas::new(1920, 1080);
    
    workspace.add_layer("Arrière-plan");
    workspace.add_layer("Tracé Principal");

    // Simulation : On injecte un pixel rouge pur sur le calque supérieur (Index 0 = en haut à gauche)
    workspace.layers[1].pixels[0] = Rgba { r: 255, g: 0, b: 0, a: 255 };
    println!("Simulation : Coup de pinceau rouge appliqué sur 'Tracé Principal'.");

    // Appel du moteur de compositing
    println!("Lancement du calcul de fusion des calques...");
    let image_finale = workspace.render_flattened();

    // Vérification du résultat
    println!("Rendu terminé !");
    println!("Couleur du premier pixel à l'écran : {:?}", image_finale[0]);
}
