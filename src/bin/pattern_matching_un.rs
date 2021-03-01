fn main() {
    let my_string = "hello";

    match my_string {
        "bonjour" => {
            println!("français");
        }
        "ciao" => {
            println!("italien");
        }
        "hello" => {
            println!("anglais");
        }
        "hola" => {
            println!("espagnol");
        }
        _ => {
            println!("je ne connais pas cette langue...");
        }
    }
}
