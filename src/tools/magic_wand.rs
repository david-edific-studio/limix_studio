use crate::core::canvas::{Canvas, Rgba};

/// Magic Wand — sélectionne par tolérance de couleur (flood fill sur le masque)
pub fn apply(canvas: &mut Canvas, x: usize, y: usize, tolerance: f32, add: bool) -> bool {
    let w = canvas.width;
    let h = canvas.height;
    if x >= w || y >= h { return false; }

    if canvas.selection_mask.is_none() || !add {
        canvas.selection_mask = Some(vec![0u8; w * h]);
    }

    // Calcule la différence de couleur entre deux pixels
    let color_diff = |a: Rgba, b: Rgba| -> f32 {
        let dr = a.r as f32 - b.r as f32;
        let dg = a.g as f32 - b.g as f32;
        let db = a.b as f32 - b.b as f32;
        let da = a.a as f32 - b.a as f32;
        (dr*dr + dg*dg + db*db + da*da).sqrt()
    };

    let threshold = tolerance * 8.83; // max diff sqrt(4*255^2) ≈ 510; scale from 0-100 range
    let target = canvas.layers.iter().rev().find(|l| !l.is_folder && l.visible)
        .map(|l| l.pixels[y * w + x])
        .unwrap_or(Rgba { r: 0, g: 0, b: 0, a: 0 });

    // Composite the canvas so we do color matching on the visible result
    let flattened = canvas.render_flattened();
    let target_flat = flattened[y * w + x];

    let mut stack = vec![(x, y)];
    let mut visited = vec![false; w * h];
    visited[y * w + x] = true;

    while let Some((cx, cy)) = stack.pop() {
        let idx = cy * w + cx;
        let current = flattened[idx];
        if color_diff(current, target_flat) > threshold { continue; }
        if let Some(ref mut mask) = canvas.selection_mask {
            mask[idx] = 255;
        }

        let neighbors = [
            (cx.wrapping_sub(1), cy),
            (cx + 1, cy),
            (cx, cy.wrapping_sub(1)),
            (cx, cy + 1),
        ];
        for (nx, ny) in neighbors {
            if nx < w && ny < h {
                let nidx = ny * w + nx;
                if !visited[nidx] {
                    visited[nidx] = true;
                    stack.push((nx, ny));
                }
            }
        }
    }

    let _ = target; // used for type inference
    true
}
