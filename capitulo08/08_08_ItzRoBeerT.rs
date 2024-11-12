fn main(){
	let grados_fahrenheit: f32 = 100.0;
	let grados_celsius = (grados_fahrenheit - 32.0) * 5.0 / 9.0;

	println!("{} grados Fahrenheit son {} grados Celsius", grados_fahrenheit, grados_celsius);
}
