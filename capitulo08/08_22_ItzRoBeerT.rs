fn main(){
	let peso: f32 = 80.0;
	let altura: f32 = 180.0;
	let altura_en_metros = altura / 100.0;
	
	let imc = peso / (altura_en_metros * altura_en_metros);
	println!("Tu IMC es: {}", imc);
}