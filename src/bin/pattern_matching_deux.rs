/* On crée la structure de données des individus :
 Celui-ci a un nom, un âge et est (ou non) un professionnel de santé.
*/
struct Individu {
    name: &'static str,
    age: u8,
    professionnel_sante: bool
}

/* Cette fonction détermine si un individu doit se faire vacciner ou non.
On ne passe pas les données de l'individu à la fonction car cela pourrait avoir des conséquences désastreuses sur la mémoire utilisée.
(imaginons des millions d'individus avec une structure de données bien plus grande)
Du coup, on passe uniquement le pointeur vers cette donnée.
*/
fn doit_se_faire_vacciner(individu: &Individu) -> bool {
    match individu {
        Individu { professionnel_sante: true, ..  } => true, // Si l'individu est un professionnel de santé, on renvoi true.
        i if i.age >= 55 => true, // Si l'individu a 55 ans ou plus, on renvoi true.
        _ => false // On renvoie false dans tous les autres cas de figure.
    }
}

fn main() {
    // On crée une liste d'individus avec leurs caractéristiques.
    let individus = vec![
        Individu {
            name: "Emmanuel Macron",
            professionnel_sante: false,
            age: 43
        },
        Individu {
            name: "Jean Castex",
            professionnel_sante: false,
            age: 55
        },
        Individu {
            name: "Didier Raoult",
            professionnel_sante: true,
            age: 68
        }
    ];

    // On boucle sur ces individus pour savoir s'ils doivent être vaccinés.
    println!("Qui doit se faire vacciner ?\n");
    for individu in individus {
        match doit_se_faire_vacciner(&individu) {
            true => println!("{} : oui", individu.name),
            false => println!("{} : non", individu.name)
        }
    }
}
