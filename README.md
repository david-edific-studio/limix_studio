# Limix Studio 🎨

Limix Studio est une alternative native à Photoshop conçue spécifiquement pour l'écosystème Linux (avec support Windows). Développé en Rust, ce logiciel vise la performance absolue, la stabilité, et la réduction de la charge cognitive grâce à une interface minimaliste et familière.

Projet propulsé par l'initiative **Edific World**.

---

## ⚙️ 1. Installation & Prérequis

Le projet requiert **Git** et l'environnement **Rust** pour être compilé.

### 🐧 Utilisateurs Linux (Arch / Debian / Ubuntu)
1. Installer les dépendances de base :
   * Arch : `sudo pacman -S git base-devel`
   * Debian/Ubuntu : `sudo apt update && sudo apt install git build-essential`
2. Installer Rust :
   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### 🪟 Utilisateurs Windows
1. Installer **Git** via [git-scm.com](https://git-scm.com/).
2. Installer les **Visual Studio Build Tools** (indispensable pour la compilation C++ sous Windows).
3. Installer **Rust** via le fichier `rustup-init.exe` disponible sur [rustup.rs](https://rustup.rs/).

---

## 🚀 2. Lancement du Projet

Une fois les prérequis installés, ouvre ton terminal et exécute :

```bash
# Récupérer le code source
git clone https://github.com/david-edific-studio/limix_studio.git

# Entrer dans le répertoire
cd limix_studio

# Compiler et lancer le logiciel
cargo run




# Procedure push
git add .
git commit -m "Description claire de ce qui a été ajouté ou modifié"
git push origin nom-de-ta-branche

# Si c'est la Branche principale, on fais 
git push origin main