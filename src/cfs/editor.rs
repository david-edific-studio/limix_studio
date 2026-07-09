use eframe::egui;
use crate::ui::{LimixApp, AppState};
use crate::core::canvas::Rgba;
use crate::cfs::engine::execute_cfs;

pub fn show(app: &mut LimixApp, ctx: &egui::Context) {
    
    // AUTO-PREVIEW INITIAL (Calcule l'image dès l'ouverture du studio)
    if app.cfs_preview_texture.is_none() {
        update_preview(app, ctx);
    }

    // --- 1. BANDEAU SUPÉRIEUR (HEADER) ---
    egui::TopBottomPanel::top("cfs_top_panel").exact_height(50.0).show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            
            if ui.add_sized([100.0, 30.0], egui::Button::new("🔙 Annuler")).clicked() {
                app.cfs_preview_texture = None;
                app.state = AppState::Workspace;
            }
            
            ui.add_space(20.0);
            ui.heading(egui::RichText::new("⚡ CFS Studio").color(egui::Color32::from_rgb(250, 181, 99)).strong());
            ui.label(egui::RichText::new("| Live-Preview Actif (Auto-save)").color(egui::Color32::DARK_GRAY));
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                
                // BOUTON FINAL : Valider & Injecter (Ne bloque plus jamais !)
                if ui.add_sized([180.0, 35.0], egui::Button::new(egui::RichText::new("✅ Valider & Injecter").strong().color(egui::Color32::WHITE))
                    .fill(egui::Color32::from_rgb(217, 90, 38))).clicked() 
                {
                    // Même si le code a une erreur de syntaxe, on sauvegarde le script !
                    let new_pixels = match execute_cfs(&app.cfs_code, app.engine.width, app.engine.height) {
                        Ok(p) => p,
                        Err(_) => vec![Rgba { r: 0, g: 0, b: 0, a: 0 }; app.engine.width * app.engine.height],
                    };

                    if let Some(idx) = app.editing_cfs_index {
                        app.engine.layers[idx].script = Some(app.cfs_code.clone());
                        app.engine.layers[idx].pixels = new_pixels;
                    } else {
                        let insert_idx = if app.engine.layers.is_empty() { 0 } else { app.active_layer + 1 };
                        let depth = if app.engine.layers.is_empty() { 0 } else { app.engine.layers[app.active_layer].depth };
                        let name = format!("Script JS {}", app.engine.layers.len());
                        
                        app.engine.insert_dynamic(insert_idx, &name, depth, &app.cfs_code);
                        app.engine.layers[insert_idx].pixels = new_pixels;
                        app.active_layer = insert_idx;
                    }
                    
                    app.texture = None; // Forcer la mise à jour de la feuille globale
                    app.cfs_preview_texture = None; 
                    app.state = AppState::Workspace; 
                }
            });
        });
    });

    // --- 2. PANNEAU DE GAUCHE (ÉDITEUR DE CODE AVEC AUTO-SAVE) ---
    egui::SidePanel::left("cfs_code_panel").exact_width(450.0).show(ctx, |ui| {
        let frame = egui::Frame::none().fill(egui::Color32::from_rgb(20, 20, 25)).inner_margin(15.0);
        
        frame.show(ui, |ui| {
            ui.label(egui::RichText::new("Code Source (JavaScript)").strong().color(egui::Color32::LIGHT_GRAY));
            ui.add_space(10.0);
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                let text_response = ui.add_sized(
                    [ui.available_width(), ui.available_height()],
                    egui::TextEdit::multiline(&mut app.cfs_code)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .lock_focus(true)
                );

                // LA MAGIE DU LIVE-PREVIEW : À chaque touche tapée, ça recalcule la Vague Isométrique !
                if text_response.changed() {
                    update_preview(app, ctx);
                }
            });
        });
    });

    // --- 3. PANNEAU CENTRAL (APERÇU ADAPTATIF ET PARFAITEMENT CENTRÉ) ---
    egui::CentralPanel::default().show(ctx, |ui| {
        let frame = egui::Frame::none().fill(egui::Color32::from_rgb(26, 26, 26)).inner_margin(20.0);
        
        frame.show(ui, |ui| {
            // Centrage mathématique absolu géré par egui
            ui.centered_and_justified(|ui| {
                
                let available_space = ui.available_size();
                let canvas_w = app.engine.width as f32;
                let canvas_h = app.engine.height as f32;
                
                // Calcul du ratio pour que la feuille rentre parfaitement (95% de l'espace)
                let scale_x = available_space.x / canvas_w;
                let scale_y = available_space.y / canvas_h;
                let scale = scale_x.min(scale_y) * 0.95; 

                let display_w = canvas_w * scale;
                let display_h = canvas_h * scale;

                if let Some(workspace_texture) = &app.texture {
                    // On alloue un rectangle rigide avec la taille parfaitement calculée
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(display_w, display_h), egui::Sense::hover());
                    
                    // 1. Fond du Workspace (Damier + Autres calques)
                    let mut base_mesh = egui::Mesh::with_texture(workspace_texture.id());
                    base_mesh.add_rect_with_uv(rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
                    ui.painter().add(egui::Shape::mesh(base_mesh));

                    // 2. Calque CFS superposé par-dessus avec le rendu en temps réel
                    if let Some(preview_tex) = &app.cfs_preview_texture {
                        let mut preview_mesh = egui::Mesh::with_texture(preview_tex.id());
                        preview_mesh.add_rect_with_uv(rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
                        ui.painter().add(egui::Shape::mesh(preview_mesh));
                    }
                } else {
                    ui.label("Erreur : Aucun canevas détecté.");
                }
            });
        });
    });
}

// Fonction de mise à jour instantanée du cache graphique
fn update_preview(app: &mut LimixApp, ctx: &egui::Context) {
    match execute_cfs(&app.cfs_code, app.engine.width, app.engine.height) {
        Ok(pixels) => {
            let mut raw_pixels = Vec::with_capacity(pixels.len() * 4);
            for p in pixels {
                raw_pixels.push(p.r); raw_pixels.push(p.g); raw_pixels.push(p.b); raw_pixels.push(p.a);
            }
            let color_image = egui::ColorImage::from_rgba_unmultiplied([app.engine.width, app.engine.height], &raw_pixels);
            app.cfs_preview_texture = Some(ctx.load_texture("cfs_preview", color_image, egui::TextureOptions::NEAREST));
        },
        Err(_) => {
            // Si l'utilisateur fait une faute de frappe, on ne fait rien.
            // Le dernier aperçu "valide" reste figé à l'écran jusqu'à ce que la faute soit corrigée !
        }
    }
}
