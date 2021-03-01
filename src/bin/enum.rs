enum TypeVirement {
    Iban { code: &'static str },
    Rib { code: &'static str }
}

enum TypeDePaiement {
    CarteBleue {
        code: i64,
        date: &'static str, // Rust n'a pas encore de primitives de dates mais vous pouvez installer cette lib qui sera à terme native : https://rust-lang-nursery.github.io/rust-cookbook/datetime 
        code_secret: i16
    },
    Virement(TypeVirement),
    PayPal { mail: &'static str }
}

struct Paiement {
    montant: i32,
    type_de_paiement: TypeDePaiement
}

fn main() {
    let virement = Paiement {
        montant: 100000,
        type_de_paiement: TypeDePaiement::Virement(
            TypeVirement::Iban { code: "FR763000100794123456789018530001" }
        )
    };
    
    let carte_bleue = Paiement {
        montant: 100000,
        type_de_paiement: TypeDePaiement::CarteBleue {
            code: 4875290154983021,
            date: "08/23",
            code_secret: 958
        }
    };
    
    let paypal = Paiement {
        montant: 100000,
        type_de_paiement: TypeDePaiement::PayPal {
            mail: "mon.destinataire@versusmind.eu"
        }
    };
}
