use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

struct Produit {
    nom: String,
    quantite: u32,
}

struct Inventaire {
    produits: HashMap<String, Produit>,
}

impl Inventaire {
    fn new() -> Self {
        Inventaire {
            produits: HashMap::new(),
        }
    }

    fn ajouter_produit(&mut self, nom: String, quantite: u32) {
        let produit = Produit {
            nom: nom.clone(),
            quantite,
        };
        self.produits.insert(nom, produit);
        println!("Produit ajouté avec succès !");
    }

    fn modifier_produit(&mut self, nom: &str, nouvelle_quantite: u32) -> bool {
        if let Some(produit) = self.produits.get_mut(nom) {
            produit.quantite = nouvelle_quantite;
            println!("Quantité modifiée avec succès !");
            true
        } else {
            println!("Produit non trouvé !");
            false
        }
    }

    fn supprimer_produit(&mut self, nom: &str) -> bool {
        if self.produits.remove(nom).is_some() {
            println!("Produit supprimé avec succès !");
            true
        } else {
            println!("Produit non trouvé !");
            false
        }
    }

    fn afficher_inventaire(&self) {
        if self.produits.is_empty() {
            println!("L'inventaire est vide.");
            return;
        }

        println!("====== INVENTAIRE ======");
        println!("{:<20} {:<10}", "NOM", "QUANTITÉ");
        println!("------------------------");

        for produit in self.produits.values() {
            println!("{:<20} {:<10}", produit.nom, produit.quantite);
        }
        println!("========================");
    }

    fn sauvegarder_dans_fichier(&self, chemin: &str) -> io::Result<()> {
        let mut file = File::create(chemin)?;

        for produit in self.produits.values() {
            writeln!(file, "{},{}", produit.nom, produit.quantite)?;
        }

        println!("Inventaire sauvegardé dans '{}'", chemin);
        Ok(())
    }
}

// Afficher le menu principal
fn afficher_menu() {
    println!("\n===== GESTION D'INVENTAIRE =====");
    println!("1. Ajouter un produit");
    println!("2. Modifier la quantité d'un produit");
    println!("3. Supprimer un produit");
    println!("4. Afficher l'inventaire");
    println!("5. Sauvegarder l'inventaire");
    println!("6. Charger l'inventaire");
    println!("0. Quitter");
    print!("Votre choix : ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut inventaire = Inventaire::new();
    let nom_fichier = "inventaire.txt";

    println!("Système de Gestion d'Inventaire");
    println!("===============================");

    loop {
        afficher_menu();

        let mut choix = String::new();
        io::stdin()
            .read_line(&mut choix)
            .expect("Erreur de lecture");

        match choix.trim() {
            "1" => {
                print!("Nom du produit : ");
                io::stdout().flush().unwrap();
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).expect("Erreur de lecture");
                let nom = nom.trim().to_string();

                print!("Quantité : ");
                io::stdout().flush().unwrap();
                let mut quantite_str = String::new();
                io::stdin()
                    .read_line(&mut quantite_str)
                    .expect("Erreur de lecture");

                match quantite_str.trim().parse::<u32>() {
                    Ok(quantite) => inventaire.ajouter_produit(nom, quantite),
                    Err(_) => println!("Quantité invalide!"),
                }
            }
            "2" => {
                print!("Nom du produit à modifier : ");
                io::stdout().flush().unwrap();
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).expect("Erreur de lecture");
                let nom = nom.trim();

                print!("Nouvelle quantité : ");
                io::stdout().flush().unwrap();
                let mut quantite_str = String::new();
                io::stdin()
                    .read_line(&mut quantite_str)
                    .expect("Erreur de lecture");

                match quantite_str.trim().parse::<u32>() {
                    Ok(quantite) => {
                        inventaire.modifier_produit(nom, quantite);
                    }
                    Err(_) => println!("Quantité invalide!"),
                }
            }
            "3" => {
                print!("Nom du produit à supprimer : ");
                io::stdout().flush().unwrap();
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).expect("Erreur de lecture");
                let nom = nom.trim();

                inventaire.supprimer_produit(nom);
            }
            "4" => {
                inventaire.afficher_inventaire();
            }
            "5" => match inventaire.sauvegarder_dans_fichier(nom_fichier) {
                Ok(_) => {}
                Err(e) => println!("Erreur lors de la sauvegarde: {}", e),
            },
            "0" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Option invalide !"),
        }
    }
}
