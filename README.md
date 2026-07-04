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

1. Installer **Git** via git-scm.com.
2. Installer les **Visual Studio Build Tools** (indispensable pour la compilation C++ sous Windows).
3. Installer **Rust** via le fichier `rustup-init.exe` disponible sur rustup.rs.

---

## 🚀 2. Lancement & Commandes de Base

Une fois les prérequis installés, ouvre ton terminal et exécute :

### Récupérer et lancer le projet

* Cloner le code source : `git clone https://github.com/david-edific-studio/limix_studio.git`
* Entrer dans le répertoire : `cd limix_studio`
* Compiler et lancer le logiciel : `cargo run`

### Procédure rapide pour envoyer son travail (Push)

* Préparer et sceller les fichiers :
`git add .`
`git commit -m "Description claire de ce qui a été ajouté ou modifié"`
* Envoyer sur sa branche de travail : `git push origin nom-de-ta-branche`

---

## 🌿 3. Règle d'Or : Le Travail par Branche

La branche **main** représente le tronc principal de l'arbre : c'est le code stable qui doit toujours fonctionner. **Il est strictement interdit de coder directement sur le main ou d'y envoyer du code sans validation.** Pour chaque modification, le protocole suivant doit être appliqué à la lettre :

### Étape 1 : Créer un espace isolé (Une branche)

Avant de modifier le moindre fichier, on crée une déviation par rapport au tronc principal :
`git checkout -b nom-de-ta-fonctionnalite`

### Étape 2 : Sauvegarder localement

Une fois le travail terminé ou mis en pause, enregistre tes modifications sur ta branche :
`git add .`
`git commit -m "Description précise du travail effectué"`

### Étape 3 : Propulser la branche sur GitHub

Envoie ton espace de travail en ligne pour qu'il soit visible par l'équipe :
`git push origin nom-de-ta-fonctionnalite`

### Étape 4 : Se mettre d'accord et Fusionner

1. Rends-toi sur GitHub.
2. Ouvre une **Pull Request** (demande de fusion).
3. **Discuter et valider ensemble :** Aucun développeur ne fusionne son propre code. C'est l'autre membre de l'équipe qui doit relire le code et valider la fusion vers le tronc principal.

---

## 🛡️ 4. Protocole Anti-Conflit

* **Séparation de l'architecture :** Ne travaillez jamais dans le même fichier en même temps. Séparez la logique en modules distincts (ex: un fichier pour l'interface graphique, un fichier pour les calculs d'images).
* **Mise à jour constante :** Avant de créer une nouvelle branche, récupère toujours les dernières nouveautés validées sur le tronc principal :
`git checkout main`
`git pull origin main`