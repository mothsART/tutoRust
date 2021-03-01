struct Dossier {
	name: &'static str,
	sous_dossier: Option<Box<Dossier>>
}

fn main() {
	let herarchie = Dossier {
		name: "racine",
		sous_dossier: Some(Box::new(Dossier {
			name: "dossier",
			sous_dossier: Some(Box::new(Dossier {
				name: "sous dossier",
				sous_dossier: None
			}))
		}))
	};
}
