fn main() {
	let edad: u8 = 25;
	let antecedentes_penales: bool = false;

	let es_apto = edad >= 18 && !antecedentes_penales;
	
	println!("{}", es_apto); 
}