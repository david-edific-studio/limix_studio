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
    
    // --- NOUVEAUTÉS POUR L'ARBORESCENCE (DOSSIERS) ---
    pub is_folder: bool,
    pub expanded: bool,
    pub depth: usize, 
    // -------------------------------------------------
    
    pub pixels: Vec<Rgba>,
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

    pub fn add_folder(&mut self, name: &str, depth: usize) {
        let folder = Layer::new_folder(name, depth);
        self.layers.push(folder);
    }

    pub fn render_flattened(&self) -> Vec<Rgba> {
        let total_pixels = self.width * self.height;
        let mut output = vec![Rgba { r: 255, g: 255, b: 255, a: 255 }; total_pixels];

        for layer in &self.layers {
            if !layer.visible || layer.is_folder {
                continue; 
            }

            for i in 0..total_pixels {
                let top = &layer.pixels[i];
                let bottom = &mut output[i];

                if top.a == 0 { continue; }

                let alpha = (top.a as f32 / 255.0) * layer.opacity;
                let inv_alpha = 1.0 - alpha;

                let tr = top.r as f32 / 255.0;
                let tg = top.g as f32 / 255.0;
                let tb = top.b as f32 / 255.0;
                let br = bottom.r as f32 / 255.0;
                let bg = bottom.g as f32 / 255.0;
                let bb = bottom.b as f32 / 255.0;

                let (blend_r, blend_g, blend_b); // CORRECTION : Plus de 'mut' inutiles

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
