use eframe::egui;
use crate::core::canvas::{Canvas, Rgba};
use crate::tools::Tool;

pub struct LimixApp {
    pub engine: Canvas,
    pub texture: Option<egui::TextureHandle>,
    
    pub current_tool: Tool,
    pub active_layer: usize,
}

impl LimixApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 1. Initialiser le chargeur d'images (Indispensable pour lire les SVG)
        egui_extras::install_image_loaders(&cc.egui_ctx);
        
        // --- LE THÈME LIMIX STUDIO ---
        let mut visuals = egui::Visuals::dark(); // On part sur la base sombre
        visuals.selection.bg_fill = egui::Color32::from_rgb(255, 136, 0); // On remplace le bleu par l'Orange Limix
        cc.egui_ctx.set_visuals(visuals);
        // -----------------------------

        let mut engine = Canvas::new(800, 600);
        engine.add_layer("Arrière-plan");
        engine.add_layer("Tracé Principal");

        // Remplissage du fond (Feuille blanche par défaut)
        for p in engine.layers[0].pixels.iter_mut() {
            *p = Rgba { r: 255, g: 255, b: 255, a: 255 }; 
        }

        Self {
            engine,
            texture: None,
            current_tool: Tool::Brush,
            active_layer: 1, // On dessine par défaut sur 'Tracé Principal'
        }
    }

    // --- LE MOTEUR DYNAMIQUE ---
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
        
        // Initialisation automatique au premier lancement
        if self.texture.is_none() {
            self.refresh_gpu_texture(ctx);
        }

        egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Fichier", |ui| {
                    if ui.button("Nouveau").clicked() {}
                    if ui.button("Exporter PNG").clicked() {}
                    ui.separator();
                    if ui.button("Quitter").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                });
                ui.menu_button("Édition", |_ui| {});
            });
        });

        egui::SidePanel::left("toolbar").resizable(false).exact_width(45.0).show(ctx, |ui| {
            // On ajoute un ScrollArea : indispensable pour afficher 27 outils sans déborder de l'écran !
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);

                    // --- 1. Déplacement & Recadrage ---
                    add_tool_button(ui, &mut self.current_tool, Tool::Move, egui::include_image!("../../assets/icons/move.svg"), "Déplacement");
                    add_tool_button(ui, &mut self.current_tool, Tool::Crop, egui::include_image!("../../assets/icons/crop.svg"), "Recadrage");
                    ui.separator();

                    // --- 2. Sélection & Découpage ---
                    add_tool_button(ui, &mut self.current_tool, Tool::SelectionRect, egui::include_image!("../../assets/icons/select_rect.svg"), "Sélection Rectangulaire");
                    add_tool_button(ui, &mut self.current_tool, Tool::SelectionEllipse, egui::include_image!("../../assets/icons/select_ellipse.svg"), "Sélection Elliptique");
                    add_tool_button(ui, &mut self.current_tool, Tool::LassoFree, egui::include_image!("../../assets/icons/lasso_free.svg"), "Lasso");
                    add_tool_button(ui, &mut self.current_tool, Tool::LassoPoly, egui::include_image!("../../assets/icons/lasso_poly.svg"), "Lasso Polygonal");
                    add_tool_button(ui, &mut self.current_tool, Tool::LassoMagnetic, egui::include_image!("../../assets/icons/lasso_magnetic.svg"), "Lasso Magnétique");
                    add_tool_button(ui, &mut self.current_tool, Tool::MagicWand, egui::include_image!("../../assets/icons/magic_wand.svg"), "Baguette Magique");
                    add_tool_button(ui, &mut self.current_tool, Tool::SelectQuick, egui::include_image!("../../assets/icons/select_quick.svg"), "Sélection Rapide");
                    add_tool_button(ui, &mut self.current_tool, Tool::RemoveBg, egui::include_image!("../../assets/icons/remove_bg.svg"), "Supprimer le fond");
                    ui.separator();

                    // --- 3. Peinture & Dessin ---
                    add_tool_button(ui, &mut self.current_tool, Tool::Brush, egui::include_image!("../../assets/icons/brush.svg"), "Pinceau");
                    add_tool_button(ui, &mut self.current_tool, Tool::Pencil, egui::include_image!("../../assets/icons/pencil.svg"), "Crayon");
                    add_tool_button(ui, &mut self.current_tool, Tool::Eraser, egui::include_image!("../../assets/icons/eraser.svg"), "Gomme");
                    add_tool_button(ui, &mut self.current_tool, Tool::Fill, egui::include_image!("../../assets/icons/fill.svg"), "Pot de peinture");
                    add_tool_button(ui, &mut self.current_tool, Tool::Gradient, egui::include_image!("../../assets/icons/gradient.svg"), "Dégradé");
                    ui.separator();

                    // --- 4. Retouche & Couleurs ---
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

                    // --- 5. Vectoriel & Texte ---
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
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(format!("Outil : {:?} | Calque : {}", self.current_tool, self.engine.layers[self.active_layer].name));
                ui.add_space(10.0);

                if let Some(texture) = &self.texture {
                    let image_widget = egui::Image::new(texture).sense(egui::Sense::click_and_drag());
                    let response = ui.add(image_widget);

                    if response.dragged() || response.clicked() {
                        if let Some(pointer_pos) = response.interact_pointer_pos() {
                            
                            let local_x = pointer_pos.x - response.rect.min.x;
                            let local_y = pointer_pos.y - response.rect.min.y;

                            if local_x >= 0.0 && local_x < self.engine.width as f32 &&
                               local_y >= 0.0 && local_y < self.engine.height as f32 {

                                let cx = local_x as usize;
                                let cy = local_y as usize;
                                let mut modified = false;
                                let brush_radius = 2; 

                                if self.current_tool == Tool::Brush {
                                    if crate::tools::brush::apply(&mut self.engine, self.active_layer, cx, cy, brush_radius) {
                                        modified = true;
                                    }
                                } else if self.current_tool == Tool::Eraser {
                                    if crate::tools::eraser::apply(&mut self.engine, self.active_layer, cx, cy, brush_radius) {
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
    // Si sélectionné (Fond Orange) -> Icône Noire
    // Si inactif (Fond Sombre)     -> Icône Blanche
    let icon_color = if is_selected {
        egui::Color32::BLACK // Tu peux aussi utiliser from_rgb(30, 30, 30) pour un noir plus doux
    } else {
        egui::Color32::WHITE
    };

    let img = egui::Image::new(icon).max_width(24.0).tint(icon_color);
    
    // On laisse egui gérer ses propres fonds de boutons proprement
    if ui.add(egui::ImageButton::new(img).selected(is_selected))
        .on_hover_text(tooltip)
        .clicked()
    {
        *current_tool = tool_variant;
    }
    ui.add_space(5.0);
}
