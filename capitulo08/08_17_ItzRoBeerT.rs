fn main() {
	let objeto1 = ("Botella", 100);
	let (nombre, tamannio) = objeto1;
	let objeto2 = ("Libro", 200);
	let (nombre2, tamannio2) = objeto2;

	if tamannio < tamannio2 {
		println!("Debes llevar: {}", nombre);
	} else {
		println!("Debes llevar: {}", nombre2);
	}

	
}