// Structure d'un pixel standard (Rouge, Vert, Bleu, Opacité)
// On utilise derive(Clone, Copy) pour manipuler la mémoire très rapidement.
#[derive(Clone, Copy, Debug)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// L'entité "Calque" qui contiendra la grille de pixels
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub pixels: Vec<Rgba>,
}

impl Layer {
    // Initialisation systémique d'un nouveau calque vide
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        let total_pixels = width * height;
        Self {
            name: name.to_string(),
            visible: true,
            opacity: 1.0,
            // Remplissage avec des pixels totalement transparents par défaut
            pixels: vec![Rgba { r: 0, g: 0, b: 0, a: 0 }; total_pixels],
        }
    }
}

// Le chef d'orchestre : l'espace de travail global
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Layer>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            layers: Vec::new(),
        }
    }

    // Ajoute un calque au-dessus de la pile
    pub fn add_layer(&mut self, name: &str) {
        let layer = Layer::new(name, self.width, self.height);
        self.layers.push(layer);
        println!("[Moteur] Calque '{}' généré en mémoire.", name);
    }
}