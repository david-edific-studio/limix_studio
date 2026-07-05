pub mod brush;
pub mod eraser;

// On déplace l'énumération ici, c'est sa vraie place
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Tool {
    Brush,
    Eraser,
    Selection,
}