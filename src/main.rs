use rand::Rng;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

// Types de Pokémon
#[derive(Debug, Clone, PartialEq)]
enum PokemonType {
    Feu,
    Eau,
    Plante,
    Electrik,
    Normal,
    Vol,
    Combat,
    Psy,
}

impl PokemonType {
    fn as_str(&self) -> &str {
        match self {
            Self::Feu => "Feu",
            Self::Eau => "Eau",
            Self::Plante => "Plante",
            Self::Electrik => "Électrik",
            Self::Normal => "Normal",
            Self::Vol => "Vol",
            Self::Combat => "Combat",
            Self::Psy => "Psy",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "feu" => Some(Self::Feu),
            "eau" => Some(Self::Eau),
            "plante" => Some(Self::Plante),
            "electrik" | "électrik" => Some(Self::Electrik),
            "normal" => Some(Self::Normal),
            "vol" => Some(Self::Vol),
            "combat" => Some(Self::Combat),
            "psy" => Some(Self::Psy),
            _ => None,
        }
    }

    fn get_all_types() -> Vec<Self> {
        vec![
            Self::Feu,
            Self::Eau,
            Self::Plante,
            Self::Electrik,
            Self::Normal,
            Self::Vol,
            Self::Combat,
            Self::Psy,
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Genre {
    Male,
    Femelle,
}

impl Genre {
    fn as_str(&self) -> &str {
        match self {
            Self::Male => "Mâle",
            Self::Femelle => "Femelle",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "male" | "mâle" | "m" => Some(Self::Male),
            "femelle" | "f" => Some(Self::Femelle),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Pokemon {
    nom: String,
    niveau: u32,
    pokemon_type: PokemonType,
    experience: u32,
    genre: Genre,
}

impl Pokemon {
    fn new(nom: String, pokemon_type: PokemonType, genre: Genre) -> Self {
        Self {
            nom,
            niveau: 1,
            pokemon_type,
            experience: 0,
            genre,
        }
    }

    fn from_details(
        nom: String,
        niveau: u32,
        pokemon_type: PokemonType,
        experience: u32,
        genre: Genre,
    ) -> Self {
        Self {
            nom,
            niveau,
            pokemon_type,
            experience,
            genre,
        }
    }

    fn gagner_experience(&mut self, xp: u32) {
        self.experience += xp;
        let niveau_avant = self.niveau;

        while self.experience >= 100 {
            self.niveau += 1;
            self.experience -= 100;
        }

        if self.niveau > niveau_avant {
            println!(
                "🎉 {} passe du niveau {} au niveau {}!",
                self.nom, niveau_avant, self.niveau
            );
        }
    }

    fn peut_se_reproduire(&self, autre: &Pokemon) -> bool {
        self.pokemon_type == autre.pokemon_type
            && self.genre != autre.genre
            && self.niveau >= 5
            && autre.niveau >= 5
    }

    fn to_string_format(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}",
            self.nom,
            self.niveau,
            self.pokemon_type.as_str(),
            self.experience,
            self.genre.as_str()
        )
    }

    fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 5 {
            return None;
        }

        let nom = parts[0].to_string();
        let niveau = parts[1].parse::<u32>().ok()?;
        let pokemon_type = PokemonType::from_str(parts[2])?;
        let experience = parts[3].parse::<u32>().ok()?;
        let genre = Genre::from_str(parts[4])?;

        Some(Self::from_details(
            nom,
            niveau,
            pokemon_type,
            experience,
            genre,
        ))
    }
}

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Pokémon: {} | Type: {} | Niveau: {} | XP: {}/100 | Genre: {}",
            self.nom,
            self.pokemon_type.as_str(),
            self.niveau,
            self.experience,
            self.genre.as_str()
        )
    }
}

struct Elevage {
    pokemons: Vec<Pokemon>,
    nom_elevage: String,
}

impl Elevage {
    fn new(nom: &str) -> Self {
        Self {
            pokemons: Vec::new(),
            nom_elevage: nom.to_string(),
        }
    }

