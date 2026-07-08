use crate::core::canvas::Canvas;

/// Recadre le canevas à la zone sélectionnée (ou à un rectangle donné).
pub fn apply(canvas: &mut Canvas, x: usize, y: usize, new_w: usize, new_h: usize) -> bool {
    if new_w == 0 || new_h == 0 { return false; }

    let old_w = canvas.width;
    let old_h = canvas.height;

    for layer in canvas.layers.iter_mut() {
        if layer.is_folder { continue; }
        let old_pixels = layer.pixels.clone();
        let mut new_pixels = vec![crate::core::canvas::Rgba { r: 0, g: 0, b: 0, a: 0 }; new_w * new_h];
        for ny in 0..new_h {
            for nx in 0..new_w {
                let ox = x + nx;
                let oy = y + ny;
                if ox < old_w && oy < old_h {
                    new_pixels[ny * new_w + nx] = old_pixels[oy * old_w + ox];
                }
            }
        }
        layer.pixels = new_pixels;
        // Réinitialise l'overflow (pixels hors-canevas) après recadrage
        layer.overflow.retain(|(ox, oy, _)| {
            let nx = *ox - x as isize;
            let ny = *oy - y as isize;
            nx >= 0 && nx < new_w as isize && ny >= 0 && ny < new_h as isize
        });
        for (ox, oy, _) in layer.overflow.iter_mut() {
            *ox -= x as isize;
            *oy -= y as isize;
        }
    }

    canvas.width = new_w;
    canvas.height = new_h;
    canvas.selection_mask = None; // La sélection est annulée après le recadrage
    let _ = old_h;
    true
}
