use crate::core::canvas::{Canvas, Rgba};

pub fn apply(
    canvas: &mut Canvas,
    layer_idx: usize,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    size: f32,
    hardness: f32,
    opacity: f32,
    flow: f32,
    color: [f32; 3], // <--- NOUVEAUTÉ : La couleur en paramètre (RGB)
) -> bool {
    let radius = size / 2.0;
    let r_sq = radius * radius;
    let hard_radius = radius * (hardness / 100.0);
    let hard_r_sq = hard_radius * hard_radius;
    
    let opac_mult = opacity / 100.0;
    let flow_mult = flow / 100.0;

    let dx = x1 - x0;
    let dy = y1 - y0;
    let dist = (dx * dx + dy * dy).sqrt();
    
    let spacing = (radius * 0.1).clamp(1.0, 3.0); 
    let steps = (dist / spacing).ceil() as usize;

    let mut modified = false;

    // --- On convertit la couleur (0.0 - 1.0) en format classique (0.0 - 255.0) ---
    let brush_r = color[0] * 255.0;
    let brush_g = color[1] * 255.0;
    let brush_b = color[2] * 255.0;

    for i in 0..=steps {
        let t = if steps == 0 { 1.0 } else { i as f32 / steps as f32 };
        let cx = x0 + dx * t;
        let cy = y0 + dy * t;

        let min_x = (cx - radius).floor() as isize;
        let max_x = (cx + radius).ceil() as isize;
        let min_y = (cy - radius).floor() as isize;
        let max_y = (cy + radius).ceil() as isize;

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                if px < 0 || px >= canvas.width as isize || py < 0 || py >= canvas.height as isize {
                    continue;
                }

                let dcx = (px as f32 + 0.5) - cx;
                let dcy = (py as f32 + 0.5) - cy;
                let dist_sq = dcx * dcx + dcy * dcy;

                if dist_sq <= r_sq {
                    let mut alpha_factor = 1.0;
                    
                    if dist_sq > hard_r_sq {
                        let d = dist_sq.sqrt();
                        let range = radius - hard_radius;
                        if range > 0.0 {
                            alpha_factor = 1.0 - ((d - hard_radius) / range);
                        }
                    }
                    
                    let src_a = alpha_factor * flow_mult * opac_mult;
                    if src_a <= 0.0 { continue; }

                    let idx = (py as usize) * canvas.width + (px as usize);
                    let current_pixel = canvas.layers[layer_idx].pixels[idx];
                    
                    let dst_a = current_pixel.a as f32 / 255.0;
                    let dst_r = current_pixel.r as f32;
                    let dst_g = current_pixel.g as f32;
                    let dst_b = current_pixel.b as f32;

                    let out_a = src_a + dst_a * (1.0 - src_a);
                    if out_a > 0.0 {
                        // LA COULEUR EST APPLIQUÉE ICI LORS DU MÉLANGE !
                        let final_r = (brush_r * src_a + dst_r * dst_a * (1.0 - src_a)) / out_a;
                        let final_g = (brush_g * src_a + dst_g * dst_a * (1.0 - src_a)) / out_a;
                        let final_b = (brush_b * src_a + dst_b * dst_a * (1.0 - src_a)) / out_a;
                        let target_alpha = (out_a * 255.0).min(opac_mult * 255.0) as u8;

                        canvas.layers[layer_idx].pixels[idx] = Rgba { 
                            r: final_r as u8, g: final_g as u8, b: final_b as u8, a: target_alpha 
                        };
                        modified = true;
                    }
                }
            }
        }
    }
    modified
}
