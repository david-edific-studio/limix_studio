use eframe::egui;
use crate::ui::{LimixApp, AppState};
use crate::core::canvas::{Canvas, Rgba};

pub fn show(app: &mut LimixApp, ctx: &egui::Context) {
    
    // --- CONFIGURATION DE TA CHARTE GRAPHIQUE ---
    let bg_color = egui::Color32::from_rgb(14, 18, 22);         
    let panel_color = egui::Color32::from_rgb(29, 36, 44);      
    let _panel_hover = egui::Color32::from_rgb(35, 42, 50);      
    let border_color = egui::Color32::from_rgb(71, 80, 90);     
    
    let btn_color = egui::Color32::from_rgb(217, 90, 38);       
    let btn_hover = egui::Color32::from_rgb(250, 181, 99);      
    
    let text_light = egui::Color32::from_rgb(240, 245, 250);
    let text_gray = egui::Color32::from_rgb(140, 150, 160);
    
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = bg_color;
    visuals.window_fill = panel_color;
    visuals.widgets.noninteractive.bg_fill = panel_color;
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, border_color);
    
    visuals.widgets.inactive.bg_fill = btn_color;
    visuals.widgets.hovered.bg_fill = btn_hover;
    visuals.widgets.active.bg_fill = btn_color;
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_light);
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
    
    ctx.set_visuals(visuals);

    // -------------------------------------------------------------
    // 1. LE TIROIR DES PARAMÈTRES
    // -------------------------------------------------------------
    if app.show_settings {
        egui::SidePanel::left("settings_drawer")
            .exact_width(250.0)
            .frame(egui::Frame::default().fill(panel_color).inner_margin(15.0))
            .show(ctx, |ui| {
                ui.add_space(20.0);
                ui.heading(egui::RichText::new("Paramètres Limix").color(text_light));
                ui.separator();
                ui.label(egui::RichText::new("Thème de l'interface :").color(text_gray));
                ui.radio_value(&mut 1, 1, "Sombre (Par défaut)");
                ui.add_space(10.0);
                let mut dummy = true;
                ui.checkbox(&mut dummy, "Accélération matérielle");
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(20.0);
                    if ui.add_sized([150.0, 35.0], egui::Button::new("Fermer")).clicked() {
                        app.show_settings = false;
                    }
                });
            });
    }

    // -------------------------------------------------------------
    // 2. LA FENÊTRE MODALE "NOUVEAU PROJET"
    // -------------------------------------------------------------
    if app.new_proj_params.show_modal {
        egui::Window::new("Nouveau Projet")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(egui::Frame::window(&ctx.style()).fill(panel_color).inner_margin(20.0))
            .show(ctx, |ui| {
                ui.add_space(10.0);
                egui::Grid::new("new_project_grid").num_columns(2).spacing([40.0, 15.0]).show(ui, |ui| {
                    ui.label(egui::RichText::new("Largeur :").strong().color(text_light));
                    ui.add(egui::DragValue::new(&mut app.new_proj_params.width).suffix(" px").speed(10));
                    ui.end_row();

                    ui.label(egui::RichText::new("Hauteur :").strong().color(text_light));
                    ui.add(egui::DragValue::new(&mut app.new_proj_params.height).suffix(" px").speed(10));
                    ui.end_row();

                    ui.label(egui::RichText::new("Orientation :").strong().color(text_light));
                    ui.horizontal(|ui| {
                        if ui.selectable_label(app.new_proj_params.orientation == 0, "Paysage ▬").clicked() {
                            app.new_proj_params.orientation = 0;
                            if app.new_proj_params.height > app.new_proj_params.width { std::mem::swap(&mut app.new_proj_params.width, &mut app.new_proj_params.height); }
                        }
                        if ui.selectable_label(app.new_proj_params.orientation == 1, "Portrait ▮").clicked() {
                            app.new_proj_params.orientation = 1;
                            if app.new_proj_params.width > app.new_proj_params.height { std::mem::swap(&mut app.new_proj_params.width, &mut app.new_proj_params.height); }
                        }
                    });
                    ui.end_row();
                    
                    ui.label(egui::RichText::new("Couleurs :").strong().color(text_light));
                    egui::ComboBox::from_id_source("bit_depth")
                        .selected_text(format!("{} bits", app.new_proj_params.bits))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.new_proj_params.bits, 8, "8 bits (Standard)");
                            ui.selectable_value(&mut app.new_proj_params.bits, 16, "16 bits (Pro)");
                        });
                    ui.end_row();

                    ui.label(egui::RichText::new("Arrière-plan :").strong().color(text_light));
                    ui.color_edit_button_rgb(&mut app.new_proj_params.bg_color);
                    ui.end_row();
                });
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add_sized([150.0, 35.0], egui::Button::new(egui::RichText::new("Commencer !").size(15.0).strong())).clicked() {
                        app.engine = Canvas::new(app.new_proj_params.width, app.new_proj_params.height);
                        app.engine.add_layer("Arrière-plan", 0);
                        app.engine.add_layer("Tracé Principal", 0);
                        
                        let r = (app.new_proj_params.bg_color[0] * 255.0) as u8;
                        let g = (app.new_proj_params.bg_color[1] * 255.0) as u8;
                        let b = (app.new_proj_params.bg_color[2] * 255.0) as u8;
                        for p in app.engine.layers[0].pixels.iter_mut() { *p = Rgba { r, g, b, a: 255 }; }
                        
                        app.texture = None; 
                        app.new_proj_params.show_modal = false;
                        app.state = AppState::Workspace; 
                        app.zoom = 0.0;
                    }
                    ui.add_space(10.0);
                    if ui.add_sized([100.0, 35.0], egui::Button::new("Annuler")).clicked() { app.new_proj_params.show_modal = false; }
                });
            });
    }

    // -------------------------------------------------------------
    // 3. LA PAGE D'ACCUEIL PRINCIPALE (DASHBOARD RESTRUCTURÉ)
    // -------------------------------------------------------------
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            let main_frame = egui::Frame::default().fill(bg_color).inner_margin(egui::Margin::symmetric(40.0, 30.0));

            main_frame.show(ui, |ui| {

                // =========================================================
                // HEADER (TOP)
                // =========================================================
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add(egui::Image::new(egui::include_image!("../../assets/logo_tc.png")).fit_to_exact_size(egui::vec2(40.0, 40.0)));
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("Limix Pro").size(24.0).strong().color(text_light));
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add_sized([40.0, 40.0], egui::Button::new(egui::RichText::new("☰").size(20.0))).clicked() {
                            app.show_settings = !app.show_settings;
                        }
                        ui.add_space(10.0);
                        if ui.add_sized([40.0, 40.0], egui::Button::new(egui::RichText::new("⊞").size(20.0))).clicked() {}
                    });
                });

                ui.add_space(40.0);

                let main_content_w = 950.0;
                let main_margin = (ui.available_width() - main_content_w).max(0.0) / 2.0;

                // =========================================================
                // SECTION 1 : LE BANDEAU D'ACTION (CREATE NEW PROJECT)
                // =========================================================
                ui.horizontal(|ui| {
                    ui.add_space(main_margin); 
                    ui.vertical(|ui| {
                        ui.set_width(main_content_w); 

                        let banner_frame = egui::Frame::none()
                            .fill(panel_color)
                            .rounding(16.0)
                            .stroke(egui::Stroke::new(1.0, border_color))
                            .inner_margin(egui::Margin::symmetric(40.0, 25.0));

                        banner_frame.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    ui.add(egui::Image::new(egui::include_image!("../../assets/logo_tc.png")).fit_to_exact_size(egui::vec2(80.0, 80.0)));
                                    ui.add_space(20.0);
                                    
                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new("Create New Project").size(24.0).strong().color(text_light));
                                        ui.add_space(2.0);
                                        ui.label(egui::RichText::new("Créer une nouvelle création").size(15.0).color(text_gray));
                                    });
                                });

                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    ui.add_space(40.0);
                                    let (line_rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width() - 340.0, 1.0), egui::Sense::hover());
                                    ui.painter().rect_filled(line_rect, 0.0, border_color);
                                });

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    
                                    let draw_big_btn = |ui: &mut egui::Ui, icon: &str, text: &str| -> bool {
                                        let (rect, resp) = ui.allocate_exact_size(egui::vec2(85.0, 85.0), egui::Sense::click());
                                        let current_bg = if resp.hovered() { btn_hover } else { btn_color };
                                        
                                        ui.painter().rect_filled(rect, 12.0, current_bg);
                                        ui.painter().rect_stroke(rect, 12.0, egui::Stroke::new(1.0, current_bg));

                                        ui.allocate_ui_at_rect(rect, |ui| {
                                            ui.vertical_centered(|ui| {
                                                ui.add_space(18.0);
                                                ui.label(egui::RichText::new(icon).size(26.0).color(egui::Color32::WHITE));
                                                ui.add_space(4.0);
                                                ui.label(egui::RichText::new(text).size(13.0).color(egui::Color32::WHITE).strong());
                                            });
                                        });
                                        resp.clicked()
                                    };

                                    if draw_big_btn(ui, "🎵", "Audio") {}
                                    ui.add_space(15.0);
                                    if draw_big_btn(ui, "✒", "Vector") {}
                                    ui.add_space(15.0);
                                    if draw_big_btn(ui, "🖼", "Image") {
                                        app.new_proj_params.show_modal = true;
                                    }
                                });
                            });
                        });
                    });
                });

                ui.add_space(40.0);

                // --- LIGNE SÉPARATRICE CENTRALE ---
                let line_rect = ui.allocate_space(egui::vec2(ui.available_width(), 1.0)).1;
                ui.painter().rect_filled(line_rect, 0.0, border_color);

                ui.add_space(40.0);

                // =========================================================
                // SECTION 2 : LE DASHBOARD (2 Colonnes Propres)
                // =========================================================
                ui.horizontal(|ui| {
                    ui.add_space(main_margin); 
                    
                    let col_spacing = 50.0;
                    let col_w = (main_content_w - col_spacing) / 2.0;

                    // --- COLONNE GAUCHE (Activité Récente & Projets Épinglés) ---
                    ui.vertical(|ui| {
                        ui.set_width(col_w);
                        ui.label(egui::RichText::new("Recent Activity").size(16.0).color(text_light).strong());
                        ui.add_space(15.0);

                        // 1. La Grande Carte 3D
                        let bc_frame = egui::Frame::none()
                            .fill(panel_color)
                            .rounding(16.0)
                            .stroke(egui::Stroke::new(1.0, border_color))
                            .inner_margin(egui::Margin::same(25.0));

                        bc_frame.show(ui, |ui| {
                            ui.set_min_height(250.0);
                            ui.set_width(col_w - 50.0);

                            ui.vertical_centered(|ui| {
                                ui.add_space(40.0);
                                ui.label(egui::RichText::new("🧊").size(80.0));
                            });

                            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new("By y 1 2522").size(12.0).color(text_gray));
                                    ui.add_space(2.0);
                                    ui.label(egui::RichText::new("3D project").size(16.0).strong().color(text_light));
                                });
                            });
                        });

                        ui.add_space(30.0);

                        // 2. Les Projets Épinglés empilés en dessous
                        ui.label(egui::RichText::new("Pinned Projects").size(16.0).color(text_light).strong());
                        ui.add_space(15.0);

                        let draw_pinned_card = |ui: &mut egui::Ui, title: &str, subtitle: &str| {
                            let frame = egui::Frame::none()
                                .fill(panel_color)
                                .rounding(10.0)
                                .stroke(egui::Stroke::new(1.0, border_color))
                                .inner_margin(egui::Margin::same(12.0));
                            
                            let response = frame.show(ui, |ui| {
                                ui.set_width(col_w - 24.0); // Prend toute la largeur de la colonne gauche
                                ui.horizontal(|ui| {
                                    let img_frame = egui::Frame::none().fill(bg_color).rounding(6.0).inner_margin(egui::Margin::same(18.0));
                                    img_frame.show(ui, |ui| { ui.label(" "); });
                                    
                                    ui.add_space(15.0);
                                    ui.vertical(|ui| {
                                        ui.add_space(2.0);
                                        ui.label(egui::RichText::new(title).color(text_light).size(13.0).strong());
                                        ui.add_space(2.0);
                                        ui.label(egui::RichText::new(subtitle).color(text_gray).size(11.0));
                                    });
                                });
                            }).response;

                            if response.interact(egui::Sense::click()).hovered() {} 
                        };

                        draw_pinned_card(ui, "Créer une nouvelle...", "By y 1234");
                        ui.add_space(15.0);
                        draw_pinned_card(ui, "M 2D pad prd pad", "By y 1590");
                    });

                    ui.add_space(col_spacing);

                    // --- COLONNE DROITE (La Timeline Connectée) ---
                    ui.vertical(|ui| {
                        ui.set_width(col_w);
                        ui.label(egui::RichText::new("Timeline").size(16.0).color(text_light).strong());
                        ui.add_space(15.0);

                        let timeline_x = ui.cursor().min.x + 15.0; 
                        let start_y = ui.cursor().min.y;
                        let mut last_y = start_y;

                        let mut draw_history_item = |ui: &mut egui::Ui, time: &str, title: &str, subtitle: &str| {
                            ui.horizontal(|ui| {
                                ui.add_space(35.0);
                                ui.label(egui::RichText::new(time).size(13.0).color(text_gray).strong());
                            });
                            ui.add_space(5.0);

                            ui.horizontal(|ui| {
                                ui.add_space(35.0);
                                
                                let card_frame = egui::Frame::none()
                                    .fill(panel_color)
                                    .rounding(10.0)
                                    .stroke(egui::Stroke::new(1.0, border_color))
                                    .inner_margin(egui::Margin::symmetric(15.0, 12.0));

                                let resp = card_frame.show(ui, |ui| {
                                    ui.set_width(col_w - 65.0); // Prend l'espace restant
                                    ui.horizontal(|ui| {
                                        let img_frame = egui::Frame::none().fill(bg_color).rounding(6.0).inner_margin(egui::Margin::same(18.0));
                                        img_frame.show(ui, |ui| { ui.label(" "); });
                                        
                                        ui.add_space(15.0);
                                        ui.vertical(|ui| {
                                            ui.add_space(2.0);
                                            ui.label(egui::RichText::new(title).size(14.0).strong().color(text_light));
                                            ui.add_space(2.0);
                                            ui.label(egui::RichText::new(subtitle).size(11.0).color(text_gray));
                                        });
                                    });
                                }).response;

                                let dot_pos = egui::pos2(timeline_x, resp.rect.center().y);
                                ui.painter().circle_filled(dot_pos, 4.0, btn_color); 
                                ui.painter().hline(timeline_x..=resp.rect.min.x, dot_pos.y, egui::Stroke::new(2.0, border_color)); 
                            });
                            
                            ui.add_space(20.0);
                            last_y = ui.cursor().min.y; 
                        };

                        draw_history_item(ui, "Today", "Minimulics d 2D print...", "By y 1 2022");
                        draw_history_item(ui, "Yesterday", "Inlags ef lMleed", "By r 1 2522");
                        draw_history_item(ui, "Last Week", "Audios wontmatoh bml...", "By x 1 2022");
                        draw_history_item(ui, "Last Month", "Stroct Jus Breading", "By 140022");

                        ui.painter().vline(timeline_x, start_y..=last_y - 50.0, egui::Stroke::new(2.0, border_color));
                    });
                });
            });
        });
    });
}
