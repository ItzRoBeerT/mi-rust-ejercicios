# Capítulo 8
* Libro «[Curso de Programación con Rust](https://amzn.to/3WEqWZJ)» (Autor: [@EliezerLopez](https://linkedin.com/in/eliezerlopez) · Grupo ANAYA).
* Cursos en video y web de soporte: [eliezerlopez.rs](https://eliezerlopez.rs).

## Ejercicio 08.01
Resta dos números reales.

## Ejercicio 08.02
Calcula el cuadrado de un número.

## Ejercicio 08.03
Calcula el cubo de un número.

## Ejercicio 08.04
Implementa la media de 3 números enteros.

## Ejercicio 08.05
Incrementa el número de unidades de un producto en una unidad usando el operador +=. 

## Ejercicio 08.06
Imagina que trabajas como desarrollador o desarrolladora de software en un banco.
Tu aplicación debe ser capaz de calcular el saldo neto de una cuenta bancaria cualquiera a partir de transacciones de ingresos (valores positivos) y gastos (valores negativos).
Modela los ingresos y gastos como dos variables y calcula el sueldo neto.

## Ejercicio 08.07
Calcula el área de un rectángulo dado su alto y su ancho.

## Ejercicio 08.08
Convierte la temperatura de grados Fahrenheit a Celsius.

## Ejercicio 08.09
Convierte la temperatura de grados Celsius a Fahrenheit.

## Ejercicio 08.10
Calcula el área de un círculo a partir de su radio.

## Ejercicio 08.11
Verifica si una persona es mayor de edad (18 años o más) y tiene permiso de conducir.

## Ejercicio 08.12
Comprueba si un año es bisiesto. Un año es bisiesto cuando es divisible por 4 pero no por 100, o divisible por 400.

## Ejercicio 08.13
Para presentar la solicitud a un determinado puesto de trabajo, un individuo debe ser mayor de edad y carecer de antecedentes penales.
Implementa esta casuística a nivel de código.

## Ejercicio 08.14
Comprueba si un número es positivo, par y menor que 108.

## Ejercicio 08.15
La asociación de donantes de sangre acepta donaciones de personas cuya edad esté comprendida entre 18 y 65 años, y su peso sea mayor a 50Kg.
Escribe el código equivalente.

## Ejercicio 08.16
Verifica si un número es positivo y divisible entre 3.

## Ejercicio 08.17
Debes elegir solo uno de dos objetos para incluir en tu mochila de viaje.
Escribe un programa que averigüe cuál el más pequeño.

## Ejercicio 08.18
Determina si un estudiante ha aprobado la asignatura de Programación.
El estudiante aprueba si su nota es igual o superior a 5.

## Ejercicio 08.19
Compara dos fechas en formato ```YYYYMMDD``` y determina cuál es anterior.

## Ejercicio 08.20
El aceite de oliva está a precios desorbitados.
Implementa un programa que, a partir de los precios obtenidos en dos supermercados diferentes, determine cuál es más barato.

## Ejercicio 08.21
Calcula el volumen de una caja a partir de su largo, ancho y alto.

## Ejercicio 08.22
Implementa un programa que calcule el IMC (Índice de Masa Corporal) dados el peso y la altura de una persona.

## Ejercicio 08.23
¿Cuál es el valor final de ```j```?

```
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
}
```
