use std::io;
fn main(){
	println!("Inserire un numero: ");
	let mut n=String::new();
	io::stdin().read_line(&mut n).expect("Impossibile leggere l'input :(");
	println!("\n");
	let mut n1: i32 = n.trim().parse().expect("Input non valido :(");
	while n1!=0 {
		println!("Il numero adesso corrisponde a: {}", n1);
		if n1%2==0 {
			n1=n1/2;
			println!("Il numero dimezzato e' corrisponde a: {}\n", n1);
		}
		else {
			n1-=1;
			println!("Il numero è dispari... Tolgo 1 per renderlo pari :)");
			n1=n1/2;
			println!("Il numero dimezzato e' corrisponde a: {}\n", n1);
		}
	}
}