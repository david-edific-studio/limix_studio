use crate::core::canvas::{Canvas, Rgba};

pub fn apply(canvas: &mut Canvas, active_layer: usize, cx: usize, cy: usize, radius: isize) -> bool {
    let mut modified = false;
    let width = canvas.width as isize;
    let height = canvas.height as isize;

    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let px = cx as isize + dx;
            let py = cy as isize + dy;

            if px >= 0 && px < width && py >= 0 && py < height {
                let index = (py as usize) * (width as usize) + (px as usize);
                
                // GOMME : Injection d'un pixel transparent
                canvas.layers[active_layer].pixels[index] = Rgba { r: 0, g: 0, b: 0, a: 0 };
                modified = true;
            }
        }
    }
    modified
}