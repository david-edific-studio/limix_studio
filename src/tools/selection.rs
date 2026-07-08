use crate::core::canvas::Canvas;

pub fn apply_rect(canvas: &mut Canvas, start_x: f32, start_y: f32, end_x: f32, end_y: f32, add: bool) {
    let min_x = start_x.min(end_x).max(0.0) as usize;
    let min_y = start_y.min(end_y).max(0.0) as usize;
    let max_x = start_x.max(end_x).min(canvas.width as f32) as usize;
    let max_y = start_y.max(end_y).min(canvas.height as f32) as usize;

    if canvas.selection_mask.is_none() {
        canvas.selection_mask = Some(vec![0; canvas.width * canvas.height]);
    }

    if let Some(mask) = &mut canvas.selection_mask {
        if !add {
            // Si on n'ajoute pas (touche Maj non pressée par ex), on réinitialise d'abord le masque
            for val in mask.iter_mut() {
                *val = 0;
            }
        }
        for y in min_y..max_y {
            for x in min_x..max_x {
                if x < canvas.width && y < canvas.height {
                    mask[y * canvas.width + x] = 255;
                }
            }
        }
    }
}

pub fn apply_ellipse(canvas: &mut Canvas, start_x: f32, start_y: f32, end_x: f32, end_y: f32, add: bool) {
    let min_x = start_x.min(end_x);
    let min_y = start_y.min(end_y);
    let max_x = start_x.max(end_x);
    let max_y = start_y.max(end_y);

    let cx = (min_x + max_x) / 2.0;
    let cy = (min_y + max_y) / 2.0;
    let rx = (max_x - min_x) / 2.0;
    let ry = (max_y - min_y) / 2.0;

    if rx <= 0.0 || ry <= 0.0 {
        return;
    }

    if canvas.selection_mask.is_none() {
        canvas.selection_mask = Some(vec![0; canvas.width * canvas.height]);
    }

    if let Some(mask) = &mut canvas.selection_mask {
        if !add {
            for val in mask.iter_mut() {
                *val = 0;
            }
        }
        
        let start_x_idx = min_x.max(0.0) as usize;
        let start_y_idx = min_y.max(0.0) as usize;
        let end_x_idx = max_x.min(canvas.width as f32) as usize;
        let end_y_idx = max_y.min(canvas.height as f32) as usize;

        for y in start_y_idx..end_y_idx {
            for x in start_x_idx..end_x_idx {
                if x < canvas.width && y < canvas.height {
                    let dx = (x as f32 - cx) / rx;
                    let dy = (y as f32 - cy) / ry;
                    if dx * dx + dy * dy <= 1.0 {
                        mask[y * canvas.width + x] = 255;
                    }
                }
            }
        }
    }
}
