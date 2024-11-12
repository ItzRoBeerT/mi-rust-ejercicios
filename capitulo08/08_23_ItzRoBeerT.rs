fn main() {
    let a = 5 + 3 * 2;
    let b = a - 4 / 2 + 6;
    let c = b * (a - 2);
    let d = c / (a + b);
    let e = d % 3;
    let f = (e * 4 + a - b) / 2;
    let g = f * (c - d) + e;
    let h = g / (a + e - b * 2);
    let i = h + 7 - c % 5;
    let j = i * (d + e - a);
	
    println!("Valor final de j: {}", j)
}
