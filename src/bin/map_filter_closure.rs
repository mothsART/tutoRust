#[derive(PartialEq)] // Sans, il vous est impossible de comparer des valeurs de l'enum.
enum LanguagesInfo {
    Javascript,
    TypeScript,
    C,
    Rust,
    Java,
    Python,
    CSharp
}

struct DevVersus {
    name: &'static str,
    age: i16,
    anciennete: i8,
    languages: Vec<LanguagesInfo>
}

struct Versus {
    devs: Vec<DevVersus>
}

impl Versus {
    fn new() -> Versus {
        /*
        On initialise avec une liste de valeurs.
        Celle-ci pourrait provenir d'une BDD, d'une API etc.
        */
        Versus {
            devs: vec![
                DevVersus {
                    name: "Linus Torvald",
                    age: 51,
                    anciennete: 20,
                    languages: vec![LanguagesInfo::C]
                },
                DevVersus {
                    name: "Evan You",
                    age: 28,
                    anciennete: 2,
                    languages: vec![
                        LanguagesInfo::Javascript,
                        LanguagesInfo::TypeScript
                     ]
                },
                DevVersus {
                    name: "James Gosling",
                    age: 65,
                    anciennete: 14,
                    languages: vec![
                        LanguagesInfo::Java,
                        LanguagesInfo::C
                     ]
                },
                DevVersus {
                    name: "Armin Ronacher",
                    age: 31,
                    anciennete: 5,
                    languages: vec![
                        LanguagesInfo::Python,
                        LanguagesInfo::Rust,
                        LanguagesInfo::Javascript
                     ]
                },
            ]
        }
    }
    
    // On part du principe qu'un nouveau chez Versus a 2 ans ou moins d'ancienneté.
    fn newbies(&self) -> Vec<&DevVersus> {
        self.devs
            .iter()
            .filter(|dev| dev.anciennete <= 2)
            .collect::<Vec<&DevVersus>>()
    }
    
    // On part du principe que les vieux de Versus ont plus de 5 ans d'ancienneté.
    fn boomers(&self) -> Vec<&DevVersus> {
        self.devs
            .iter()
            .filter(|dev| dev.anciennete > 5)
            .collect::<Vec<&DevVersus>>()
    }
}

fn main() {
    let versus = Versus::new();
    
    println!("1. Les ptits jeunes de Versus :");
    versus.newbies().iter().for_each(|dev| println!("- {}", dev.name));
    
    println!("\n2. Les dinosaures de Versus :");
    versus.boomers().iter().for_each(|dev| println!("- {}", dev.name));
    
    println!("\n3. Les devs qui maitrisent plus de 2 langages :");
    versus.devs
        .iter()
        .filter(|dev| dev.languages.len() > 2)
        .for_each(|dev| println!("- {}", dev.name));

    println!("\n4. L'âge moyen à Versus est de :");
    let somme_ages = versus.devs
        .iter()
        .map(move |dev| dev.age)
        .fold(0, |sum, i| sum + i);
    println!("{} ans", somme_ages as usize / versus.devs.len());

    println!("\n5. Le nombre de langages supportés à Versus :");
    let mut languages_supportes = vec![];
    for dev in versus.devs {
        for language in dev.languages {
            match languages_supportes.contains(&language) {
                true => continue,
                false => { }
            }
            languages_supportes.push(language);
        }
    }
    println!("{}", languages_supportes.len());
}
