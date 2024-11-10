fn main() {
    let producto = ("Camiseta", 15.90, 3);
    let (nombre, precio, cantidad) = producto;
    println!("Nombre: {}", nombre);
    println!("Precio: {}", precio);
    println!("Cantidad: {}", cantidad);
}