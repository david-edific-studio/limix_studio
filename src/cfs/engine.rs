use boa_engine::{Context, Source};
use crate::core::canvas::Rgba;

/// Fonction qui prend le code JS, l'exécute dans Boa, et retourne un tableau de pixels
pub fn execute_cfs(code: &str, width: usize, height: usize) -> Result<Vec<Rgba>, String> {
    
    // On prépare un calque vide
    let mut pixels = vec![Rgba { r: 0, g: 0, b: 0, a: 0 }; width * height];

    // LE PONT MAGIQUE : On simule l'API Canvas HTML5 en pur JS !
    // Il va transformer les commandes JS en une chaîne de caractères lisible par Rust.
    let wrapper = format!(r#"
        const __COMMANDS__ = [];
        const ctx = {{
            fillStyle: "rgba(255, 255, 255, 1.0)",
            fillRect: function(x, y, w, h) {{
                let c = this.fillStyle.replace('rgba(', '').replace(')', '').split(',');
                let r = parseInt(c[0] || 0);
                let g = parseInt(c[1] || 0);
                let b = parseInt(c[2] || 0);
                let a = parseInt((parseFloat(c[3] || 1.0) * 255));
                __COMMANDS__.push(`rect|${{Math.floor(x)}}|${{Math.floor(y)}}|${{Math.floor(w)}}|${{Math.floor(h)}}|${{r}}|${{g}}|${{b}}|${{a}}`);
            }}
        }};

        try {{
            {} // <-- TON CODE JAVASCRIPT EST INJECTÉ ICI
            
            if (typeof render === 'function') {{
                render(ctx);
            }}
        }} catch(e) {{
            __COMMANDS__.push("error|" + e.toString());
        }}
        
        __COMMANDS__.join(';')
    "#, code);

    // Initialisation du moteur V8/V9 (Boa)
    let mut context = Context::default();
    let source = Source::from_bytes(wrapper.as_bytes());
    
    // Exécution du code dans la Sandbox
    let result = match context.eval(source) {
        Ok(res) => res,
        Err(e) => return Err(format!("Erreur syntaxique fatale: {}", e)),
    };

    // On récupère le résultat sous forme de texte
    let result_str = result.to_string(&mut context).map_err(|e| e.to_string())?.to_std_string_escaped();

    // S'il n'y a pas de commandes, on retourne juste l'image vide
    if result_str.is_empty() { 
        return Ok(pixels); 
    }

    // INTERPRÉTATION PAR RUST (On peint les pixels à la vitesse de la lumière)
    for cmd_str in result_str.split(';') {
        let parts: Vec<&str> = cmd_str.split('|').collect();
        if parts.is_empty() { continue; }

        match parts[0] {
            "rect" => {
                if parts.len() == 9 {
                    let rx: isize = parts[1].parse().unwrap_or(0);
                    let ry: isize = parts[2].parse().unwrap_or(0);
                    let rw: isize = parts[3].parse().unwrap_or(0);
                    let rh: isize = parts[4].parse().unwrap_or(0);
                    let r: u8 = parts[5].parse().unwrap_or(255);
                    let g: u8 = parts[6].parse().unwrap_or(255);
                    let b: u8 = parts[7].parse().unwrap_or(255);
                    let a: u8 = parts[8].parse().unwrap_or(255);

                    // Peinture manuelle dans le tableau de pixels
                    for y in ry..ry+rh {
                        for x in rx..rx+rw {
                            if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                                let idx = (y as usize) * width + (x as usize);
                                pixels[idx] = Rgba { r, g, b, a };
                            }
                        }
                    }
                }
            },
            "error" => {
                return Err(parts[1..].join("|"));
            },
            _ => {}
        }
    }

    Ok(pixels)
}