    fn ajouter_pokemon(&mut self, pokemon: Pokemon) -> usize {
        println!("✅ Ajout du Pokémon: {}", pokemon);
        self.pokemons.push(pokemon);
        self.pokemons.len() - 1
    }

    fn afficher_tous_les_pokemons(&self) {
        if self.pokemons.is_empty() {
            println!("⚠️ Votre élevage est vide ! Ajoutez des Pokémon.");
            return;
        }

        println!(
            "\n=== Liste des Pokémon de l'élevage {} ({}) ===",
            self.nom_elevage,
            self.pokemons.len()
        );
        for (i, pokemon) in self.pokemons.iter().enumerate() {
            println!("#{}: {}", i + 1, pokemon);
        }
        println!("===================================\n");
    }

    fn entrainer_pokemon(&mut self, index: usize, xp: u32) -> bool {
        if index >= self.pokemons.len() {
            return false;
        }

        println!(
            "🏋️ Entraînement de {} (+{} XP)...",
            self.pokemons[index].nom, xp
        );
        self.pokemons[index].gagner_experience(xp);
        true
    }

    fn entrainer_tous_les_pokemons(&mut self, xp: u32) {
        println!("🏋️ Entraînement de tous les Pokémon (+{} XP)...", xp);
        for pokemon in &mut self.pokemons {
            pokemon.gagner_experience(xp);
        }
    }

    fn tenter_reproduction(&mut self, index1: usize, index2: usize) -> bool {
        if index1 >= self.pokemons.len() || index2 >= self.pokemons.len() || index1 == index2 {
            println!("❌ Indices non valides pour la reproduction");
            return false;
        }

        let pokemon1 = self.pokemons[index1].clone();
        let pokemon2 = self.pokemons[index2].clone();

        if pokemon1.peut_se_reproduire(&pokemon2) {
            let mut rng = rand::thread_rng();
            let genre = if rng.gen_bool(0.5) {
                Genre::Male
            } else {
                Genre::Femelle
            };

            let noms = ["Mystère", "Junior", "Petit", "Mini", "Pousse"];
            let nom = format!(
                "{} {}",
                noms[rng.gen_range(0..noms.len())],
                &pokemon1.nom[0..std::cmp::min(2, pokemon1.nom.len())]
            );

            let bebe_pokemon = Pokemon::new(nom, pokemon1.pokemon_type.clone(), genre);

            println!("🥚 Reproduction réussie! Un nouveau Pokémon est né:");
            println!("👶 {}", bebe_pokemon);

            self.pokemons.push(bebe_pokemon);
            true
        } else {
            println!("❌ Ces Pokémon ne peuvent pas se reproduire ensemble.");
            println!(
                "ℹ️ Rappel: ils doivent être du même type, de genres opposés, et de niveau 5 minimum."
            );
            false
        }
    }

    fn trier_par_niveau(&mut self) {
        self.pokemons.sort_by(|a, b| b.niveau.cmp(&a.niveau));
        println!("📊 Pokémon triés par niveau (décroissant).");
        self.afficher_tous_les_pokemons();
    }

    fn trier_par_type(&mut self) {
        self.pokemons
            .sort_by(|a, b| a.pokemon_type.as_str().cmp(b.pokemon_type.as_str()));
        println!("📊 Pokémon triés par type.");
        self.afficher_tous_les_pokemons();
    }

    fn supprimer_pokemon(&mut self, index: usize) -> bool {
        if index >= self.pokemons.len() {
            println!("❌ Index invalide");
            return false;
        }

        let pokemon = self.pokemons.remove(index);
        println!("🗑️ {} a été retiré de l'élevage.", pokemon.nom);
        true
    }

    fn sauvegarder(&self, chemin: &str) -> io::Result<()> {
        let mut fichier = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(chemin)?;

        writeln!(fichier, "{}", self.nom_elevage)?;

        for pokemon in &self.pokemons {
            writeln!(fichier, "{}", pokemon.to_string_format())?;
        }

        println!("💾 Élevage sauvegardé avec succès dans {}", chemin);
        Ok(())
    }

