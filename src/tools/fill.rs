use crate::core::canvas::{Canvas, Rgba};

pub fn apply(
    canvas: &mut Canvas,
    layer_idx: usize,
    x: usize,
    y: usize,
    color: [f32; 3],
    tolerance: f32,
    opacity: f32,
) -> bool {
    let w = canvas.width;
    let h = canvas.height;
    if x >= w || y >= h { return false; }

    let fill_r = (color[0] * 255.0) as u8;
    let fill_g = (color[1] * 255.0) as u8;
    let fill_b = (color[2] * 255.0) as u8;
    let fill_a = (opacity / 100.0 * 255.0) as u8;
    
    let target = canvas.layers[layer_idx].pixels[y * w + x];
    
    // Si la couleur cible est la même que la couleur de remplissage, on sort
    let color_diff = |a: Rgba, b: Rgba| -> f32 {
        let dr = a.r as f32 - b.r as f32;
        let dg = a.g as f32 - b.g as f32;
        let db = a.b as f32 - b.b as f32;
        let da = a.a as f32 - b.a as f32;
        (dr*dr + dg*dg + db*db + da*da).sqrt()
    };
    
    let fill_pixel = Rgba { r: fill_r, g: fill_g, b: fill_b, a: fill_a };
    if color_diff(target, fill_pixel) < 1.0 { return false; }
    
    let threshold = tolerance * 4.41; // 255 * sqrt(4) = 510, scale to 0-255 range roughly

    let mut stack = vec![(x, y)];
    let mut visited = vec![false; w * h];
    visited[y * w + x] = true;
    let mut modified = false;

    let selection_mask = canvas.selection_mask.clone();

    while let Some((cx, cy)) = stack.pop() {
        let idx = cy * w + cx;

        if let Some(ref mask) = selection_mask {
            if mask[idx] == 0 { continue; }
        }

        let current = canvas.layers[layer_idx].pixels[idx];
        if color_diff(current, target) > threshold { continue; }

        canvas.layers[layer_idx].pixels[idx] = fill_pixel;
        modified = true;

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

    modified
}
