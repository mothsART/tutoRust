// on définit le gabarit :
struct Planete {
    name: &'static str, // Le type de cette propriété est une chaîne de caractères.
    tellurique: bool, // Le type de cette propriété est une valeur booléenne.
    diametre: f32 // Le type de cette propriété est une valeur décimale correspondant au rapport de grandeur avec le diamètre de la terre.
}

fn main() {
    // on crée les données correspondant à ce gabarit.
    let terre = Planete {
        name: "Terre",
        tellurique: true,
        diametre: 1. // On renseigne volontairement "1." (et non "1") pour bien préciser au compilateur qu'on souhaite renseigner une valeur décimale et non un entier.
    };
    
    let mars =  Planete {
        name: "Mars",
        tellurique: true,
        diametre: 0.532 // Le diamètre de Mars fait environ la moitié de celle de la Terre.
    };
    
    let jupiter = Planete {
        name: "Jupiter",
        tellurique: false,
        diametre: 11.209 // Le diamètre de Jupiter est supérieur à 12 fois le diamètre de la Terre!
    };
}