    fn charger(chemin: &str) -> io::Result<Self> {
        let fichier = File::open(chemin)?;
        let lecteur = BufReader::new(fichier);
        let mut lignes = lecteur.lines();

        let nom_elevage = match lignes.next() {
            Some(Ok(nom)) => nom,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Format de fichier invalide",
                ));
            }
        };

        let mut elevage = Self::new(&nom_elevage);

        for ligne in lignes {
            let ligne = ligne?;
            if let Some(pokemon) = Pokemon::from_string(&ligne) {
                elevage.pokemons.push(pokemon);
            }
        }

        println!(
            "📂 Élevage \"{}\" chargé avec succès ({} Pokémon).",
            elevage.nom_elevage,
            elevage.pokemons.len()
        );

        Ok(elevage)
    }
}

// Fonctions utilitaires
fn lire_entree() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");
    input.trim().to_string()
}

fn lire_entier() -> usize {
    loop {
        let input = lire_entree();
        match input.parse::<usize>() {
            Ok(n) => return n,
            Err(_) => {
                println!("❌ Veuillez entrer un nombre valide.");
                continue;
            }
        }
    }
}

fn afficher_menu() {
    println!("\n===== MENU PRINCIPAL =====");
    println!("1. Ajouter un Pokémon");
    println!("2. Afficher tous les Pokémon");
    println!("3. Entraîner un Pokémon");
    println!("4. Entraîner tous les Pokémon");
    println!("5. Reproduire deux Pokémon");
    println!("6. Trier les Pokémon par niveau");
    println!("7. Trier les Pokémon par type");
    println!("8. Supprimer un Pokémon");
    println!("9. Quitter");
    print!("Votre choix: ");
}

fn ajouter_pokemon_interactif(elevage: &mut Elevage) {
    println!("\n=== AJOUT D'UN POKÉMON ===");

    print!("Nom du Pokémon: ");
    let nom = lire_entree();

    println!("Types disponibles:");
    for (i, t) in PokemonType::get_all_types().iter().enumerate() {
        println!("{}. {}", i + 1, t.as_str());
    }

    print!("Choisissez un type (1-8): ");
    let type_idx = lire_entier();

    let pokemon_type = if type_idx >= 1 && type_idx <= 8 {
        PokemonType::get_all_types()[type_idx - 1].clone()
    } else {
        println!("❌ Type invalide. Utilisation du type Normal par défaut.");
        PokemonType::Normal
    };

    println!("Genre:\n1. Mâle\n2. Femelle");
    print!("Choisissez un genre (1-2): ");

    let genre = match lire_entier() {
        1 => Genre::Male,
        2 => Genre::Femelle,
        _ => {
            println!("❌ Genre invalide. Utilisation du genre Mâle par défaut.");
            Genre::Male
        }
    };

    elevage.ajouter_pokemon(Pokemon::new(nom, pokemon_type, genre));
}

fn entrainer_pokemon_interactif(elevage: &mut Elevage) {
    println!("\n=== ENTRAÎNEMENT D'UN POKÉMON ===");
    elevage.afficher_tous_les_pokemons();

    if elevage.pokemons.is_empty() {
        return;
    }

    print!("Choisissez un Pokémon (1-{}): ", elevage.pokemons.len());
    let index = lire_entier();

    if index < 1 || index > elevage.pokemons.len() {
        println!("❌ Indice invalide");
        return;
    }

    print!("Quantité d'XP à gagner (10-100): ");
    let xp = lire_entier();

    if xp < 10 || xp > 100 {
        println!("❌ Quantité d'XP invalide. Utilisation de 10 XP par défaut.");
        elevage.entrainer_pokemon(index - 1, 10);
    } else {
        elevage.entrainer_pokemon(index - 1, xp as u32);
    }
}

