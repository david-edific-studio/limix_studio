use crate::core::canvas::{Canvas, Rgba};

fn sample(pixels: &[Rgba], w: usize, h: usize, x: isize, y: isize) -> Rgba {
    let cx = x.clamp(0, w as isize - 1) as usize;
    let cy = y.clamp(0, h as isize - 1) as usize;
    pixels[cy * w + cx]
}

/// Blur — flou gaussien local (rayon dépend de `size`)
pub fn apply_blur(canvas: &mut Canvas, layer_idx: usize, cx: f32, cy: f32, size: f32, strength: f32) -> bool {
    let w = canvas.width;
    let h = canvas.height;
    let radius = (size / 2.0) as isize;
    let blur_radius = (strength / 100.0 * 5.0).max(1.0) as isize;
    let brush_r_sq = (size / 2.0) * (size / 2.0);
    
    let snapshot = canvas.layers[layer_idx].pixels.clone();
    let selection_mask = canvas.selection_mask.clone();
    let mut modified = false;

    let min_x = (cx - size / 2.0).floor() as isize;
    let max_x = (cx + size / 2.0).ceil() as isize;
    let min_y = (cy - size / 2.0).floor() as isize;
    let max_y = (cy + size / 2.0).ceil() as isize;

    let _ = radius;

    for py in min_y..=max_y {
        for px in min_x..=max_x {
            if px < 0 || px >= w as isize || py < 0 || py >= h as isize { continue; }
            let dpx = px as f32 - cx;
            let dpy = py as f32 - cy;
            if dpx*dpx + dpy*dpy > brush_r_sq { continue; }
            let idx = py as usize * w + px as usize;
            if let Some(ref mask) = selection_mask {
                if mask[idx] == 0 { continue; }
            }
            let mut sr = 0u32; let mut sg = 0u32; let mut sb = 0u32; let mut sa = 0u32;
            let mut count = 0u32;
            for ky in -blur_radius..=blur_radius {
                for kx in -blur_radius..=blur_radius {
                    let p = sample(&snapshot, w, h, px + kx, py + ky);
                    sr += p.r as u32; sg += p.g as u32; sb += p.b as u32; sa += p.a as u32;
                    count += 1;
                }
            }
            canvas.layers[layer_idx].pixels[idx] = Rgba {
                r: (sr / count) as u8, g: (sg / count) as u8,
                b: (sb / count) as u8, a: (sa / count) as u8,
            };
            modified = true;
        }
    }
    modified
}

/// Sharpen — accentuation locale (unsharp mask simplifié)
pub fn apply_sharpen(canvas: &mut Canvas, layer_idx: usize, cx: f32, cy: f32, size: f32, strength: f32) -> bool {
    let w = canvas.width;
    let h = canvas.height;
    let brush_r_sq = (size / 2.0) * (size / 2.0);
    let amount = strength / 100.0;

    let snapshot = canvas.layers[layer_idx].pixels.clone();
    let selection_mask = canvas.selection_mask.clone();
    let mut modified = false;

    let min_x = (cx - size / 2.0).floor() as isize;
    let max_x = (cx + size / 2.0).ceil() as isize;
    let min_y = (cy - size / 2.0).floor() as isize;
    let max_y = (cy + size / 2.0).ceil() as isize;

    for py in min_y..=max_y {
        for px in min_x..=max_x {
            if px < 0 || px >= w as isize || py < 0 || py >= h as isize { continue; }
            let dpx = px as f32 - cx;
            let dpy = py as f32 - cy;
            if dpx*dpx + dpy*dpy > brush_r_sq { continue; }
            let idx = py as usize * w + px as usize;
            if let Some(ref mask) = selection_mask {
                if mask[idx] == 0 { continue; }
            }
            // Laplacian sharpen kernel
            let kernel = [
                (0i32, -1i32, -1f32), (-1, 0, -1.0), (0, 0, 5.0), (1, 0, -1.0), (0, 1, -1.0),
            ];
            let mut sr = 0f32; let mut sg = 0f32; let mut sb = 0f32;
            for (kdx, kdy, kw) in kernel {
                let p = sample(&snapshot, w, h, px + kdx as isize, py + kdy as isize);
                sr += p.r as f32 * kw; sg += p.g as f32 * kw; sb += p.b as f32 * kw;
            }
            let orig = snapshot[idx];
            let nr = (orig.r as f32 + (sr - orig.r as f32) * amount).clamp(0.0, 255.0) as u8;
            let ng = (orig.g as f32 + (sg - orig.g as f32) * amount).clamp(0.0, 255.0) as u8;
            let nb = (orig.b as f32 + (sb - orig.b as f32) * amount).clamp(0.0, 255.0) as u8;
            canvas.layers[layer_idx].pixels[idx] = Rgba { r: nr, g: ng, b: nb, a: orig.a };
            modified = true;
        }
    }
    modified
}

