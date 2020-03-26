fn main(){
	let mut sum;
	for n in 0..11{
		println!("{}", n );
		sum=n+(n-1);
		println!("La somma del valore n({}) piu' il valore precedente ({}) e': {}\n", n, n-1, sum);
	}
}