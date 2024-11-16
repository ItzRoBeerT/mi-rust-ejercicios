fn main() {
	macro_rules !convertir_a_mayusculas {
		($cadena: expr) => {
			$cadena.to_ascii_uppercase()
		}
	}

	let cadena: &str = "¡Hola mundo!";

	println!("{}", convertir_a_mayusculas!(cadena));
}