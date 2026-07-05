use eframe::egui;
use crate::core::canvas::{Canvas, Rgba};

#[derive(PartialEq, Debug)]
pub enum Tool {
    Brush,
    Eraser,
    Selection,
}

pub struct LimixApp {
    pub engine: Canvas,
    pub texture: Option<egui::TextureHandle>,
    
    pub current_tool: Tool,
    pub active_layer: usize,
}

impl LimixApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut engine = Canvas::new(800, 600);
        engine.add_layer("Arrière-plan");
        engine.add_layer("Tracé Principal");

        // Remplissage du fond
        for p in engine.layers[0].pixels.iter_mut() {
            *p = Rgba { r: 40, g: 40, b: 40, a: 255 };
        }

        Self {
            engine,
            texture: None,
            current_tool: Tool::Brush,
            active_layer: 1, // On dessine par défaut sur 'Tracé Principal'
        }
    }

    // --- LE MOTEUR DYNAMIQUE ---
    // Cette fonction aspire la RAM du CPU et l'envoie au GPU instantanément
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
                ui.menu_button("Édition", |ui| {});
            });
        });

        egui::SidePanel::left("toolbar").resizable(false).exact_width(45.0).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.selectable_value(&mut self.current_tool, Tool::Brush, "🖌").on_hover_text("Pinceau Rouge");
                ui.add_space(5.0);
                ui.selectable_value(&mut self.current_tool, Tool::Eraser, "🧽").on_hover_text("Gomme");
                ui.add_space(5.0);
                ui.selectable_value(&mut self.current_tool, Tool::Selection, "⬚").on_hover_text("Sélection");
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
                    // 1. LES CAPTEURS EGUI
                    // On transforme l'image statique en zone sensible au clic et au glissement
                    let image_widget = egui::Image::new(texture).sense(egui::Sense::click_and_drag());
                    let response = ui.add(image_widget);

                    // 2. LA DÉTECTION DU MOUVEMENT
                    if response.dragged() || response.clicked() {
                        if let Some(pointer_pos) = response.interact_pointer_pos() {
                            
                            // 3. LA MATHÉMATIQUE SPATIALE (Écran -> Moteur)
                            let local_x = pointer_pos.x - response.rect.min.x;
                            let local_y = pointer_pos.y - response.rect.min.y;

                            // Vérification : La souris est-elle bien dans la zone de l'image ?
                            if local_x >= 0.0 && local_x < self.engine.width as f32 &&
                               local_y >= 0.0 && local_y < self.engine.height as f32 {

                                let cx = local_x as usize;
                                let cy = local_y as usize;
                                let mut modified = false;

                                // 4. APPLICATION DE L'ÉPAISSEUR DU PINCEAU (5x5 pixels)
                                let brush_radius = 2; 

                                for dy in -brush_radius..=brush_radius {
                                    for dx in -brush_radius..=brush_radius {
                                        let px = cx as isize + dx;
                                        let py = cy as isize + dy;

                                        // On s'assure que le pinceau ne déborde pas des limites de l'image
                                        if px >= 0 && px < self.engine.width as isize &&
                                           py >= 0 && py < self.engine.height as isize {
                                            
                                            // Indexation linéaire pure
                                            let index = (py as usize) * self.engine.width + (px as usize);
                                            
                                            if self.current_tool == Tool::Brush {
                                                // Injection d'un pixel ROUGE OPAQUE
                                                self.engine.layers[self.active_layer].pixels[index] = Rgba { r: 255, g: 50, b: 50, a: 255 };
                                                modified = true;
                                            } else if self.current_tool == Tool::Eraser {
                                                // GOMME : Injection d'un pixel TRANSPARENT (Alpha = 0)
                                                self.engine.layers[self.active_layer].pixels[index] = Rgba { r: 0, g: 0, b: 0, a: 0 };
                                                modified = true;
                                            }
                                        }
                                    }
                                }

                                // 5. LA BOUCLE TEMPS RÉEL : On recalcule instantanément si on a dessiné
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
