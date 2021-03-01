#[derive(Debug)] // On précise ceci pour que l'on puisse afficher facilement le détail de notre struct dans un println!("{:?}", ...)
struct Adresse {
    code_postal: String,
    rue: &'static str,
    numero: &'static str
}

fn verifier(mon_adresse: &Adresse) -> bool {
    if mon_adresse.code_postal.len() == 5 {
        return true;
    }
    false
}

fn main() {
    let ancienne_adresse = Adresse {
        code_postal: String::from("54000"),
        rue: "Isabey",
        numero: "20"
    };
    println!("L'ancienne adresse de Versus : {:?}", ancienne_adresse);
    
    let nouvelle_adresse = Adresse {
        code_postal: String::from("54000"),
        rue: "Christian Pfister",
        numero: "30 TER"
    };  
    if verifier(&nouvelle_adresse) {
        println!("La nouvelle adresse de Versus : {:?}", nouvelle_adresse);
    }
}
