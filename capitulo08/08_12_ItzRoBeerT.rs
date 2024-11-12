fn main() {
	let annio_actual: u16 = 2021;

	let es_bisiesto = annio_actual % 4 == 0 && (annio_actual % 100 != 0 || annio_actual % 400 == 0);

	println!("{}", es_bisiesto);
}