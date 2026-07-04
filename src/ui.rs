use eframe::egui;
use crate::canvas::{Canvas, Rgba};

pub struct LimixApp {
    pub engine: Canvas,
    pub texture: Option<egui::TextureHandle>,
}

impl LimixApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut engine = Canvas::new(800, 600);
        engine.add_layer("Arrière-plan");
        engine.add_layer("Tracé Principal");

        // Remplissage de l'arrière-plan (gris sombre)
        for p in engine.layers[0].pixels.iter_mut() {
            *p = Rgba { r: 40, g: 40, b: 40, a: 255 };
        }

        // Dessin du carré bleu de test
        for y in 100..300 {
            for x in 100..300 {
                let index = y * 800 + x;
                engine.layers[1].pixels[index] = Rgba { r: 0, g: 150, b: 255, a: 255 };
            }
        }

        Self {
            engine,
            texture: None,
        }
    }
}

impl eframe::App for LimixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // --- 1. BARRE DE MENU SUPÉRIEURE ---
        egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Fichier", |ui| {
                    if ui.button("Nouveau").clicked() { /* Action future */ }
                    if ui.button("Exporter PNG").clicked() { /* Action future */ }
                    ui.separator();
                    if ui.button("Quitter").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                });
                ui.menu_button("Édition", |ui| {
                    if ui.button("Annuler (Ctrl+Z)").clicked() { /* Action future */ }
                });
            });
        });

        // --- 2. BARRE D'OUTILS (GAUCHE) ---
        egui::SidePanel::left("toolbar")
            .resizable(false)
            .exact_width(45.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    // Placeholders temporaires ultra-minimalistes
                    if ui.button("🖌").on_hover_text("Pinceau (B)").clicked() {}
                    ui.add_space(5.0);
                    if ui.button("🧽").on_hover_text("Gomme (E)").clicked() {}
                    ui.add_space(5.0);
                    if ui.button("⬚").on_hover_text("Sélection (M)").clicked() {}
                });
            });

        // --- 3. PANNEAU DES CALQUES (DROITE) ---
        egui::SidePanel::right("layers_panel")
            .resizable(true)
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Calques");
                ui.separator();
                
                // On boucle sur les calques à l'envers pour afficher le calque supérieur en haut de la liste
                for layer in self.engine.layers.iter().rev() {
                    ui.horizontal(|ui| {
                        // Un faux bouton "œil" pour la visibilité (à connecter plus tard)
                        let mut visible = true; 
                        ui.checkbox(&mut visible, "");
                        ui.label(&layer.name);
                    });
                }
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    if ui.button("+ Nouveau Calque").clicked() {
                        // L'action d'ajout de calque sera câblée ici
                    }
                });
            });

        // --- 4. ESPACE DE TRAVAIL (CENTRE) ---
        // Le CentralPanel doit toujours être déclaré en dernier
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                if ui.button("Calculer le rendu de la Toile").clicked() {
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

                    self.texture = Some(ctx.load_texture(
                        "canvas_render",
                        color_image,
                        egui::TextureOptions::LINEAR
                    ));
                }

                ui.add_space(20.0);

                // On affiche l'image centrée dans l'espace disponible
                if let Some(texture) = &self.texture {
                    ui.image(texture);
                }
            });
        });
    }
}
