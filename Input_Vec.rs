use std::io;
fn main() {
    println!("Ora crea il tuo vettore! Inserire la dimensione che gli si vuole dare: ");
	let mut n=String::new();
	io::stdin().read_line(&mut n).expect("Impossibile leggere l'input :(");
	println!("\n");
	let n1: i32 =n.trim().parse().expect("Input non valido :(");
	let mut v: Vec<i32> = Vec::new();
	let mut i: i32= 0;
	while i < n1{
		println!("Inserire l'elemento '{}' del vettore: ", i);
		let mut number=String::new();
		io::stdin().read_line(&mut number).expect("Impossibile leggere l'input :(");
		let element: i32 =number.trim().parse().expect("Input non valido :(");
		v.push(element);
		i=i+1;
	}
	println!("{:?}", v);
}