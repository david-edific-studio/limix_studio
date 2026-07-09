pub mod welcome;

use eframe::egui;
use crate::core::canvas::{Canvas, Rgba, LayerType};
use crate::tools::Tool;

#[derive(PartialEq)]
pub enum AppState {
    WelcomeScreen, 
    Workspace,     
    CfsEditor, // --- NOUVELLE PAGE PLEIN ÉCRAN ! ---
}

pub struct NewProjectParams {
    pub width: usize,
    pub height: usize,
    pub bits: u8,
    pub bg_color: [f32; 3], 
    pub orientation: usize, 
    pub show_modal: bool,   
}

impl Default for NewProjectParams {
    fn default() -> Self {
        Self { width: 1920, height: 1080, bits: 8, bg_color: [1.0, 1.0, 1.0], orientation: 0, show_modal: false }
    }
}

pub struct LimixApp {
    pub engine: Canvas,
    pub texture: Option<egui::TextureHandle>,
    pub current_tool: Tool,
    pub active_layer: usize,
    
    pub brush_size: f32,
    pub brush_hardness: f32,
    pub brush_opacity: f32,
    pub brush_flow: f32,
    pub primary_color: [f32; 3],
    pub last_draw_pos: Option<(f32, f32)>, 
    
    pub renaming_layer: Option<(usize, String)>, 
    pub dragging_layer: Option<usize>,           

    pub transform_state: Option<(usize, Vec<Rgba>, Vec<(isize, isize, Rgba)>, egui::Rect, egui::Rect, Option<usize>)>,
    pub transform_active: bool, 

    pub zoom: f32,
    pub state: AppState,       
    pub show_settings: bool,   
    pub new_proj_params: NewProjectParams, 

    // --- MODULE CFS (Architecture Plein Écran) ---
    pub cfs_code: String,
    pub cfs_preview_texture: Option<egui::TextureHandle>, 
    pub editing_cfs_index: Option<usize>,                 
}

impl LimixApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut visuals = egui::Visuals::dark();
        visuals.selection.bg_fill = egui::Color32::from_rgb(255, 136, 0);
        cc.egui_ctx.set_visuals(visuals);

        let mut engine = Canvas::new(800, 600);
        
        engine.add_layer("Arrière-plan", 0);
        engine.add_layer("Tracé Principal", 0);

        let default_js = r#"// Limix Pro - CFS Mode
// Cible du rendu : Layer Actif

function render(ctx) {
    // Exemple : Dessiner un rectangle Orange LP
    ctx.fillStyle = "rgba(250, 181, 99, 1.0)";
    ctx.fillRect(50, 50, 400, 400);
}
"#.to_string();

        Self {
            engine,
            texture: None,
            current_tool: Tool::Brush,
            active_layer: 1, 
            
            brush_size: 40.0,
            brush_hardness: 50.0,
            brush_opacity: 100.0,
            brush_flow: 100.0,
            primary_color: [0.0, 0.0, 0.0], 
            last_draw_pos: None,
            
            renaming_layer: None,
            dragging_layer: None,
            
            transform_state: None,
            transform_active: true,

            zoom: 1.0,
            state: AppState::WelcomeScreen,
            show_settings: false,
            new_proj_params: NewProjectParams::default(),
            
            cfs_code: default_js,
            cfs_preview_texture: None,
            editing_cfs_index: None,
        }
    }

    fn refresh_gpu_texture(&mut self, ctx: &egui::Context) {
        let resultat = self.engine.render_flattened();
        let mut raw_pixels = Vec::with_capacity(resultat.len() * 4);
        for p in resultat {
            raw_pixels.push(p.r); raw_pixels.push(p.g); raw_pixels.push(p.b); raw_pixels.push(p.a);
        }
        let color_image = egui::ColorImage::from_rgba_unmultiplied([self.engine.width, self.engine.height], &raw_pixels);

        if let Some(tex) = &mut self.texture { tex.set(color_image, egui::TextureOptions::NEAREST); } 
        else { self.texture = Some(ctx.load_texture("canvas_render", color_image, egui::TextureOptions::NEAREST)); }
    }
}

