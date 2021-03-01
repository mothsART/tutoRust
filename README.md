# tutoRust

Quelques exemples pour apprendre Rust dans le cadre d'un article de  blog à Versusmind.

## Installer Rust

Si vous n'avez pas Rust d'installé sur votre poste, je vous invites à suivre ce guide :

https://www.rust-lang.org/fr/tools/install

## Disposer de Make

Pour simplifier les choses, j'utilise un fichier makefile.
En réalité, chacun instructions lance ceci :

```
cargo run --bin <nom_executable>
```

Cargo, le gestionnaire de paquet va déduire à partir du nom, directement l'emplacement du fichier.
Il sera forcément à cet emplacement : "src/bin/<nom_executable>.rs"

