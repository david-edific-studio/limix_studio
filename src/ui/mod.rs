pub mod welcome;

use eframe::egui;
use crate::core::canvas::{Canvas, Rgba};
use crate::tools::Tool;

#[derive(PartialEq)]
pub enum AppState {
    WelcomeScreen, // La page d'accueil moderne
    Workspace,     // L'éditeur de dessin
}

// --- LA MÉMOIRE DES PARAMÈTRES AVANT CRÉATION ---
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
        Self {
            width: 1920,
            height: 1080,
            bits: 8,
            bg_color: [1.0, 1.0, 1.0], 
            orientation: 0,
            show_modal: false,
        }
    }
}

pub struct LimixApp {
    pub engine: Canvas,
    pub texture: Option<egui::TextureHandle>,
    pub current_tool: Tool,
    pub active_layer: usize,
    pub brush_size: f32,
    pub zoom: f32,
    
    pub state: AppState,       
    pub show_settings: bool,   
    pub new_proj_params: NewProjectParams, 
}

impl LimixApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut visuals = egui::Visuals::dark();
        visuals.selection.bg_fill = egui::Color32::from_rgb(255, 136, 0);
        cc.egui_ctx.set_visuals(visuals);

        let engine = Canvas::new(800, 600);

        Self {
            engine,
            texture: None,
            current_tool: Tool::Brush,
            active_layer: 1,
            brush_size: 10.0,
            zoom: 1.0,
            
            state: AppState::WelcomeScreen,
            show_settings: false,
            new_proj_params: NewProjectParams::default(),
        }
    }

    fn refresh_gpu_texture(&mut self, ctx: &egui::Context) {
        let resultat = self.engine.render_flattened();
        let mut raw_pixels = Vec::with_capacity(resultat.len() * 4);
        for p in resultat {
            raw_pixels.push(p.r);
            raw_pixels.push(p.g);
            raw_pixels.push(p.b);
            raw_pixels.push(p.a);
        }
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [self.engine.width, self.engine.height],
            &raw_pixels,
        );
        self.texture = Some(ctx.load_texture("canvas_render", color_image, egui::TextureOptions::LINEAR));
    }
}

