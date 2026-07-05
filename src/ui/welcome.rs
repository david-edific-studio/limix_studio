use eframe::egui;
use crate::ui::{LimixApp, AppState};
use crate::core::canvas::{Canvas, Rgba};

pub fn show(app: &mut LimixApp, ctx: &egui::Context) {
    
    // -------------------------------------------------------------
    // 1. LE TIROIR DES PARAMÈTRES (Glisse depuis la gauche)
    // -------------------------------------------------------------
    if app.show_settings {
        egui::SidePanel::left("settings_drawer")
            .exact_width(250.0)
            .show(ctx, |ui| {
                ui.add_space(20.0);
                ui.heading("Paramètres Limix");
                ui.separator();
                ui.label("Thème de l'interface :");
                ui.radio_value(&mut 1, 1, "Sombre (Par défaut)");
                ui.radio_value(&mut 1, 2, "Clair");
                ui.add_space(10.0);
                ui.label("Performances :");
                let mut dummy = true;
                ui.checkbox(&mut dummy, "Accélération matérielle");
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(20.0);
                    if ui.button("Fermer").clicked() {
                        app.show_settings = false;
                    }
                });
            });
    }

    // -------------------------------------------------------------
    // 2. LA FENÊTRE MODALE "NOUVEAU PROJET" (Paramétrique Optimisée)
    // -------------------------------------------------------------
    if app.new_proj_params.show_modal {
        egui::Window::new("Nouveau Projet")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // Fixé parfaitement au centre de l'écran
            .show(ctx, |ui| {
                ui.add_space(10.0);
                
                // --- OPTIMISATION : UTILISATION D'UNE GRILLE POUR UN ALIGNEMENT PARFAIT ---
                egui::Grid::new("new_project_grid")
                    .num_columns(2)
                    .spacing([40.0, 15.0]) // Espacement aéré entre les colonnes et les lignes
                    .show(ui, |ui| {
                        
                        // DIMENSIONS
                        ui.label(egui::RichText::new("Largeur :").strong());
                        ui.add(egui::DragValue::new(&mut app.new_proj_params.width).suffix(" px").speed(10));
                        ui.end_row();

                        ui.label(egui::RichText::new("Hauteur :").strong());
                        ui.add(egui::DragValue::new(&mut app.new_proj_params.height).suffix(" px").speed(10));
                        ui.end_row();

                        // ORIENTATION (Bascule automatique largeur/hauteur)
                        ui.label(egui::RichText::new("Orientation :").strong());
                        ui.horizontal(|ui| {
                            if ui.selectable_label(app.new_proj_params.orientation == 0, "Paysage ▬").clicked() {
                                app.new_proj_params.orientation = 0;
                                if app.new_proj_params.height > app.new_proj_params.width {
                                    std::mem::swap(&mut app.new_proj_params.width, &mut app.new_proj_params.height);
                                }
                            }
                            if ui.selectable_label(app.new_proj_params.orientation == 1, "Portrait ▮").clicked() {
                                app.new_proj_params.orientation = 1;
                                if app.new_proj_params.width > app.new_proj_params.height {
                                    std::mem::swap(&mut app.new_proj_params.width, &mut app.new_proj_params.height);
                                }
                            }
                        });
                        ui.end_row();
                        
                        // TYPE / PROFONDEUR DE COULEUR
                        ui.label(egui::RichText::new("Couleurs :").strong());
                        egui::ComboBox::from_id_source("bit_depth")
                            .selected_text(format!("{} bits", app.new_proj_params.bits))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut app.new_proj_params.bits, 8, "8 bits (Standard)");
                                ui.selectable_value(&mut app.new_proj_params.bits, 16, "16 bits (Pro)");
                                ui.selectable_value(&mut app.new_proj_params.bits, 32, "32 bits (HDR)");
                            });
                        ui.end_row();

                        // COULEUR D'ARRIÈRE PLAN
                        ui.label(egui::RichText::new("Arrière-plan :").strong());
                        ui.color_edit_button_rgb(&mut app.new_proj_params.bg_color);
                        ui.end_row();
                    });
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);
                
                // BOUTONS D'ACTION (Alignés à droite pour un style plus "Système d'exploitation")
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    
                    // Le bouton "Commencer" avec la charte graphique Orange Limix
                    let btn_start = ui.add_sized(
                        [150.0, 35.0], 
                        egui::Button::new(egui::RichText::new("Commencer !").size(16.0).color(egui::Color32::WHITE))
                        .fill(egui::Color32::from_rgb(255, 136, 0))
                    );

                    if btn_start.clicked() {
                        // 1. On génère la NOUVELLE zone de dessin avec les vraies valeurs !
                        app.engine = Canvas::new(app.new_proj_params.width, app.new_proj_params.height);
                        app.engine.add_layer("Arrière-plan");
                        app.engine.add_layer("Tracé Principal");
                        
                        // 2. On convertit la couleur d'arrière-plan choisie
                        let r = (app.new_proj_params.bg_color[0] * 255.0) as u8;
                        let g = (app.new_proj_params.bg_color[1] * 255.0) as u8;
                        let b = (app.new_proj_params.bg_color[2] * 255.0) as u8;
                        
                        // 3. On remplit le fond
                        for p in app.engine.layers[0].pixels.iter_mut() {
                            *p = Rgba { r, g, b, a: 255 };
                        }
                        
                        // 4. On lance l'éditeur !
                        app.texture = None; // Force le moteur graphique à recalculer l'image
                        app.new_proj_params.show_modal = false; // Ferme la fenêtre
                        app.state = AppState::Workspace; // Bascule la machine à états !
                        
                        // ASTUCE : On met le zoom à 0.0 au lieu de 1.0. 
                        // L'espace de travail comprendra qu'il doit calculer le "Auto-Fit" !
                        app.zoom = 0.0;
                    }

                    ui.add_space(10.0); // Espace entre les deux boutons

                    if ui.add_sized([100.0, 35.0], egui::Button::new("Annuler")).clicked() {
                        app.new_proj_params.show_modal = false;
                    }
                });
                ui.add_space(5.0);
            });
    }

    // -------------------------------------------------------------
    // 3. LA PAGE D'ACCUEIL (Le fond)
    // -------------------------------------------------------------
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(150.0);
            ui.heading(egui::RichText::new("Limix Studio").size(60.0).color(egui::Color32::from_rgb(255, 136, 0)));
            ui.add_space(10.0);
            ui.label(egui::RichText::new("Création et Retouche Native.").size(20.0));
            ui.add_space(80.0);

            let btn_size = egui::vec2(250.0, 60.0);
            if ui.add_sized(btn_size, egui::Button::new(egui::RichText::new("Nouveau Projet").size(22.0))).clicked() {
                // Au lieu de lancer direct le logiciel, ça ouvre la fenêtre paramétrique !
                app.new_proj_params.show_modal = true; 
            }
            ui.add_space(20.0);
            if ui.add_sized(btn_size, egui::Button::new(egui::RichText::new("Paramètres").size(22.0))).clicked() {
                app.show_settings = !app.show_settings; 
            }
        });
    });
}