/// Smudge — doigt (mélange directionnel)
pub fn apply_smudge(canvas: &mut Canvas, layer_idx: usize, x0: f32, y0: f32, x1: f32, y1: f32, size: f32, strength: f32) -> bool {
    let w = canvas.width;
    let h = canvas.height;
    let brush_r_sq = (size / 2.0) * (size / 2.0);
    let mix = (strength / 100.0).clamp(0.0, 0.95);
    let dx = (x1 - x0).round() as isize;
    let dy = (y1 - y0).round() as isize;
    if dx == 0 && dy == 0 { return false; }

    let snapshot = canvas.layers[layer_idx].pixels.clone();
    let selection_mask = canvas.selection_mask.clone();
    let mut modified = false;

    let min_x = (x1 - size / 2.0).floor() as isize;
    let max_x = (x1 + size / 2.0).ceil() as isize;
    let min_y = (y1 - size / 2.0).floor() as isize;
    let max_y = (y1 + size / 2.0).ceil() as isize;

    for py in min_y..=max_y {
        for px in min_x..=max_x {
            if px < 0 || px >= w as isize || py < 0 || py >= h as isize { continue; }
            let dpx = px as f32 - x1;
            let dpy = py as f32 - y1;
            if dpx*dpx + dpy*dpy > brush_r_sq { continue; }
            let idx = py as usize * w + px as usize;
            if let Some(ref mask) = selection_mask {
                if mask[idx] == 0 { continue; }
            }
            let src = sample(&snapshot, w, h, px - dx, py - dy);
            let dst = snapshot[idx];
            let nr = (src.r as f32 * mix + dst.r as f32 * (1.0 - mix)) as u8;
            let ng = (src.g as f32 * mix + dst.g as f32 * (1.0 - mix)) as u8;
            let nb = (src.b as f32 * mix + dst.b as f32 * (1.0 - mix)) as u8;
            let na = (src.a as f32 * mix + dst.a as f32 * (1.0 - mix)) as u8;
            canvas.layers[layer_idx].pixels[idx] = Rgba { r: nr, g: ng, b: nb, a: na };
            modified = true;
        }
    }
    modified
}

/// Burn — assombrit la zone (augmente la densité)
pub fn apply_burn(canvas: &mut Canvas, layer_idx: usize, cx: f32, cy: f32, size: f32, strength: f32) -> bool {
    apply_dodge_burn_inner(canvas, layer_idx, cx, cy, size, strength, false)
}

/// Dodge — éclaircit la zone
pub fn apply_dodge(canvas: &mut Canvas, layer_idx: usize, cx: f32, cy: f32, size: f32, strength: f32) -> bool {
    apply_dodge_burn_inner(canvas, layer_idx, cx, cy, size, strength, true)
}

fn apply_dodge_burn_inner(canvas: &mut Canvas, layer_idx: usize, cx: f32, cy: f32, size: f32, strength: f32, dodge: bool) -> bool {
    let w = canvas.width;
    let h = canvas.height;
    let brush_r_sq = (size / 2.0) * (size / 2.0);
    let amount = strength / 100.0 * 0.3;
    let selection_mask = canvas.selection_mask.clone();
    let mut modified = false;

    let min_x = (cx - size / 2.0).floor() as isize;
    let max_x = (cx + size / 2.0).ceil() as isize;
    let min_y = (cy - size / 2.0).floor() as isize;
    let max_y = (cy + size / 2.0).ceil() as isize;

    for py in min_y..=max_y {
        for px in min_x..=max_x {
            if px < 0 || px >= w as isize || py < 0 || py >= h as isize { continue; }
            let dpx = px as f32 - cx;
            let dpy = py as f32 - cy;
            if dpx*dpx + dpy*dpy > brush_r_sq { continue; }
            let idx = py as usize * w + px as usize;
            if let Some(ref mask) = selection_mask {
                if mask[idx] == 0 { continue; }
            }
            let p = canvas.layers[layer_idx].pixels[idx];
            let factor = if dodge { 1.0 + amount } else { 1.0 - amount };
            let nr = (p.r as f32 * factor).clamp(0.0, 255.0) as u8;
            let ng = (p.g as f32 * factor).clamp(0.0, 255.0) as u8;
            let nb = (p.b as f32 * factor).clamp(0.0, 255.0) as u8;
            canvas.layers[layer_idx].pixels[idx] = Rgba { r: nr, g: ng, b: nb, a: p.a };
            modified = true;
        }
    }
    modified
}

