#[derive(Debug)]
struct Coordonnees {
    latitude: f32,
    longitude: f32
}

#[derive(Debug)]
struct Rover {
    coordonnees_initiales: Coordonnees,
    coordonnees_actuelles: Coordonnees
}

impl Rover {
    fn new(latitude: f32, longitude: f32) -> Rover {
        Rover {
            coordonnees_initiales: Coordonnees {
                latitude: latitude,
                longitude: longitude
            },
            coordonnees_actuelles: Coordonnees {
                latitude: latitude,
                longitude: longitude
            },
        }
    }

    fn deplacer(&mut self, latitude: f32, longitude: f32) {
        self.coordonnees_actuelles.latitude = latitude;
        self.coordonnees_actuelles.longitude = longitude;
        // ... => Ensemble d'instructions métiers pour déplacer le rover jusqu'aux coordonnées demandées (ex: tourner à gauche pendant x secondes, accélérer, etc.).
    }
    
    fn afficher_position(&self) {
        println!("{:?}", self);
    }
}
    
fn main() {
    // On initialise avec les coordonnées d'atterrissage du Rover Persévérance (totalement bidon, c'est pour l'exemple).
    let mut rover_perseverance = Rover::new(37.28, 76.60);
    // On ordonne le premier déplacement.
    rover_perseverance.deplacer(40.2, 78.3);
    // On ordonne le deuxième déplacement.
    rover_perseverance.deplacer(42.6, 78.3);
    // On affiche les dernières coordonnées.
    rover_perseverance.afficher_position();
}
