fn main() {
	let numero: i32 = 234;
	let valido = numero > 0 && numero % 2 == 0 && numero < 108;

	println!("{}", valido);
}