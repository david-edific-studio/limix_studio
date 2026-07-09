#[derive(Clone, Copy, Debug)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// --- NOUVEAUTÉ : La nature du calque ---
#[derive(Clone, PartialEq, Debug)]
pub enum LayerType {
    Raster,   // Calque classique (Pinceau, Gomme, etc.)
    Folder,   // Dossier
    Dynamic,  // CFS Mode (Code For Scratch)
}

#[derive(Clone)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub blend_mode: usize,
    
    // Type et hiérarchie
    pub layer_type: LayerType,
    pub expanded: bool,
    pub depth: usize, 
    
    // --- NOUVEAUTÉ CFS : Le code source ---
    // Si c'est un calque Dynamic, il stocke ici son script JavaScript.
    pub script: Option<String>, 
    
    // Le Buffer visuel (le cache)
    pub pixels: Vec<Rgba>,
    pub overflow: Vec<(isize, isize, Rgba)>, 
}

impl Layer {
    // Création d'un calque classique (Raster)
    pub fn new(name: &str, width: usize, height: usize, depth: usize) -> Self {
        let total_pixels = width * height;
        Self {
            name: name.to_string(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: 0,
            layer_type: LayerType::Raster,
            expanded: false,
            depth,
            script: None,
            pixels: vec![Rgba { r: 0, g: 0, b: 0, a: 0 }; total_pixels],
            overflow: Vec::new(),
        }
    }

    // Création d'un dossier
    pub fn new_folder(name: &str, depth: usize) -> Self {
        Self {
            name: name.to_string(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: 0,
            layer_type: LayerType::Folder,
            expanded: true,
            depth,
            script: None,
            pixels: Vec::new(), 
            overflow: Vec::new(),
        }
    }

    // --- NOUVEAUTÉ : Création d'un calque Dynamique (CFS) ---
    pub fn new_dynamic(name: &str, width: usize, height: usize, depth: usize, code: &str) -> Self {
        let total_pixels = width * height;
        Self {
            name: name.to_string(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: 0,
            layer_type: LayerType::Dynamic,
            expanded: false,
            depth,
            script: Some(code.to_string()), // On stocke le JS ici !
            pixels: vec![Rgba { r: 0, g: 0, b: 0, a: 0 }; total_pixels], // Le buffer est vide, le moteur JS le remplira
            overflow: Vec::new(),
        }
    }
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Layer>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, layers: Vec::new() }
    }

    pub fn add_layer(&mut self, name: &str, depth: usize) {
        let layer = Layer::new(name, self.width, self.height, depth);
        self.layers.push(layer);
    }

    pub fn insert_layer(&mut self, index: usize, name: &str, depth: usize) {
        let layer = Layer::new(name, self.width, self.height, depth);
        self.layers.insert(index, layer);
    }

    pub fn insert_folder(&mut self, index: usize, name: &str, depth: usize) {
        let folder = Layer::new_folder(name, depth);
        self.layers.insert(index, folder);
    }

    // --- NOUVEAUTÉ : Insérer un calque codé ---
    pub fn insert_dynamic(&mut self, index: usize, name: &str, depth: usize, code: &str) {
        let layer = Layer::new_dynamic(name, self.width, self.height, depth, code);
        self.layers.insert(index, layer);
    }

    pub fn render_flattened(&self) -> Vec<Rgba> {
        let total_pixels = self.width * self.height;
        let mut output = Vec::with_capacity(total_pixels);

        // Damier de transparence
        let checker_size = 8; 
        for y in 0..self.height {
            for x in 0..self.width {
                let is_light = ((x / checker_size) + (y / checker_size)) % 2 == 0;
                let c = if is_light { 255 } else { 204 }; 
                output.push(Rgba { r: c, g: c, b: c, a: 255 }); 
            }
        }

        let mut actual_visible = vec![true; self.layers.len()];
        let mut current_hidden_depth = None;

        for i in (0..self.layers.len()).rev() {
            let layer = &self.layers[i];
            
            if let Some(hd) = current_hidden_depth {
                if layer.depth > hd {
                    actual_visible[i] = false; 
                    continue;
                } else {
                    current_hidden_depth = None; 
                }
            }

            if !layer.visible {
                actual_visible[i] = false;
                if layer.layer_type == LayerType::Folder {
                    current_hidden_depth = Some(layer.depth); 
                }
            }
        }

        for i in 0..self.layers.len() {
            let layer = &self.layers[i];
            
            if !actual_visible[i] || layer.layer_type == LayerType::Folder { continue; }

            for j in 0..total_pixels {
                let top = &layer.pixels[j];
                let bottom = &mut output[j];

                if top.a == 0 { continue; }

                let alpha = (top.a as f32 / 255.0) * layer.opacity;
                let inv_alpha = 1.0 - alpha;

                let tr = top.r as f32 / 255.0;
                let tg = top.g as f32 / 255.0;
                let tb = top.b as f32 / 255.0;
                let br = bottom.r as f32 / 255.0;
                let bg = bottom.g as f32 / 255.0;
                let bb = bottom.b as f32 / 255.0;

                let (blend_r, blend_g, blend_b);

                match layer.blend_mode {
                    1 => { blend_r = tr * br; blend_g = tg * bg; blend_b = tb * bb; },
                    2 => { blend_r = 1.0 - (1.0 - tr) * (1.0 - br); blend_g = 1.0 - (1.0 - tg) * (1.0 - bg); blend_b = 1.0 - (1.0 - tb) * (1.0 - bb); },
                    3 => {
                        blend_r = if br < 0.5 { 2.0 * tr * br } else { 1.0 - 2.0 * (1.0 - tr) * (1.0 - br) };
                        blend_g = if bg < 0.5 { 2.0 * tg * bg } else { 1.0 - 2.0 * (1.0 - tg) * (1.0 - bg) };
                        blend_b = if bb < 0.5 { 2.0 * tb * bb } else { 1.0 - 2.0 * (1.0 - tb) * (1.0 - bb) };
                    },
                    _ => { blend_r = tr; blend_g = tg; blend_b = tb; }
                }

                bottom.r = ((blend_r * 255.0 * alpha) + (bottom.r as f32 * inv_alpha)) as u8;
                bottom.g = ((blend_g * 255.0 * alpha) + (bottom.g as f32 * inv_alpha)) as u8;
                bottom.b = ((blend_b * 255.0 * alpha) + (bottom.b as f32 * inv_alpha)) as u8;
            }
        }
        output
    }
}
