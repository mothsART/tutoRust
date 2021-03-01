#[derive(Debug)]
struct Coordonnees {
    latitude: f32,
    longitude: f32
}

#[derive(Debug)]
struct RoverPerseverance {
    coordonnees_initiales: Coordonnees,
    coordonnees_actuelles: Coordonnees
}

trait Rover {
    fn new(latitude: f32, longitude: f32) -> Self;
    fn deplacer(&mut self, latitude: f32, longitude: f32);
    fn afficher(&self);
}

impl Rover for RoverPerseverance {
    fn new(latitude: f32, longitude: f32) -> RoverPerseverance {
        RoverPerseverance {
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
    }
    
    fn afficher(&self) {
        println!("{:?}", self);
    }
}
    
fn main() {
    let mut rover_perseverance = RoverPerseverance::new(37.28, 76.60);
    rover_perseverance.deplacer(40.2, 78.3);
    rover_perseverance.deplacer(42.6, 78.3);
    rover_perseverance.afficher();
}
