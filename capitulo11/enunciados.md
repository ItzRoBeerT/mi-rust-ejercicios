# Capítulo 11
* Libro «[Curso de Programación con Rust](https://amzn.to/3WEqWZJ)» (Autor: [@EliezerLopez](https://linkedin.com/in/eliezerlopez) · Grupo ANAYA).
* Cursos en video y web de soporte: [eliezerlopez.rs](https://eliezerlopez.rs).

## Ejercicio 11.01
Imprime tu nombre y apellidos en pantalla.

## Ejercicio 11.02
Imprime tu nombre y apellidos en pantalla incluyendo al final un salto de línea.

## Ejercicio 11.03
Rescata el ejercicio 6.1 y muestra el contenido de la variable ```cuenta_corriente``` utilizando la referencia.

## Ejercicio 11.04
¿Has encontrado todos los errores en el código del ejercicio 6.9? Muestra en pantalla el contenido de la variable ```altura``` con 3 decimales.

## Ejercicio 11.05
En el ejercicio 7.1 creaste una tupla para almacenar el nombre, el precio y la cantidad de un producto. Muestra ahora esta información en pantalla en diferentes líneas.

## Ejercicio 11.06
En el ejercicio 7.2 declaraste una tupla para representar la latitud y la longitud de un lugar. Pide ahora esta información por teclado al usuario de tu programa.

## Ejercicio 11.07
Crea un array unidimensional que almacene los precios de 5 productos diferentes en una tienda. Pide la información por teclado y muéstrala en pantalla, en diferentes filas y con alineación a la derecha.

## Ejercicio 11.08
Vuelve al ejercicio 7.8, convierte las cadenas literales a cadenas de tipo ```String``` y pide al usuario su nombre, su primer apellido y su segundo apellido.
Confirma que los datos se recogen correctamente mostrándolos en pantalla.

## Ejercicio 11.09
El código del ejercicio 7.9 funciona gracias al sombreado. Imprime en pantalla el contenido de la variable ```libro``` antes y después de que el shadowing entre en funcionamiento.

## Ejercicio 11.10
Comenta una de las líneas del ejercicio 11.9.

## Ejercicio 11.11
Implementa la media de 4 números enteros y muestra el resultado en pantalla.

## Ejercicio 11.12
Rescata el ejercicio 8.7. Solicita por teclado las medidas de un rectángulo y muestra su área en pantalla.

## Ejercicio 11.13
Implementa un programa que solicite por teclado la temperatura en grados Fahrenheit, la convierta a grados Celsius y muestre el resultado en pantalla.

## Ejercicio 11.14
Modifica el programa anterior para que funcione en el sentido opuesto: solicitar por teclado la temperatura en grados Celsius y mostrar en pantalla su equivalencia en grados Fahrenheit.

## Ejercicio 11.15
Muestra en pantalla el contenido de la variable booleana que creaste en el ejercicio 8.11. ¿Cómo se muestran los valores booleanos en pantalla?

## Ejercicio 11.16
Solicita por teclado el año en el que naciste y comprueba si ese año fue bisiesto.

## Ejercicio 11.17
Solicita 2 valores por teclado, llama a las funciones ```sumar()```, ```restar()``` y ```multiplicar()``` que implementaste en los ejercicios 9.1, 09.3 y 09.4 y muestra los resultados en pantalla.

## Ejercicio 11.18
Modifica el programa del ejercicio 11.17 y muestra los resultados en hexadecimal.

## Ejercicio 11.19
¿Estás controlando correctamente los valores devueltos de tipo ```Result``` en los ejercicios que estás resolviendo? Repasa todos tus programas y presta atención a esto.

## Ejercicio 11.20
Implementa un programa que solicite una letra por teclado, llame a la función ```es_vocal()``` implementada en el ejercicio 09.13 e imprima en pantalla si la letra leída es una vocal.

## Ejercicio 11.21
Crea un vector vacío de ```String``` y rellénalo con 3 apellidos solicitados por teclado al usuario de tu programa.

## Ejercicio 11.22
Muestra en pantalla el valor del vector implementado en el ejercicio 10.23 y confirma que la inserción se realizó correctamente.

## Ejercicio 11.23
Muestra en pantalla el resultado de la transformación implementada en el ejercicio 10.27.

## Ejercicio 11.24
Modifica el ejercicio 11.21 y transforma cualquier frase proporcionada por teclado por el usuario del programa.

## Ejercicio 11.25
Muestra en pantalla el resultado de la suma que implementaste en el ejercicio 10.37. La información debe estar centrada en un ancho de 42. Completa con asteriscos los espacios a izquierda y derecha.

## Ejercicio 11.26
Vuelve al ejercicio 10.38 y muestra la información de la subcadena.

## Ejercicio 11.27
¿Qué valor se muestra en pantalla?
```
fn main() {
    let v = vec![5, 10, 15, 20];
    let suma = /* v[0] + */ v[2];
    println!("Resultado: {}", suma);
}
```
Reflexiona sobre lo que hace este programa. Una vez que tengas tu solución, copia el código en tu entorno de desarrollo para verificarla.

## Ejercicio 11.28
¿Cómo crees que se muestra en consola la matriz modelada en el siguiente programa?
```
fn main(){
    let numeros = vec![
                  vec![4, 8, 15],
                  vec![16, 23, 42],
                  vec![23, 16, 8,
                  ];
    println!("Contenido de la matriz: {:?}", numeros);
}
```
Compila el programa. Ups… ¿qué ocurre? ¿Es lo que esperabas?

## Ejercicio 11.29
Vuelve al ejercicio 9.23 y asegúrate de que ```suma_array_por_referencia()``` funciona correctamente imprimiendo el resultado en pantalla.