fn reproduire_pokemon_interactif(elevage: &mut Elevage) {
    println!("\n=== REPRODUCTION DE POKÉMON ===");
    elevage.afficher_tous_les_pokemons();

    if elevage.pokemons.len() < 2 {
        println!("❌ Vous avez besoin d'au moins 2 Pokémon pour la reproduction.");
        return;
    }

    print!(
        "Choisissez le premier Pokémon (1-{}): ",
        elevage.pokemons.len()
    );
    let index1 = lire_entier();

    print!(
        "Choisissez le second Pokémon (1-{}): ",
        elevage.pokemons.len()
    );
    let index2 = lire_entier();

    if index1 < 1
        || index1 > elevage.pokemons.len()
        || index2 < 1
        || index2 > elevage.pokemons.len()
    {
        println!("❌ Indices invalides");
        return;
    }

    elevage.tenter_reproduction(index1 - 1, index2 - 1);
}

fn supprimer_pokemon_interactif(elevage: &mut Elevage) {
    println!("\n=== SUPPRIMER UN POKÉMON ===");
    elevage.afficher_tous_les_pokemons();

    if elevage.pokemons.is_empty() {
        return;
    }

    print!(
        "Choisissez un Pokémon à supprimer (1-{}): ",
        elevage.pokemons.len()
    );
    let index = lire_entier();

    if index < 1 || index > elevage.pokemons.len() {
        println!("❌ Indice invalide");
        return;
    }

    elevage.supprimer_pokemon(index - 1);
}

const FICHIER_SAUVEGARDE: &str = "elevage_pokemon.txt";

fn main() {
    println!("\n==============================================");
    println!("🎮 POKÉMON ÉLEVAGE SIMULATOR 🎮");
    println!("==============================================\n");

    let mut elevage = if Path::new(FICHIER_SAUVEGARDE).exists() {
        println!("🔍 Fichier de sauvegarde trouvé!");
        match Elevage::charger(FICHIER_SAUVEGARDE) {
            Ok(e) => {
                println!(
                    "👋 Bienvenue de retour dans votre élevage \"{}\"!",
                    e.nom_elevage
                );
                e
            }
            Err(err) => {
                println!("❌ Erreur lors du chargement de la sauvegarde: {}", err);
                println!("🆕 Création d'un nouvel élevage...");

                print!("Donnez un nom à votre élevage: ");
                let nom_elevage = lire_entree();

                let e = Elevage::new(&nom_elevage);
                println!("\n👋 Bienvenue dans votre élevage \"{}\"!", nom_elevage);
                e
            }
        }
    } else {
        print!("Donnez un nom à votre élevage: ");
        let nom_elevage = lire_entree();

        let e = Elevage::new(&nom_elevage);
        println!("\n👋 Bienvenue dans votre élevage \"{}\"!", nom_elevage);
        println!("Commençons par ajouter quelques Pokémon...\n");
        e
    };

    loop {
        afficher_menu();
        match lire_entier() {
            1 => ajouter_pokemon_interactif(&mut elevage),
            2 => elevage.afficher_tous_les_pokemons(),
            3 => entrainer_pokemon_interactif(&mut elevage),
            4 => {
                print!("Quantité d'XP à gagner pour tous (10-100): ");
                let xp = lire_entier();
                if xp < 10 || xp > 100 {
                    println!("❌ Quantité d'XP invalide. Utilisation de 10 XP par défaut.");
                    elevage.entrainer_tous_les_pokemons(10);
                } else {
                    elevage.entrainer_tous_les_pokemons(xp as u32);
                }
            }
            5 => reproduire_pokemon_interactif(&mut elevage),
            6 => elevage.trier_par_niveau(),
            7 => elevage.trier_par_type(),
            8 => supprimer_pokemon_interactif(&mut elevage),
            9 => {
                if let Err(err) = elevage.sauvegarder(FICHIER_SAUVEGARDE) {
                    println!("❌ Erreur lors de la sauvegarde: {}", err);
                }
                println!("\n👋 Merci d'avoir joué à Pokémon Élevage Simulator!");
                println!("À bientôt pour de nouvelles aventures Pokémon!\n");
                break;
            }
            _ => println!("❌ Option invalide. Veuillez réessayer."),
        }
    }
}
