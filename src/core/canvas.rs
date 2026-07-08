#[derive(Clone, Copy, Debug)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub blend_mode: usize,
    
    pub is_folder: bool,
    pub expanded: bool,
    pub depth: usize, 
    
    pub pixels: Vec<Rgba>,
    // --- NOUVEAUTÉ : Mémoire dimensionnelle pour les pixels hors de la feuille ! ---
    pub overflow: Vec<(isize, isize, Rgba)>, 
}

impl Layer {
    pub fn new(name: &str, width: usize, height: usize, depth: usize) -> Self {
        let total_pixels = width * height;
        Self {
            name: name.to_string(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: 0,
            is_folder: false,
            expanded: false,
            depth,
            pixels: vec![Rgba { r: 0, g: 0, b: 0, a: 0 }; total_pixels],
            overflow: Vec::new(), // Initialisation du vide
        }
    }

    pub fn new_folder(name: &str, depth: usize) -> Self {
        Self {
            name: name.to_string(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: 0,
            is_folder: true,
            expanded: true,
            depth,
            pixels: Vec::new(), 
            overflow: Vec::new(),
        }
    }
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Layer>,
    pub selection_mask: Option<Vec<u8>>, // 0 = non sélectionné, 255 = sélectionné
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, layers: Vec::new(), selection_mask: None }
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

    pub fn render_flattened(&self) -> Vec<Rgba> {
        let total_pixels = self.width * self.height;
        let mut output = Vec::with_capacity(total_pixels);

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
                if layer.is_folder {
                    current_hidden_depth = Some(layer.depth); 
                }
            }
        }

        for i in 0..self.layers.len() {
            let layer = &self.layers[i];
            
            if !actual_visible[i] || layer.is_folder { continue; }

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
