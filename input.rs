use std::io;

fn main() {
    println!("Inserisci qualcosa: ");

    let mut qualcosa = String::new();

    io::stdin().read_line(&mut qualcosa).expect("Failed to read line");

    println!("Hai inserito: {}", qualcosa);
}