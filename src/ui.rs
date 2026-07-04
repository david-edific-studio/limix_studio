use eframe::egui;

// La structure qui contiendra l'état de notre interface
#[derive(Default)]
pub struct LimixApp;

impl LimixApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // C'est ici qu'on pourra charger des polices personnalisées ou un thème sombre plus tard
        Self::default()
    }
}

// Implémentation du trait App d'eframe (la boucle de rendu)
impl eframe::App for LimixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Un panneau central sobre et propre
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Limix Studio 🎨");
            ui.separator();
            ui.label("Bienvenue dans l'espace de travail natif.");
            ui.add_space(10.0);
            
            if ui.button("Simuler un calcul de calques").clicked() {
                println!("Clic détecté : Le moteur de rendu sera bientôt connecté ici !");
            }
        });
    }
}
