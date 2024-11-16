fn main() {
    fn es_vocal(letra: char) -> bool {
        let letra = letra.to_ascii_lowercase();
        return letra == 'a' || letra == 'e' || letra == 'i' || letra == 'o' || letra == 'u';
    }

    let letra = 'A';
    println!("{}", es_vocal(letra));
}
