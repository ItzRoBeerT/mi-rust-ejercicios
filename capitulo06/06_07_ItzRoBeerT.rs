fn main(){
    let mut sueldo: f64 = 1000.50;

    let referencia_sueldo = &mut sueldo;

    *referencia_sueldo += 1000.50;

    println!("El sueldo es: {}", referencia_sueldo);
}