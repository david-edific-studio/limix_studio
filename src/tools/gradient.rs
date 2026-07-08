use crate::core::canvas::{Canvas, Rgba};

pub fn apply_linear(
    canvas: &mut Canvas,
    layer_idx: usize,
    x0: f32, y0: f32,
    x1: f32, y1: f32,
    color_start: [f32; 3],
    color_end: [f32; 3],
    opacity: f32,
) -> bool {
    let w = canvas.width;
    let h = canvas.height;
    let opac = opacity / 100.0;

    let dx = x1 - x0;
    let dy = y1 - y0;
    let len_sq = dx * dx + dy * dy;
    if len_sq < 1.0 { return false; }

    let selection_mask = canvas.selection_mask.clone();

    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;

            if let Some(ref mask) = selection_mask {
                if mask[idx] == 0 { continue; }
            }

            // Projection du pixel sur le vecteur de dégradé
            let px = x as f32 - x0;
            let py = y as f32 - y0;
            let t = ((px * dx + py * dy) / len_sq).clamp(0.0, 1.0);

            let r = (color_start[0] + (color_end[0] - color_start[0]) * t) * 255.0;
            let g = (color_start[1] + (color_end[1] - color_start[1]) * t) * 255.0;
            let b = (color_start[2] + (color_end[2] - color_start[2]) * t) * 255.0;
            let a = opac * 255.0;

            let src = Rgba { r: r as u8, g: g as u8, b: b as u8, a: a as u8 };
            let dst = canvas.layers[layer_idx].pixels[idx];

            let src_a = src.a as f32 / 255.0;
            let dst_a = dst.a as f32 / 255.0;
            let out_a = src_a + dst_a * (1.0 - src_a);

            if out_a > 0.0 {
                let fr = (src.r as f32 * src_a + dst.r as f32 * dst_a * (1.0 - src_a)) / out_a;
                let fg = (src.g as f32 * src_a + dst.g as f32 * dst_a * (1.0 - src_a)) / out_a;
                let fb = (src.b as f32 * src_a + dst.b as f32 * dst_a * (1.0 - src_a)) / out_a;
                canvas.layers[layer_idx].pixels[idx] = Rgba {
                    r: fr as u8, g: fg as u8, b: fb as u8, a: (out_a * 255.0) as u8,
                };
            }
        }
    }
    true
}
