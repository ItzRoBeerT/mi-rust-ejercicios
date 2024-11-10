fn main() {
    let mut temperatura_semanal: [[f64; 3]; 6] = [
        [21.5, 24.5, 22.3],
        [21.5, 24.5, 22.3],
        [21.5, 24.5, 22.3],
        [21.5, 24.5, 22.3],
        [21.5, 24.5, 22.3],
        [21.5, 24.5, 22.3],
    ];

    let primera_fila = &mut temperatura_semanal[0];
    primera_fila[0] = 20.0;
    primera_fila[1] = 22.5;
    primera_fila[2] = 24.5;
    println!("{:?}", primera_fila);
}