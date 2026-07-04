mod canvas;

use canvas::{Canvas, Rgba};
// On importe le module externe
use image::{ImageBuffer, RgbaImage};

fn main() {
    println!("--- Démarrage de Limix Studio ---");

    // 1. On crée une petite toile de 800x600 pour le test
    let width = 800;
    let height = 600;
    let mut workspace = Canvas::new(width, height);
    
    workspace.add_layer("Arrière-plan");
    workspace.add_layer("Tracé Principal");

    // 2. Simulation : On dessine un carré rouge de 100x100 pixels en haut à gauche
    println!("Simulation : Dessin d'un carré rouge sur 'Tracé Principal'...");
    for y in 50..150 {
        for x in 50..150 {
            let index = (y * width + x) as usize;
            workspace.layers[1].pixels[index] = Rgba { r: 255, g: 50, b: 50, a: 255 };
        }
    }

    // 3. Appel du moteur de compositing
    println!("Lancement du calcul de fusion...");
    let image_finale = workspace.render_flattened();

    // 4. Conversion et Exportation en fichier .png
    println!("Exportation vers export_limix.png...");
    
    let mut img_buffer: RgbaImage = ImageBuffer::new(width as u32, height as u32);
    
    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let index = (y as usize * width) + x as usize;
        let p = &image_finale[index];
        *pixel = image::Rgba([p.r, p.g, p.b, p.a]);
    }

    // Sauvegarde physique sur le disque
    img_buffer.save("export_limix.png").expect("Échec de l'écriture du fichier PNG");

    println!("Succès ! Ouvre ton dossier projet, le fichier image t'attend.");
}
