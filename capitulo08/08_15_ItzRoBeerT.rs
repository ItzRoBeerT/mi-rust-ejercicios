fn main() {
	let persona = ("Juan", 25, 60);
	let (nombre, edad, peso) = persona;

	let valido = edad >= 18 && edad <= 65 && peso >= 50;

	println!("{}", valido);
}