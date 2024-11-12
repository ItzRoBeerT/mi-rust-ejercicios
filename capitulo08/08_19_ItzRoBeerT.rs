fn main() {
	let fecha1 = (2002, 04, 21);
	let fecha2 = (2020, 05, 13);

	let anterior = fecha1.0 < fecha2.0; 
	println!("{}", anterior);
}