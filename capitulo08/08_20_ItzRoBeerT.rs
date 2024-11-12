fn main() {
	let precio1: f32 = 10.0;
	let precio2: f32 = 15.0;

	let precio_mas_barato: f32;

	if precio1 < precio2 {
		precio_mas_barato = precio1;
	} else {
		precio_mas_barato = precio2;
	}

	println!("El precio más barato es: {}", precio_mas_barato);
}