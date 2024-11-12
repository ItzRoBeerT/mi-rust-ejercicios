fn main() {
	let numero: i32 = 234;

	let valido = numero > 0 && numero % 3 == 0;

	println!("{}", valido);
}