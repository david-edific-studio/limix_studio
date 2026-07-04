// Structure d'un pixel standard (Rouge, Vert, Bleu, Opacité)
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

    // Moteur de rendu : fusionne tous les calques de bas en haut
    pub fn render_flattened(&self) -> Vec<Rgba> {
        let total_pixels = self.width * self.height;
        
        // On initialise une "toile de fond" blanche et opaque
        let mut output = vec![Rgba { r: 255, g: 255, b: 255, a: 255 }; total_pixels];

        for layer in &self.layers {
            if !layer.visible {
                continue; // On ignore les calques masqués pour économiser le CPU
            }
		
	    // Ajoute cette ligne pour utiliser la variable 'name' et informer l'utilisateur :
            println!("  -> Calcul des pixels pour le calque : {}", layer.name);

            for i in 0..total_pixels {
                let top = &layer.pixels[i];
                let bottom = &mut output[i];

                // Calcul de l'opacité réelle
                let alpha = (top.a as f32 / 255.0) * layer.opacity;
                let inv_alpha = 1.0 - alpha;

                // Application stricte de l'Alpha Blending (Interpolation linéaire)
                bottom.r = ((top.r as f32 * alpha) + (bottom.r as f32 * inv_alpha)) as u8;
                bottom.g = ((top.g as f32 * alpha) + (bottom.g as f32 * inv_alpha)) as u8;
                bottom.b = ((top.b as f32 * alpha) + (bottom.b as f32 * inv_alpha)) as u8;
            }
        }
        
        output
    }
}
