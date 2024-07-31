# Capítulo 6
* Libro «[Curso de Programación con Rust](https://amzn.to/3WEqWZJ)» (Autor: [@EliezerLopez](https://linkedin.com/in/eliezerlopez) · Grupo ANAYA).
* Cursos en video y web de soporte: [eliezerlopez.rs](https://eliezerlopez.rs).

## Ejercicio 06.01
Declara una referencia, y defínela con el contenido de la variable ```cuenta_corriente```. Deja que el compilador infiera el tipo ```&f64```.

## Ejercicio 06.02
Responde a la siguiente pregunta: una referencia inmutable, ¿será siempre de lectura y escritura?

## Ejercicio 06.03
¿Cuántas referencias inmutables pueden existir al mismo tiempo, dentro de un mismo ámbito?

## Ejercicio 06.04
¿Cuántas referencias mutables pueden existir a la vez, dentro de un mismo ámbito?

## Ejercicio 06.05
¿Cuántas referencias mutables e inmutables pueden coexistir en un determinado ámbito?

## Ejercicio 06.06
Crea una referencia inmutable llamada ```leer_letra```, que tome prestado el valor ```E```.

## Ejercicio 06.07
Crea una referencia exclusiva capaz de modificar el sueldo de un directivo de una determinada empresa.

## Ejercicio 06.08
¿Es posible aplicar el sombreado (_shadowing_) a referencias? 

## Ejercicio 06.09
El siguiente programa no funciona. Copia el código en tu editor de código favorito y haz que compile.
```
fn main() {
    let edad: f64 = 30;
    let altura: &f32 = 182.00;
    let inicial_nombre = E;

    edad = 32;
    altura = 183;
    modificador_caracter: &char = &inicial_nombre;
}
```
