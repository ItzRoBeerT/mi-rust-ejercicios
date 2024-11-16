fn main() {

	fn division_entera(dividendo: i32, divisor: i32, cociente: &mut i32, resto: &mut i32) -> () {
		*cociente = dividendo / divisor;
		*resto = dividendo % divisor;
	}

	let dividendo: i32 = 10;
	let divisor: i32 = 3;
	let mut cociente: i32 = 0;
	let mut resto: i32 = 0;

	division_entera(dividendo, divisor, &mut cociente, &mut resto);

	println!("El cociente es: {}", cociente);
	println!("El resto es: {}", resto);

}