impl eframe::App for LimixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        if self.texture.is_none() {
            self.refresh_gpu_texture(ctx);
        }
        
        match self.state {

            // =========================================================================================
            // ÉTAT 1 : LA PAGE D'ACCUEIL
            // =========================================================================================
            AppState::WelcomeScreen => {
                crate::ui::welcome::show(self, ctx);
            }

            // =========================================================================================
            // ÉTAT 2 : L'ESPACE DE TRAVAIL
            // =========================================================================================
            AppState::Workspace => {

                egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
                    ui.add_space(2.0); 
                    ui.horizontal(|ui| {
                        
                        if ui.button("🏠 Accueil").clicked() {
                            self.state = AppState::WelcomeScreen;
                        }
                        ui.separator();
                        
                        ui.menu_button("Fichier", |ui| {
                            if ui.button("Nouveau...").clicked() {
                                self.new_proj_params.show_modal = true;
                                self.state = AppState::WelcomeScreen; 
                            }
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
                            if ui.button("Quitter").clicked() { 
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close); 
                            }
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
                            if ui.button("Style de calque...").clicked() {}
                            if ui.button("Masque de calque...").clicked() {}
                        });

                        ui.menu_button("Sélection", |ui| {
                            if ui.button("Tout sélectionner").clicked() {}
                            if ui.button("Désélectionner").clicked() {}
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
                        ui.add_space(10.0);
                        ui.strong(format!("{:?}", self.current_tool));
                        ui.separator();
                        
                        match self.current_tool {
                            Tool::Brush | Tool::Pencil | Tool::Eraser => {
                                ui.label("Taille :");
                                ui.add(egui::Slider::new(&mut self.brush_size, 1.0..=100.0).text("px"));
                                ui.separator();
                                ui.label("Opacité :");
                                let mut dummy_opacity = 100.0;
                                ui.add(egui::Slider::new(&mut dummy_opacity, 0.0..=100.0).text("%"));
                            }
                            Tool::Text => {
                                ui.label("Police :");
                                if ui.button("Arial").clicked() {}
                                ui.label("Taille :");
                                ui.add(egui::Slider::new(&mut self.brush_size, 8.0..=144.0).text("pt"));
                            }
                            _ => {
                                ui.label("Prêt.");
                            }
                        }
                    });
                });

                egui::TopBottomPanel::bottom("status_bar").exact_height(28.0).show(ctx, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.add_space(10.0);
                        ui.label("Zoom :");
                        
                        if ui.button("➖").clicked() {
                            self.zoom -= 0.1;
                            if self.zoom < 0.1 { self.zoom = 0.1; }
                        }
                        
                        ui.label(format!("{} %", (self.zoom * 100.0).round()));
                        
                        if ui.button("➕").clicked() {
                            self.zoom += 0.1;
                            if self.zoom > 5.0 { self.zoom = 5.0; }
                        }
                        
                        ui.separator();
                        ui.label(format!("Document : {} x {} px", self.engine.width, self.engine.height));
                    });
                });

                egui::SidePanel::left("toolbar").resizable(false).exact_width(65.0).show(ctx, |ui| {
                    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(10.0);
                            add_tool_button(ui, &mut self.current_tool, Tool::Move, egui::include_image!("../../assets/icons/move.svg"), "Déplacement");
                            add_tool_button(ui, &mut self.current_tool, Tool::Crop, egui::include_image!("../../assets/icons/crop.svg"), "Recadrage");
                            ui.separator();
                            add_tool_button(ui, &mut self.current_tool, Tool::SelectionRect, egui::include_image!("../../assets/icons/select_rect.svg"), "Sélection Rectangulaire");
                            add_tool_button(ui, &mut self.current_tool, Tool::SelectionEllipse, egui::include_image!("../../assets/icons/select_ellipse.svg"), "Sélection Elliptique");
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
                            add_tool_button(ui, &mut self.current_tool, Tool::CloneStamp, egui::include_image!("../../assets/icons/clone_stamp.svg"), "Tampon de duplication");
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

                egui::SidePanel::right("layers_panel").resizable(true).min_width(200.0).show(ctx, |ui| {
                    ui.heading("Calques");
                    ui.separator();
                    
                    let layer_count = self.engine.layers.len();
                    for i in (0..layer_count).rev() {
                        let layer = &self.engine.layers[i];
                        ui.horizontal(|ui| {
                            let mut visible = true; 
                            ui.checkbox(&mut visible, "");
                            ui.selectable_value(&mut self.active_layer, i, &layer.name);
                        });
                    }
                    
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        if ui.button("+ Nouveau Calque").clicked() {
                            self.engine.add_layer(&format!("Calque {}", layer_count + 1));
                            self.active_layer = layer_count;
                        }
                    });
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    // L'étiquette de l'outil et du calque reste en haut, fixée
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.label(format!("Outil : {:?} | Calque : {}", self.current_tool, self.engine.layers[self.active_layer].name));
                        ui.add_space(10.0);
                    });

                    // ---> LE CALCUL MAGIQUE DE CADRAGE (Auto-Fit) <---
                    let available_space = ui.available_size();
                    
                    if self.zoom <= 0.0 {
                        let zoom_x = available_space.x / self.engine.width as f32;
                        let zoom_y = available_space.y / self.engine.height as f32;
                        
                        // On prend le plus petit pour que tout rentre, et on laisse 10% de marge (0.9)
                        self.zoom = zoom_x.min(zoom_y) * 0.9;
                        self.zoom = self.zoom.clamp(0.05, 5.0); 
                    }

                    // On enferme la zone de dessin dans une boîte avec Scrollbars
                    egui::ScrollArea::both()
                        .auto_shrink([false, false]) 
                        .show(ui, |ui| {
                            
                        // --- RÉTABLISSEMENT DU ZOOM MOLETTE ICI ! ---
                        let (zoom_delta, ctrl_down) = ui.input(|i| (i.raw_scroll_delta.y, i.modifiers.ctrl));
                        if ctrl_down && zoom_delta != 0.0 {
                            self.zoom += zoom_delta * 0.005; 
                            self.zoom = self.zoom.clamp(0.05, 5.0); 
                        }
                            
                        // ---> LE CENTRAGE ABSOLU DE LA FEUILLE <---
                        ui.centered_and_justified(|ui| { 
                            if let Some(texture) = &self.texture {
                                // LA TOILE RÉAGIT AU ZOOM ET A SA NOUVELLE VRAIE TAILLE
                                let current_width = self.engine.width as f32 * self.zoom;
                                let current_height = self.engine.height as f32 * self.zoom;

                                let image_widget = egui::Image::new(texture)
                                    .fit_to_exact_size(egui::vec2(current_width, current_height))
                                    .sense(egui::Sense::click_and_drag());

                                let response = ui.add(image_widget);

                                // --- TA LOGIQUE DE DESSIN INTACTE ---
                                if response.dragged() || response.clicked() {
                                    if let Some(pointer_pos) = response.interact_pointer_pos() {
                                        
                                        // Calcul corrigé pour le Zoom
                                        let local_x = (pointer_pos.x - response.rect.min.x) / self.zoom;
                                        let local_y = (pointer_pos.y - response.rect.min.y) / self.zoom;

                                        if local_x >= 0.0 && local_x < self.engine.width as f32 &&
                                           local_y >= 0.0 && local_y < self.engine.height as f32 {

                                            let cx = local_x as usize;
                                            let cy = local_y as usize;
                                            let mut modified = false;
                                            
                                            // LA CORRECTION FATALE : isize au lieu de usize !
                                            let current_radius = self.brush_size as isize; 

                                            if self.current_tool == Tool::Brush {
                                                if crate::tools::brush::apply(&mut self.engine, self.active_layer, cx, cy, current_radius) {
                                                    modified = true;
                                                }
                                            } else if self.current_tool == Tool::Eraser {
                                                if crate::tools::eraser::apply(&mut self.engine, self.active_layer, cx, cy, current_radius) {
                                                    modified = true;
                                                }
                                            }

                                            if modified {
                                                self.refresh_gpu_texture(ctx);
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    });
                });
            } // Fin de AppState::Workspace
        } // Fin du match state
    }
}

// Notre usine à boutons (Version Propre et Contraste Parfait)
fn add_tool_button(
    ui: &mut egui::Ui,
    current_tool: &mut Tool,
    tool_variant: Tool,
    icon: egui::ImageSource<'_>,
    tooltip: &str,
) {
    let is_selected = *current_tool == tool_variant;

    // --- LA MAGIE DU CONTRASTE ---
    let icon_color = if is_selected {
        egui::Color32::BLACK 
    } else {
        egui::Color32::WHITE
    };

    // On force une taille mathématique stricte (24px sur 24px)
    let img = egui::Image::new(icon)
        .fit_to_exact_size(egui::vec2(24.0, 24.0)) 
        .tint(icon_color);
    
    // On laisse egui gérer ses propres fonds de boutons proprement
    if ui.add(egui::ImageButton::new(img).selected(is_selected))
        .on_hover_text(tooltip)
        .clicked()
    {
        *current_tool = tool_variant;
    }
    ui.add_space(5.0);
}
