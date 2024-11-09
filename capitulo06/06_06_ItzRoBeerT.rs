fn main(){
    let caracter: char = 'E';
    let leer_letra = &caracter;

    println!("El valor de la referencia es: {}", *leer_letra);
}