/// Sponge — désature ou sature la zone
pub fn apply_sponge(canvas: &mut Canvas, layer_idx: usize, cx: f32, cy: f32, size: f32, strength: f32, saturate: bool) -> bool {
    let w = canvas.width;
    let h = canvas.height;
    let brush_r_sq = (size / 2.0) * (size / 2.0);
    let amount = strength / 100.0 * 0.15;
    let selection_mask = canvas.selection_mask.clone();
    let mut modified = false;

    let min_x = (cx - size / 2.0).floor() as isize;
    let max_x = (cx + size / 2.0).ceil() as isize;
    let min_y = (cy - size / 2.0).floor() as isize;
    let max_y = (cy + size / 2.0).ceil() as isize;

    for py in min_y..=max_y {
        for px in min_x..=max_x {
            if px < 0 || px >= w as isize || py < 0 || py >= h as isize { continue; }
            let dpx = px as f32 - cx;
            let dpy = py as f32 - cy;
            if dpx*dpx + dpy*dpy > brush_r_sq { continue; }
            let idx = py as usize * w + px as usize;
            if let Some(ref mask) = selection_mask {
                if mask[idx] == 0 { continue; }
            }
            let p = canvas.layers[layer_idx].pixels[idx];
            let lum = 0.299 * p.r as f32 + 0.587 * p.g as f32 + 0.114 * p.b as f32;
            let mix = if saturate { amount } else { -amount };
            let nr = (p.r as f32 + (p.r as f32 - lum) * mix).clamp(0.0, 255.0) as u8;
            let ng = (p.g as f32 + (p.g as f32 - lum) * mix).clamp(0.0, 255.0) as u8;
            let nb = (p.b as f32 + (p.b as f32 - lum) * mix).clamp(0.0, 255.0) as u8;
            canvas.layers[layer_idx].pixels[idx] = Rgba { r: nr, g: ng, b: nb, a: p.a };
            modified = true;
        }
    }
    modified
}

/// Clone Stamp — copie depuis un point source
pub fn apply_clone(
    canvas: &mut Canvas, layer_idx: usize,
    dest_x: f32, dest_y: f32,
    src_x: f32, src_y: f32,
    size: f32, opacity: f32,
) -> bool {
    let w = canvas.width;
    let h = canvas.height;
    let brush_r_sq = (size / 2.0) * (size / 2.0);
    let opac = opacity / 100.0;
    let snapshot = canvas.layers[layer_idx].pixels.clone();
    let selection_mask = canvas.selection_mask.clone();
    let mut modified = false;

    let offset_x = (dest_x - src_x) as isize;
    let offset_y = (dest_y - src_y) as isize;

    let min_x = (dest_x - size / 2.0).floor() as isize;
    let max_x = (dest_x + size / 2.0).ceil() as isize;
    let min_y = (dest_y - size / 2.0).floor() as isize;
    let max_y = (dest_y + size / 2.0).ceil() as isize;

    for py in min_y..=max_y {
        for px in min_x..=max_x {
            if px < 0 || px >= w as isize || py < 0 || py >= h as isize { continue; }
            let dpx = px as f32 - dest_x;
            let dpy = py as f32 - dest_y;
            if dpx*dpx + dpy*dpy > brush_r_sq { continue; }
            let idx = py as usize * w + px as usize;
            if let Some(ref mask) = selection_mask {
                if mask[idx] == 0 { continue; }
            }
            let src = sample(&snapshot, w, h, px - offset_x, py - offset_y);
            let dst = snapshot[idx];
            let src_a = src.a as f32 / 255.0 * opac;
            let dst_a = dst.a as f32 / 255.0;
            let out_a = src_a + dst_a * (1.0 - src_a);
            if out_a > 0.0 {
                let fr = (src.r as f32 * src_a + dst.r as f32 * dst_a * (1.0 - src_a)) / out_a;
                let fg = (src.g as f32 * src_a + dst.g as f32 * dst_a * (1.0 - src_a)) / out_a;
                let fb = (src.b as f32 * src_a + dst.b as f32 * dst_a * (1.0 - src_a)) / out_a;
                canvas.layers[layer_idx].pixels[idx] = Rgba { r: fr as u8, g: fg as u8, b: fb as u8, a: (out_a * 255.0) as u8 };
                modified = true;
            }
        }
    }
    modified
}
