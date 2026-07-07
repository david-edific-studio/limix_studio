use crate::core::canvas::{Canvas, Rgba};
use eframe::egui;
use std::collections::HashMap;

/// Scanne le calque (y compris ce qui déborde de la feuille) pour trouver la "Boîte de Sélection" parfaite
pub fn get_bounding_box(canvas: &Canvas, layer_idx: usize) -> Option<egui::Rect> {
    let layer = &canvas.layers[layer_idx];
    let w = canvas.width as isize;
    let h = canvas.height as isize;

    let mut min_x = w;
    let mut min_y = h;
    let mut max_x = -9999;
    let mut max_y = -9999;
    let mut found = false;

    // Scan des pixels sur la feuille
    for y in 0..h {
        for x in 0..w {
            if layer.pixels[(y * w + x) as usize].a > 0 {
                if x < min_x { min_x = x; }
                if x > max_x { max_x = x; }
                if y < min_y { min_y = y; }
                if y > max_y { max_y = y; }
                found = true;
            }
        }
    }

    // Scan des pixels qui ont débordé de la feuille !
    for &(x, y, pixel) in &layer.overflow {
        if pixel.a > 0 {
            if x < min_x { min_x = x; }
            if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
            found = true;
        }
    }

    if found {
        Some(egui::Rect::from_min_max(
            egui::pos2(min_x as f32, min_y as f32),
            egui::pos2((max_x + 1) as f32, (max_y + 1) as f32) 
        ))
    } else {
        None
    }
}

/// Transformation "Absolue" (Déplacement & Redimensionnement ANTI-DESTRUCTION)
pub fn apply_transform_absolute(
    canvas: &mut Canvas,
    layer_idx: usize,
    original_pixels: &[Rgba],
    original_overflow: &[(isize, isize, Rgba)],
    orig_rect: egui::Rect,
    new_rect: egui::Rect,
) -> bool {
    let width = canvas.width as isize;
    let height = canvas.height as isize;

    // 1. On efface la feuille et la mémoire de débordement
    for p in canvas.layers[layer_idx].pixels.iter_mut() {
        p.r = 0; p.g = 0; p.b = 0; p.a = 0;
    }
    canvas.layers[layer_idx].overflow.clear();

    let w_new = new_rect.width();
    let h_new = new_rect.height();
    if w_new < 0.5 || h_new < 0.5 { return false; }

    let scale_x = orig_rect.width() / w_new;
    let scale_y = orig_rect.height() / h_new;

    // 2. On indexe les pixels cachés pour les retrouver vite
    let mut overflow_map = HashMap::new();
    for &(x, y, p) in original_overflow {
        overflow_map.insert((x, y), p);
    }

    // 3. On calcule TOUT le nouveau rectangle, même ce qui sort de la feuille
    let start_x = new_rect.min.x.floor() as isize;
    let end_x = new_rect.max.x.ceil() as isize;
    let start_y = new_rect.min.y.floor() as isize;
    let end_y = new_rect.max.y.ceil() as isize;

    for dest_y in start_y..=end_y {
        for dest_x in start_x..=end_x {
            let src_x = orig_rect.min.x + (dest_x as f32 - new_rect.min.x) * scale_x;
            let src_y = orig_rect.min.y + (dest_y as f32 - new_rect.min.y) * scale_y;

            let sx = src_x as isize;
            let sy = src_y as isize;

            let mut pixel = Rgba { r: 0, g: 0, b: 0, a: 0 };

            // On vérifie d'abord si le pixel venait de la feuille
            if sx >= 0 && sx < width && sy >= 0 && sy < height {
                pixel = original_pixels[(sy * width + sx) as usize];
            } 
            // Sinon on va le chercher dans la dimension cachée !
            else if let Some(&p) = overflow_map.get(&(sx, sy)) {
                pixel = p;
            }

            if pixel.a > 0 {
                // Si la NOUVELLE position est sur la feuille, on dessine
                if dest_x >= 0 && dest_x < width && dest_y >= 0 && dest_y < height {
                    canvas.layers[layer_idx].pixels[(dest_y * width + dest_x) as usize] = pixel;
                } 
                // Sinon on le sauvegarde en sécurité pour plus tard !
                else {
                    canvas.layers[layer_idx].overflow.push((dest_x, dest_y, pixel));
                }
            }
        }
    }

    true
}
