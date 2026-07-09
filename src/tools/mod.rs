// --- DÉCLARATION DES FICHIERS PHYSIQUES ---
pub mod brush;
pub mod eraser;
pub mod selection;
pub mod transform;
pub mod vector;
pub mod pencil;
pub mod fill;
pub mod gradient;
pub mod retouch;
pub mod crop;
pub mod magic_wand;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tool {
    // 1. Déplacement & Recadrage
    Move, 
    Crop,
    
    // 2. Sélection & Découpage
    SelectionRect, 
    SelectionEllipse, 
    LassoFree, 
    LassoPoly, 
    LassoMagnetic, 
    MagicWand, 
    SelectQuick, 
    RemoveBg,
    
    // 3. Peinture & Dessin
    Brush, 
    Pencil, 
    Eraser, 
    Fill, 
    Gradient,
    
    // 4. Retouche & Couleurs
    CloneStamp, 
    HealingBrush, 
    Eyedropper,
    Sharpen, 
    Blur, 
    Smudge, 
    Burn, 
    Dodge, 
    Sponge,
    
    // 5. Vectoriel & Texte
    Pen, 
    Shapes, 
    Text,
}