impl eframe::App for LimixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        if self.texture.is_none() { self.refresh_gpu_texture(ctx); }
        
        match self.state {
            AppState::WelcomeScreen => { crate::ui::welcome::show(self, ctx); }
            AppState::CfsEditor => { crate::cfs::editor::show(self, ctx); } // APPEL DE LA PAGE PLEIN ÉCRAN
            AppState::Workspace => {
                
                let mut needs_gpu_refresh = false;
                let mut context_action: Option<(String, usize, usize)> = None;

                egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
                    ui.add_space(2.0); 
                    ui.horizontal(|ui| {
                        if ui.button("🏠 Accueil").clicked() { self.state = AppState::WelcomeScreen; }
                        ui.separator();
                        ui.menu_button("Fichier", |ui| {
                            if ui.button("Nouveau...").clicked() { self.new_proj_params.show_modal = true; self.state = AppState::WelcomeScreen; }
                            if ui.button("Ouvrir...").clicked() {}
                            ui.separator();
                            if ui.button("Enregistrer").clicked() {}
                            if ui.button("Enregistrer sous...").clicked() {}
                            ui.separator();
                            ui.menu_button("Exporter", |ui| {
                                if ui.button("Exporter en PNG").clicked() {}
                                if ui.button("Exporter en JPEG").clicked() {}
                                if ui.button("Exporter en WebP").clicked() {}
                                if ui.button("Exporter en SVG").clicked() {}
                                if ui.button("Exporter en PDF").clicked() {}
                            });
                            if ui.button("Importer...").clicked() {}
                            ui.separator();
                            if ui.button("Propriétés du document").clicked() {}
                            ui.separator();
                            if ui.button("Quitter").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                        });
                        ui.menu_button("Édition", |ui| {
                            if ui.button("Annuler (Undo)").clicked() {}
                            if ui.button("Rétablir (Redo)").clicked() {}
                            ui.separator();
                            if ui.button("Couper").clicked() {}
                            if ui.button("Copier").clicked() {}
                            if ui.button("Coller").clicked() {}
                            if ui.button("Coller spécial (sur place)").clicked() {}
                            ui.separator();
                            if ui.button("Remplir...").clicked() {}
                            if ui.button("Contour (Stroke)...").clicked() {}
                            ui.separator();
                            if ui.button("Préférences").clicked() {}
                        });
                        ui.menu_button("Image", |ui| {
                            if ui.button("Mode colorimétrique...").clicked() {}
                            ui.separator();
                            if ui.button("Taille de l'image...").clicked() {}
                            if ui.button("Taille du canevas...").clicked() {}
                            ui.separator();
                            if ui.button("Recadrer").clicked() {}
                            if ui.button("Rogner (Trim)...").clicked() {}
                            if ui.button("Rotation du canevas").clicked() {}
                        });
                        ui.menu_button("Calque", |ui| {
                            if ui.button("Nouveau calque").clicked() {}
                            if ui.button("Dupliquer le calque").clicked() {}
                            if ui.button("Supprimer le calque").clicked() {}
                            ui.separator();
                            if ui.button("Fusionner vers le bas").clicked() {}
                            if ui.button("Aplatir l'image").clicked() {}
                            ui.separator();
                            // BOUTON CFS DANS LE MENU
                            if ui.button("⚡ Nouveau Script CFS...").clicked() {
                                self.editing_cfs_index = None;
                                self.cfs_preview_texture = None;
                                self.state = AppState::CfsEditor; 
                                ui.close_menu();
                            }
                            ui.separator();
                            if ui.button("Style de calque...").clicked() {}
                            if ui.button("Masque de calque...").clicked() {}
                        });
                        ui.menu_button("Sélection", |ui| {
                            if ui.button("Tout sélectionner").clicked() {
                                self.transform_active = true;
                                ui.close_menu();
                            }
                            if ui.button("Désélectionner").clicked() {
                                self.transform_active = false;
                                self.transform_state = None;
                                ui.close_menu();
                            }
                            if ui.button("Inverser la sélection").clicked() {}
                            ui.separator();
                            ui.menu_button("Modifier", |ui| {
                                if ui.button("Étendre...").clicked() {}
                                if ui.button("Adoucir (Feather)...").clicked() {}
                            });
                            ui.separator();
                            if ui.button("Enregistrer la sélection").clicked() {}
                        });
                        ui.menu_button("Filtre", |ui| {
                            if ui.button("Dernier filtre appliqué (Ctrl+F)").clicked() {}
                            ui.separator();
                            if ui.button("Galerie de filtres...").clicked() {}
                        });
                        ui.menu_button("Affichage", |ui| {
                            if ui.button("Zoom +").clicked() {}
                            if ui.button("Zoom -").clicked() {}
                            if ui.button("Ajuster à l'écran").clicked() {}
                            ui.separator();
                            let mut dummy_bool = false; 
                            ui.checkbox(&mut dummy_bool, "Règles");
                            ui.checkbox(&mut dummy_bool, "Repères (guides)");
                            ui.checkbox(&mut dummy_bool, "Grille");
                            ui.separator();
                            if ui.button("Mode plein écran").clicked() {}
                        });
                        ui.menu_button("Fenêtre", |ui| {
                            let mut tools_visible = true; 
                            ui.checkbox(&mut tools_visible, "Panneau Outils");
                            let mut layers_visible = true; 
                            ui.checkbox(&mut layers_visible, "Panneau Calques");
                            let mut dummy_bool = false; 
                            ui.checkbox(&mut dummy_bool, "Panneau Couleurs");
                            ui.checkbox(&mut dummy_bool, "Panneau Historique");
                        });
                    });
                    ui.add_space(2.0);
                });
                
                egui::TopBottomPanel::top("tool_options").exact_height(35.0).show(ctx, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.add_space(10.0); ui.strong(format!("{:?}", self.current_tool)); ui.separator();
                        match self.current_tool {
                            Tool::Brush | Tool::Pencil | Tool::Eraser => {
                                ui.label("Taille :"); ui.add(egui::Slider::new(&mut self.brush_size, 1.0..=500.0).text("px")); ui.separator();
                                ui.label("Dureté :"); ui.add(egui::Slider::new(&mut self.brush_hardness, 0.0..=100.0).text("%")); ui.separator();
                                ui.label("Opacité :"); ui.add(egui::Slider::new(&mut self.brush_opacity, 0.0..=100.0).text("%")); ui.separator();
                                ui.label("Flux :"); ui.add(egui::Slider::new(&mut self.brush_flow, 0.0..=100.0).text("%"));
                            }
                            Tool::Text => {
                                ui.label("Police :"); if ui.button("Arial").clicked() {}
                                ui.label("Taille :"); ui.add(egui::Slider::new(&mut self.brush_size, 8.0..=144.0).text("pt"));
                            }
                            _ => { ui.label("Prêt."); }
                        }
                    });
                });

                egui::TopBottomPanel::bottom("status_bar").exact_height(28.0).show(ctx, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.add_space(10.0); ui.label("Zoom :");
                        if ui.button("➖").clicked() { self.zoom = (self.zoom - 0.1).max(0.05); }
                        ui.label(format!("{} %", (self.zoom * 100.0).round()));
                        if ui.button("➕").clicked() { self.zoom = (self.zoom + 0.1).min(5.0); }
                        ui.separator(); ui.label(format!("Document : {} x {} px", self.engine.width, self.engine.height));
                    });
                });

                egui::SidePanel::left("toolbar").resizable(false).exact_width(65.0).show(ctx, |ui| {
                    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(10.0);
                            add_tool_button(ui, &mut self.current_tool, Tool::Move, egui::include_image!("../../assets/icons/move.svg"), "Déplacement");
                            add_tool_button(ui, &mut self.current_tool, Tool::Crop, egui::include_image!("../../assets/icons/crop.svg"), "Recadrage");
                            ui.separator();
                            add_tool_button(ui, &mut self.current_tool, Tool::SelectionRect, egui::include_image!("../../assets/icons/select_rect.svg"), "Sél. Rectangulaire");
                            add_tool_button(ui, &mut self.current_tool, Tool::SelectionEllipse, egui::include_image!("../../assets/icons/select_ellipse.svg"), "Sél. Elliptique");
                            add_tool_button(ui, &mut self.current_tool, Tool::LassoFree, egui::include_image!("../../assets/icons/lasso_free.svg"), "Lasso");
                            add_tool_button(ui, &mut self.current_tool, Tool::LassoPoly, egui::include_image!("../../assets/icons/lasso_poly.svg"), "Lasso Polygonal");
                            add_tool_button(ui, &mut self.current_tool, Tool::LassoMagnetic, egui::include_image!("../../assets/icons/lasso_magnetic.svg"), "Lasso Magnétique");
                            add_tool_button(ui, &mut self.current_tool, Tool::MagicWand, egui::include_image!("../../assets/icons/magic_wand.svg"), "Baguette Magique");
                            add_tool_button(ui, &mut self.current_tool, Tool::SelectQuick, egui::include_image!("../../assets/icons/select_quick.svg"), "Sélection Rapide");
                            add_tool_button(ui, &mut self.current_tool, Tool::RemoveBg, egui::include_image!("../../assets/icons/remove_bg.svg"), "Supprimer le fond");
                            ui.separator();
                            add_tool_button(ui, &mut self.current_tool, Tool::Brush, egui::include_image!("../../assets/icons/brush.svg"), "Pinceau");
                            add_tool_button(ui, &mut self.current_tool, Tool::Pencil, egui::include_image!("../../assets/icons/pencil.svg"), "Crayon");
                            add_tool_button(ui, &mut self.current_tool, Tool::Eraser, egui::include_image!("../../assets/icons/eraser.svg"), "Gomme");
                            add_tool_button(ui, &mut self.current_tool, Tool::Fill, egui::include_image!("../../assets/icons/fill.svg"), "Pot de peinture");
                            add_tool_button(ui, &mut self.current_tool, Tool::Gradient, egui::include_image!("../../assets/icons/gradient.svg"), "Dégradé");
                            ui.separator();
                            add_tool_button(ui, &mut self.current_tool, Tool::CloneStamp, egui::include_image!("../../assets/icons/clone_stamp.svg"), "Tampon");
                            add_tool_button(ui, &mut self.current_tool, Tool::HealingBrush, egui::include_image!("../../assets/icons/healing_brush.svg"), "Correcteur");
                            add_tool_button(ui, &mut self.current_tool, Tool::Eyedropper, egui::include_image!("../../assets/icons/eyedropper.svg"), "Pipette");
                            add_tool_button(ui, &mut self.current_tool, Tool::Sharpen, egui::include_image!("../../assets/icons/sharpen.svg"), "Netteté");
                            add_tool_button(ui, &mut self.current_tool, Tool::Blur, egui::include_image!("../../assets/icons/blur.svg"), "Goutte d'eau (Flou)");
                            add_tool_button(ui, &mut self.current_tool, Tool::Smudge, egui::include_image!("../../assets/icons/smudge.svg"), "Doigt");
                            add_tool_button(ui, &mut self.current_tool, Tool::Burn, egui::include_image!("../../assets/icons/burn.svg"), "Densité +");
                            add_tool_button(ui, &mut self.current_tool, Tool::Dodge, egui::include_image!("../../assets/icons/dodge.svg"), "Densité -");
                            add_tool_button(ui, &mut self.current_tool, Tool::Sponge, egui::include_image!("../../assets/icons/sponge.svg"), "Éponge");
                            ui.separator();
                            add_tool_button(ui, &mut self.current_tool, Tool::Pen, egui::include_image!("../../assets/icons/pen.svg"), "Plume");
                            add_tool_button(ui, &mut self.current_tool, Tool::Shapes, egui::include_image!("../../assets/icons/shapes.svg"), "Formes");
                            add_tool_button(ui, &mut self.current_tool, Tool::Text, egui::include_image!("../../assets/icons/text.svg"), "Texte");
                            ui.add_space(10.0);
                        });
                    });
                });

                egui::SidePanel::right("layers_panel").resizable(true).min_width(280.0).show(ctx, |ui| {
                    
                    ui.add_space(10.0); ui.heading("Couleurs"); ui.separator();
                    ui.horizontal(|ui| { ui.label("Active :"); ui.color_edit_button_rgb(&mut self.primary_color); });
                    ui.add_space(5.0); ui.label("Nuancier Rapide :"); ui.add_space(5.0);
                    ui.horizontal_wrapped(|ui| {
                        let swatches = [
                            [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0],
                            [0.0, 0.5, 1.0], [1.0, 0.5, 0.0], [0.8, 0.0, 1.0], [1.0, 0.0, 0.5],
                        ];
                        for swatch in swatches {
                            let c = egui::Color32::from_rgb((swatch[0]*255.0) as u8, (swatch[1]*255.0) as u8, (swatch[2]*255.0) as u8);
                            if ui.add(egui::Button::new("").fill(c).min_size(egui::vec2(22.0, 22.0))).clicked() { self.primary_color = swatch; }
                        }
                    });

                    ui.add_space(15.0);
                    let rect = ui.allocate_space(egui::vec2(ui.available_width(), 3.0)).1;
                    ui.painter().rect_filled(rect, 1.5, egui::Color32::from_rgb(255, 136, 0));
                    ui.add_space(10.0);

                    ui.heading("Calques"); ui.separator();
                    if self.active_layer < self.engine.layers.len() {
                        let active_layer_idx = self.active_layer;
                        let layer = &mut self.engine.layers[active_layer_idx];

                        egui::Grid::new("layer_props_grid").num_columns(2).spacing([10.0, 10.0]).show(ui, |ui| {
                            ui.label("Mode :");
                            let mut current_blend = layer.blend_mode;
                            egui::ComboBox::from_id_source("blend_mode").selected_text(match current_blend {
                                1 => "Multiplier", 2 => "Écran", 3 => "Incrustation", _ => "Normal",
                            }).width(120.0).show_ui(ui, |ui| {
                                if ui.selectable_value(&mut current_blend, 0, "Normal").changed() { needs_gpu_refresh = true; }
                                if ui.selectable_value(&mut current_blend, 1, "Multiplier").changed() { needs_gpu_refresh = true; }
                                if ui.selectable_value(&mut current_blend, 2, "Écran").changed() { needs_gpu_refresh = true; }
                                if ui.selectable_value(&mut current_blend, 3, "Incrustation").changed() { needs_gpu_refresh = true; }
                            });
                            layer.blend_mode = current_blend; ui.end_row();

                            ui.label("Opacité :");
                            let mut current_opacity = layer.opacity * 100.0;
                            if ui.add(egui::Slider::new(&mut current_opacity, 0.0..=100.0).text("%")).changed() {
                                layer.opacity = current_opacity / 100.0; needs_gpu_refresh = true;
                            }
                            ui.end_row();
                        });
                    }
                    ui.separator();
                    
                    // CORRECTION ICI : Typage explicite du Option
                    let mut move_action: Option<(usize, usize, &str)> = None;

                    let bottom_bar_height = 40.0; 
                    let available_height = ui.available_height();

                    egui::ScrollArea::vertical()
                        .max_height(available_height - bottom_bar_height)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            
                            let mut visible_indices = Vec::new();
                            let mut current_collapsed_depth = None;
                            let mut current_hidden_depth = None;
                            
                            for i in (0..self.engine.layers.len()).rev() {
                                let layer = &self.engine.layers[i];
                                
                                if let Some(cd) = current_collapsed_depth {
                                    if layer.depth > cd { continue; } 
                                    else { current_collapsed_depth = None; }
                                }

                                if let Some(hd) = current_hidden_depth {
                                    if layer.depth <= hd { current_hidden_depth = None; }
                                }

                                visible_indices.push((i, current_hidden_depth.is_some()));
                                
                                if layer.layer_type == LayerType::Folder {
                                    if !layer.expanded && current_collapsed_depth.is_none() { 
                                        current_collapsed_depth = Some(layer.depth); 
                                    }
                                    if !layer.visible && current_hidden_depth.is_none() {
                                        current_hidden_depth = Some(layer.depth);
                                    }
                                }
                            }

                            for &(i, is_inherited_hidden) in &visible_indices {
                                let layer = &mut self.engine.layers[i];
                                
                                let is_active = self.active_layer == i && !layer.locked && (self.current_tool != Tool::Move || self.transform_active);
                                
                                let row_height = 26.0;

                                let (rect, response) = ui.allocate_exact_size(
                                    egui::vec2(ui.available_width(), row_height), 
                                    egui::Sense::click_and_drag()
                                );

                                if response.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::Default); }
                                if response.drag_started() && !layer.locked { self.dragging_layer = Some(i); }

                                let bg_color = if is_active { egui::Color32::from_rgb(60, 60, 60) } 
                                               else if response.hovered() { egui::Color32::from_rgb(45, 45, 45) } 
                                               else { egui::Color32::TRANSPARENT };
                                
                                ui.painter().rect_filled(rect, 4.0, bg_color);

                                ui.allocate_ui_at_rect(rect, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.add_space(8.0);
                                        let indentation = layer.depth as f32 * 20.0;
                                        ui.add_space(indentation);

                                        let arrow_width = 16.0;
                                        if layer.layer_type == LayerType::Folder {
                                            let arrow = if layer.expanded { "v" } else { ">" };
                                            let arrow_resp = ui.add_sized(
                                                [arrow_width, row_height], 
                                                egui::Label::new(egui::RichText::new(arrow).strong()).selectable(false).sense(egui::Sense::click())
                                            );
                                            if arrow_resp.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
                                            if arrow_resp.clicked() { layer.expanded = !layer.expanded; }
                                        } else {
                                            ui.add_sized([arrow_width, row_height], egui::Label::new("").selectable(false));
                                        }

                                        let is_visible = layer.visible; 
                                        if ui.add_sized([20.0, row_height], egui::SelectableLabel::new(is_visible, "👁")).on_hover_text("Visibilité").clicked() {
                                            layer.visible = !is_visible; needs_gpu_refresh = true; 
                                        }
                                        
                                        let is_locked = layer.locked;
                                        let lock_icon = if is_locked { "🔒" } else { "🔓" };
                                        
                                        if ui.add_sized([20.0, row_height], egui::SelectableLabel::new(is_locked, lock_icon)).on_hover_text("Verrouiller/Déverrouiller").clicked() {
                                            layer.locked = !is_locked;
                                            if layer.locked && self.active_layer == i {
                                                self.transform_active = false;
                                                self.transform_state = None;
                                            }
                                        }

                                        let icon = match layer.layer_type {
                                            LayerType::Folder => "📁",
                                            LayerType::Dynamic => "⚡",
                                            LayerType::Raster => "📄",
                                        };
                                        let mut is_renaming = false;

                                        if let Some((ren_idx, ref mut new_name)) = self.renaming_layer {
                                            if ren_idx == i {
                                                is_renaming = true;
                                                let res = ui.add_sized([ui.available_width() - 8.0, row_height], egui::TextEdit::singleline(new_name));
                                                res.request_focus();
                                                if res.lost_focus() || ui.input(|inp| inp.key_pressed(egui::Key::Enter)) {
                                                    layer.name = new_name.clone();
                                                    self.renaming_layer = None;
                                                }
                                            }
                                        }

                                        if !is_renaming {
                                            let mut text_color = if is_active { egui::Color32::WHITE } else { egui::Color32::LIGHT_GRAY };
                                            if !layer.visible || is_inherited_hidden { text_color = egui::Color32::from_gray(100); }
                                            let name_label = egui::RichText::new(format!("{} {}", icon, layer.name)).color(text_color);
                                            ui.add_sized([ui.available_width() - 8.0, row_height], egui::Label::new(name_label).selectable(false));
                                        }
                                    });
                                });

                                if response.clicked() && !layer.locked { 
                                    self.active_layer = i; 
                                    self.transform_active = true; 
                                    self.transform_state = None; 
                                }
                                if response.double_clicked() && !layer.locked { self.renaming_layer = Some((i, layer.name.clone())); }

                                response.context_menu(|ui| {
                                    if ui.button("✏ Renommer").clicked() { context_action = Some(("rename".to_string(), i, layer.depth)); ui.close_menu(); }
                                    if ui.button("📁 Nouveau Groupe").clicked() { context_action = Some(("new_folder".to_string(), i, layer.depth)); ui.close_menu(); }
                                    
                                    if layer.layer_type == LayerType::Dynamic {
                                        if ui.button("⚡ Éditer le Script (CFS)").clicked() { 
                                            context_action = Some(("open_cfs".to_string(), i, layer.depth)); 
                                            ui.close_menu(); 
                                        }
                                    }

                                    ui.separator();
                                    if ui.button("📄 Dupliquer").clicked() { context_action = Some(("duplicate".to_string(), i, layer.depth)); ui.close_menu(); }
                                    ui.separator();
                                    if ui.button("🗑 Supprimer").clicked() { context_action = Some(("delete".to_string(), i, layer.depth)); ui.close_menu(); }
                                });

                                if let Some(drag_idx) = self.dragging_layer {
                                    if response.hovered() && drag_idx != i {
                                        let pointer_y = ui.input(|inp| inp.pointer.hover_pos().unwrap().y);
                                        if layer.layer_type == LayerType::Folder && pointer_y > rect.top() + 6.0 && pointer_y < rect.bottom() - 6.0 {
                                            ui.painter().rect_stroke(rect, 2.0, egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 136, 0)));
                                            if ui.input(|inp| inp.pointer.any_released()) { move_action = Some((drag_idx, i, "inside")); }
                                        } else if pointer_y < rect.center().y {
                                            ui.painter().hline(rect.x_range(), rect.top(), egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 136, 0)));
                                            if ui.input(|inp| inp.pointer.any_released()) { move_action = Some((drag_idx, i, "above")); }
                                        } else {
                                            ui.painter().hline(rect.x_range(), rect.bottom(), egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 136, 0)));
                                            if ui.input(|inp| inp.pointer.any_released()) { move_action = Some((drag_idx, i, "below")); }
                                        }
                                    }
                                }
                            }
                            
                            let remaining_space = ui.available_rect_before_wrap();
                            if ui.interact(remaining_space, ui.id().with("empty_space"), egui::Sense::click()).clicked() {
                                self.transform_active = false;
                                self.transform_state = None;
                            }

                            if ui.input(|inp| inp.pointer.any_released()) { self.dragging_layer = None; }
                        });

                    ui.add_space(5.0);
                    ui.separator();
                    ui.columns(5, |cols| {
                        cols[0].vertical_centered_justified(|ui| {
                            if ui.button("➕").on_hover_text("Nouveau calque").clicked() {
                                let mut insert_idx = self.engine.layers.len();
                                let mut depth = 0;
                                if !self.engine.layers.is_empty() {
                                    let active = &self.engine.layers[self.active_layer];
                                    depth = active.depth;
                                    if active.layer_type == LayerType::Folder && active.expanded {
                                        depth += 1;
                                        insert_idx = self.active_layer;
                                    } else {
                                        insert_idx = self.active_layer + 1; 
                                    }
                                }
                                self.engine.insert_layer(insert_idx, &format!("Calque {}", self.engine.layers.len() + 1), depth);
                                self.active_layer = insert_idx;
                                needs_gpu_refresh = true;
                            }
                        });
                        cols[1].vertical_centered_justified(|ui| {
                            if ui.button("📁").on_hover_text("Nouveau groupe").clicked() {
                                let mut insert_idx = self.engine.layers.len();
                                let mut depth = 0;
                                if !self.engine.layers.is_empty() {
                                    let active = &self.engine.layers[self.active_layer];
                                    depth = active.depth;
                                    if active.layer_type == LayerType::Folder && active.expanded {
                                        depth += 1;
                                        insert_idx = self.active_layer;
                                    } else {
                                        insert_idx = self.active_layer + 1; 
                                    }
                                }
                                self.engine.insert_folder(insert_idx, &format!("Groupe {}", self.engine.layers.len() + 1), depth);
                                self.active_layer = insert_idx;
                                needs_gpu_refresh = true;
                            }
                        });
                        cols[2].vertical_centered_justified(|ui| {
                            if ui.button("📄").on_hover_text("Dupliquer").clicked() {
                                if self.active_layer < self.engine.layers.len() {
                                    let mut new_layer = self.engine.layers[self.active_layer].clone();
                                    new_layer.name = format!("{} (Copie)", new_layer.name);
                                    self.engine.layers.insert(self.active_layer + 1, new_layer);
                                    self.active_layer += 1;
                                    needs_gpu_refresh = true;
                                }
                            }
                        });
                        cols[3].vertical_centered_justified(|ui| {
                            if ui.button("⬲").on_hover_text("Fusionner").clicked() {} 
                        });
                        cols[4].vertical_centered_justified(|ui| {
                            if ui.button("🗑").on_hover_text("Supprimer").clicked() {
                                if self.engine.layers.len() > 1 {
                                    self.engine.layers.remove(self.active_layer);
                                    if self.active_layer >= self.engine.layers.len() { self.active_layer = self.engine.layers.len() - 1; }
                                    needs_gpu_refresh = true;
                                }
                            }
                        });
                    });

                    if let Some((action, i, depth)) = context_action {
                        match action.as_str() {
                            "rename" => { self.renaming_layer = Some((i, self.engine.layers[i].name.clone())); }
                            "new_folder" => { self.engine.insert_folder(i + 1, "Nouveau Groupe", depth); needs_gpu_refresh = true; }
                            "duplicate" => {
                                let mut new_layer = self.engine.layers[i].clone();
                                new_layer.name = format!("{} (Copie)", new_layer.name);
                                self.engine.layers.insert(i + 1, new_layer);
                                needs_gpu_refresh = true;
                            }
                            "delete" => {
                                if self.engine.layers.len() > 1 {
                                    self.engine.layers.remove(i);
                                    if self.active_layer >= self.engine.layers.len() { self.active_layer = self.engine.layers.len() - 1; }
                                    needs_gpu_refresh = true;
                                }
                            }
                            "open_cfs" => { 
                                if let Some(code) = &self.engine.layers[i].script {
                                    self.cfs_code = code.clone();
                                }
                                self.editing_cfs_index = Some(i);
                                self.cfs_preview_texture = None;
                                self.state = AppState::CfsEditor; 
                            } 
                            _ => {}
                        }
                    }

                    if let Some((from, to, position)) = move_action {
                        let mut item = self.engine.layers.remove(from);
                        let target = if from < to { to - 1 } else { to }; 
                        
                        if position == "inside" {
                            item.depth = self.engine.layers[target].depth + 1;
                            self.engine.layers.insert(target, item); 
                        } else if position == "above" {
                            item.depth = self.engine.layers[target].depth;
                            self.engine.layers.insert(target + 1, item); 
                        } else if position == "below" {
                            item.depth = self.engine.layers[target].depth;
                            self.engine.layers.insert(target, item); 
                        }
                        needs_gpu_refresh = true;
                    }
                });

                // ==============================================================================
                // LE CANEVAS DE DESSIN CENTRAL ET TRANSFORMATION
                // ==============================================================================
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        let layer_name = if self.engine.layers.is_empty() { "Aucun".to_string() } else { self.engine.layers[self.active_layer].name.clone() };
                        ui.label(format!("Outil : {:?} | Calque : {}", self.current_tool, layer_name));
                        ui.add_space(10.0);
                    });

                    let available_space = ui.available_size();
                    if self.zoom <= 0.0 {
                        let zoom_x = available_space.x / self.engine.width as f32;
                        let zoom_y = available_space.y / self.engine.height as f32;
                        self.zoom = zoom_x.min(zoom_y) * 0.9;
                        self.zoom = self.zoom.clamp(0.05, 5.0); 
                    }

                    egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                        let (zoom_delta, ctrl_down) = ui.input(|i| (i.raw_scroll_delta.y, i.modifiers.ctrl));
                        if ctrl_down && zoom_delta != 0.0 { self.zoom = (self.zoom + zoom_delta * 0.005).clamp(0.05, 5.0); }
                            
                        ui.centered_and_justified(|ui| { 
                            if let Some(texture) = &self.texture {
                                let current_width = self.engine.width as f32 * self.zoom;
                                let current_height = self.engine.height as f32 * self.zoom;
                                let image_widget = egui::Image::new(texture).fit_to_exact_size(egui::vec2(current_width, current_height)).sense(egui::Sense::click_and_drag());
                                let response = ui.add(image_widget);

                                let image_top_left_x = response.rect.center().x - (current_width / 2.0);
                                let image_top_left_y = response.rect.center().y - (current_height / 2.0);
                                let image_rect = egui::Rect::from_min_size(egui::pos2(image_top_left_x, image_top_left_y), egui::vec2(current_width, current_height));

                                let mut modified = false;

                                if self.current_tool == Tool::Move {
                                    
                                    let mut clicked_inside_current_bbox = false;
                                    let mut s_rect_expanded = egui::Rect::NOTHING;

                                    if let Some((_, _, _, ref _orig_bbox, ref mut current_bbox, ref mut _dragging_handle)) = self.transform_state {
                                        let s_min = egui::pos2(image_top_left_x + current_bbox.min.x * self.zoom, image_top_left_y + current_bbox.min.y * self.zoom);
                                        let s_max = egui::pos2(image_top_left_x + current_bbox.max.x * self.zoom, image_top_left_y + current_bbox.max.y * self.zoom);
                                        s_rect_expanded = egui::Rect::from_min_max(s_min, s_max).expand(12.0); 
                                    }

                                    if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                                        if s_rect_expanded.contains(pos) && self.transform_active {
                                            clicked_inside_current_bbox = true;
                                        }

                                        if (response.drag_started() || response.clicked()) && !clicked_inside_current_bbox {
                                            let mut found_layer = None;
                                            let mut current_hidden_depth = None;
                                            
                                            if image_rect.contains(pos) {
                                                let local_x = (pos.x - image_top_left_x) / self.zoom;
                                                let local_y = (pos.y - image_top_left_y) / self.zoom;
                                                let lx = local_x as usize;
                                                let ly = local_y as usize;

                                                if lx < self.engine.width && ly < self.engine.height {
                                                    for i in (0..self.engine.layers.len()).rev() {
                                                        let layer = &self.engine.layers[i];
                                                        if let Some(hd) = current_hidden_depth {
                                                            if layer.depth > hd { continue; } else { current_hidden_depth = None; }
                                                        }
                                                        if !layer.visible {
                                                            if layer.layer_type == LayerType::Folder { current_hidden_depth = Some(layer.depth); }
                                                            continue;
                                                        }
                                                        if layer.layer_type == LayerType::Folder || layer.locked { continue; } 
                                                        
                                                        if layer.pixels[ly * self.engine.width + lx].a > 0 {
                                                            found_layer = Some(i);
                                                            break;
                                                        }
                                                    }
                                                }
                                            }

                                            if let Some(idx) = found_layer {
                                                self.active_layer = idx;
                                                self.transform_active = true;
                                                
                                                if let Some(bbox) = crate::tools::transform::get_bounding_box(&self.engine, idx) {
                                                    self.transform_state = Some((
                                                        idx,
                                                        self.engine.layers[idx].pixels.clone(),
                                                        self.engine.layers[idx].overflow.clone(),
                                                        bbox,
                                                        bbox,
                                                        if response.drag_started() { Some(8) } else { None } 
                                                    ));
                                                } else {
                                                    self.transform_state = None;
                                                }
                                            } 
                                            else if response.clicked() || response.drag_started() {
                                                self.transform_active = false;
                                                self.transform_state = None;
                                            }
                                        }
                                    }

                                    if self.transform_active && !self.engine.layers.is_empty() && !self.engine.layers[self.active_layer].locked {
                                        let active_idx = self.active_layer;
                                        
                                        let init_needed = match self.transform_state {
                                            Some((idx, ..)) => idx != active_idx,
                                            None => true,
                                        };
                                        if init_needed {
                                            if let Some(bbox) = crate::tools::transform::get_bounding_box(&self.engine, active_idx) {
                                                self.transform_state = Some((
                                                    active_idx,
                                                    self.engine.layers[active_idx].pixels.clone(),
                                                    self.engine.layers[active_idx].overflow.clone(),
                                                    bbox,
                                                    bbox,
                                                    None
                                                ));
                                            } else {
                                                self.transform_state = None;
                                            }
                                        }

                                        let mut transform_args = None;
                                        
                                        if let Some((_, _, _, ref orig_bbox, ref mut current_bbox, ref mut dragging_handle)) = self.transform_state {
                                            
                                            let s_min = egui::pos2(image_top_left_x + current_bbox.min.x * self.zoom, image_top_left_y + current_bbox.min.y * self.zoom);
                                            let s_max = egui::pos2(image_top_left_x + current_bbox.max.x * self.zoom, image_top_left_y + current_bbox.max.y * self.zoom);
                                            let s_rect = egui::Rect::from_min_max(s_min, s_max);

                                            ui.painter().rect_stroke(s_rect, 0.0, egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 136, 0)));

                                            let handles = [
                                                (s_rect.left_top(), egui::CursorIcon::ResizeNwSe),
                                                (s_rect.center_top(), egui::CursorIcon::ResizeVertical),
                                                (s_rect.right_top(), egui::CursorIcon::ResizeNeSw),
                                                (s_rect.right_center(), egui::CursorIcon::ResizeHorizontal),
                                                (s_rect.right_bottom(), egui::CursorIcon::ResizeNwSe),
                                                (s_rect.center_bottom(), egui::CursorIcon::ResizeVertical),
                                                (s_rect.left_bottom(), egui::CursorIcon::ResizeNeSw),
                                                (s_rect.left_center(), egui::CursorIcon::ResizeHorizontal),
                                            ];

                                            let mut hovered_handle = None;
                                            if let Some(pos) = ui.input(|i| i.pointer.latest_pos()) {
                                                for (i, &(h_pos, cursor)) in handles.iter().enumerate() {
                                                    let h_rect = egui::Rect::from_center_size(h_pos, egui::vec2(10.0, 10.0));
                                                    ui.painter().rect_filled(h_rect, 0.0, egui::Color32::BLACK);
                                                    ui.painter().rect_stroke(h_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::WHITE)); 
                                                    
                                                    if h_rect.contains(pos) {
                                                        hovered_handle = Some(i);
                                                        ui.ctx().set_cursor_icon(cursor);
                                                    }
                                                }
                                                if hovered_handle.is_none() && s_rect.contains(pos) {
                                                    ui.ctx().set_cursor_icon(egui::CursorIcon::AllScroll);
                                                } else if hovered_handle.is_none() {
                                                    for &(h_pos, _) in &handles {
                                                        let h_rect = egui::Rect::from_center_size(h_pos, egui::vec2(10.0, 10.0));
                                                        ui.painter().rect_filled(h_rect, 0.0, egui::Color32::BLACK);
                                                        ui.painter().rect_stroke(h_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
                                                    }
                                                }
                                            } else {
                                                for &(h_pos, _) in &handles {
                                                    let h_rect = egui::Rect::from_center_size(h_pos, egui::vec2(10.0, 10.0));
                                                    ui.painter().rect_filled(h_rect, 0.0, egui::Color32::BLACK);
                                                    ui.painter().rect_stroke(h_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
                                                }
                                            }

                                            if response.drag_started() {
                                                if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                                                    if let Some(idx) = hovered_handle {
                                                        *dragging_handle = Some(idx);
                                                    } else if s_rect.contains(pos) {
                                                        *dragging_handle = Some(8);
                                                    }
                                                }
                                            }

                                            if response.dragged() {
                                                if let Some(handle_idx) = *dragging_handle {
                                                    let delta = ui.input(|i| i.pointer.delta());
                                                    let c_delta = egui::vec2(delta.x / self.zoom, delta.y / self.zoom);
                                                    
                                                    match handle_idx {
                                                        0 => { current_bbox.min += c_delta; },
                                                        1 => { current_bbox.min.y += c_delta.y; },
                                                        2 => { current_bbox.max.x += c_delta.x; current_bbox.min.y += c_delta.y; },
                                                        3 => { current_bbox.max.x += c_delta.x; },
                                                        4 => { current_bbox.max += c_delta; },
                                                        5 => { current_bbox.max.y += c_delta.y; },
                                                        6 => { current_bbox.min.x += c_delta.x; current_bbox.max.y += c_delta.y; },
                                                        7 => { current_bbox.min.x += c_delta.x; },
                                                        8 => { *current_bbox = current_bbox.translate(c_delta); },
                                                        _ => {}
                                                    }

                                                    if current_bbox.max.x - current_bbox.min.x < 1.0 {
                                                        if handle_idx == 0 || handle_idx == 6 || handle_idx == 7 { current_bbox.min.x = current_bbox.max.x - 1.0; }
                                                        else { current_bbox.max.x = current_bbox.min.x + 1.0; }
                                                    }
                                                    if current_bbox.max.y - current_bbox.min.y < 1.0 {
                                                        if handle_idx == 0 || handle_idx == 1 || handle_idx == 2 { current_bbox.min.y = current_bbox.max.y - 1.0; }
                                                        else { current_bbox.max.y = current_bbox.min.y + 1.0; }
                                                    }

                                                    transform_args = Some((orig_bbox.clone(), current_bbox.clone()));
                                                }
                                            }
                                        }

                                        if let Some((obbox, cbbox)) = transform_args {
                                            let original_backup = self.transform_state.as_ref().unwrap().1.clone();
                                            let original_overflow = self.transform_state.as_ref().unwrap().2.clone(); 
                                            if crate::tools::transform::apply_transform_absolute(&mut self.engine, self.active_layer, &original_backup, &original_overflow, obbox, cbbox) {
                                                modified = true;
                                            }
                                        }

                                        if ui.input(|i| i.pointer.any_released()) {
                                            if let Some(ref mut state) = self.transform_state {
                                                state.5 = None; 
                                            }
                                        }
                                    }
                                } 
                                else {
                                    self.transform_state = None;

                                    if let Some(pos) = ui.input(|i| i.pointer.latest_pos()) {
                                        if image_rect.contains(pos) || response.dragged() {
                                            if matches!(self.current_tool, Tool::Brush | Tool::Eraser) {
                                                let screen_radius = (self.brush_size / 2.0) * self.zoom;
                                                ui.painter().circle_stroke(pos, screen_radius, egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 200)));
                                                ui.painter().circle_stroke(pos, screen_radius + 1.0, egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 150)));
                                                ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
                                            }
                                        }
                                    }

                                    let active_depth = self.engine.layers[self.active_layer].depth;
                                    let mut parent_is_hidden = false;
                                    let mut current_depth = active_depth;
                                    
                                    for i in self.active_layer + 1 .. self.engine.layers.len() {
                                        let l = &self.engine.layers[i];
                                        if l.layer_type == LayerType::Folder && l.depth < current_depth {
                                            if !l.visible { parent_is_hidden = true; break; }
                                            current_depth = l.depth;
                                            if current_depth == 0 { break; }
                                        }
                                    }

                                    let can_draw = !self.engine.layers.is_empty() 
                                        && self.engine.layers[self.active_layer].visible 
                                        && !parent_is_hidden 
                                        && !self.engine.layers[self.active_layer].locked 
                                        && self.engine.layers[self.active_layer].layer_type == LayerType::Raster;

                                    if response.dragged() || response.clicked() {
                                        if can_draw {
                                            if let Some(p_pos) = response.interact_pointer_pos() {
                                                let local_x = (p_pos.x - image_top_left_x) / self.zoom;
                                                let local_y = (p_pos.y - image_top_left_y) / self.zoom;
                                                let (last_x, last_y) = self.last_draw_pos.unwrap_or((local_x, local_y));
                                                
                                                if self.current_tool == Tool::Brush {
                                                    if crate::tools::brush::apply(&mut self.engine, self.active_layer, last_x, last_y, local_x, local_y, self.brush_size, self.brush_hardness, self.brush_opacity, self.brush_flow, self.primary_color) {
                                                        modified = true;
                                                    }
                                                } else if self.current_tool == Tool::Eraser {
                                                    if crate::tools::eraser::apply(&mut self.engine, self.active_layer, last_x, last_y, local_x, local_y, self.brush_size, self.brush_hardness, self.brush_opacity, self.brush_flow) {
                                                        modified = true;
                                                    }
                                                }
                                                self.last_draw_pos = Some((local_x, local_y));
                                            }
                                        }
                                    } else { self.last_draw_pos = None; }
                                }

                                if modified { needs_gpu_refresh = true; }
                            }
                        });
                    });
                });

                if needs_gpu_refresh {
                    self.refresh_gpu_texture(ctx);
                    ctx.request_repaint();
                }

            } 
        } 
    }
}

fn add_tool_button(ui: &mut egui::Ui, current_tool: &mut Tool, tool_variant: Tool, icon: egui::ImageSource<'_>, tooltip: &str) {
    let is_selected = *current_tool == tool_variant;
    let icon_color = if is_selected { egui::Color32::BLACK } else { egui::Color32::WHITE };
    let img = egui::Image::new(icon).fit_to_exact_size(egui::vec2(24.0, 24.0)).tint(icon_color);
    if ui.add(egui::ImageButton::new(img).selected(is_selected)).on_hover_text(tooltip).clicked() { *current_tool = tool_variant; }
    ui.add_space(5.0);
}
