# Capítulo 14
* Libro «[Curso de Programación con Rust](https://amzn.to/3WEqWZJ)» (Autor: [@EliezerLopez](https://linkedin.com/in/eliezerlopez) · Grupo ANAYA).
* Cursos en video y web de soporte: [eliezerlopez.rs](https://eliezerlopez.rs).

## Ejercicio 14.01
Define una estructura llamada ```Persona``` con los campos ```nombre```, ```edad``` y ```correo_electronico```.
Crea un vector dinámico con 10 personas cuya información sea proporcionada por el usuario del programa.
Tras introducir la información de las 10 personas, imprime un listado en pantalla con las direcciones de correo electrónico. Contempla la situación de que una persona no tenga correo electrónico.

## Ejercicio 14.02
Implementa una estructura ```Coche``` que almacene la marca, el modelo y el año.

## Ejercicio 14.03
Diseña una estructura de tupla llamada ```Distancia``` con los campos ```grados``` y ```escala```.

## Ejercicio 14.04
Modela la hora con los campos ```hora```, ```minutos``` y ```segundos``` a través de una estructura de tupla.

## Ejercicio 14.05
Define una estructura llamada ```Complejo``` para el modelado de números complejos. Esta estructura debe contar dos campos para el almacenamiento de la parte real y la parte imaginaria.

## Ejercicio 14.06
Implementa una estructura unitaria ```Administrador``` y escribe un programa que la utilice.

## Ejercicio 14.07
Implementa un método observador para la estructura ```Persona``` que creaste en el primer ejercicio.

## Ejercicio 14.08
Crea una estructura ```Libro``` con un método observador que permita obtener su título.

## Ejercicio 14.09
Diseña el tipo ```Producto``` con un método modificador que permita actualizar su precio.

## Ejercicio 14.10
Implementa una estructura ```Canción``` con los campos ```titulo```, ```duración``` y ```artista```.

## Ejercicio 14.11
Crea una lista de reproducción de canciones. Para ello, utiliza la estructura que implementaste en el ejercicio anterior.

## Ejercicio 14.12
Crea un nuevo tipo ```Color``` para modelar cualquier color escrito en RGB (R = *Red*, G = *Green*, B = *Blue*).

## Ejercicio 14.13
Modela la resolución de una pantalla a través de una estructura de tupla.

## Ejercicio 14.14
Crea el tipo ```Direccion``` para modelar la dirección física en la que vive una persona.

## Ejercicio 14.15
Añade métodos modificadores al tipo ```Direccion``` para actualizar el código postal y la ciudad.

## Ejercicio 14.16
Modifica la estructura ```Persona``` y añádele una dirección física. Para ello, utiliza el tipo ```Direccion```.

## Ejercicio 14.17
Oye… ¿Estás comprobando que tus tipos funcionan correctamente, ¿verdad?
Implementa un programa que muestre un menú en pantalla y permita al usuario probar los 16 ejercicios anteriores.
Puedes incluir cada uno de estos ejercicios en una función y llamarla según la opción elegida por el usuario.

## Ejercicio 14.18
Incluye un enumerado en el tipo ```Direccion``` para modelar el ```Pais``` en el que vive una persona. Debes asegurar que el programa reacciona correctamente si el nombre de algún país introducido no es correcto.

## Ejercicio 14.19
Diseña una estructura ```Evento``` y define métodos para cambiar la hora y obtener una descripción completa.

## Ejercicio 14.20
Crea una estructura de tupla llamada ```Intervalo``` y define métodos para verificar si un rango está dentro del intervalo.

## Ejercicio 14.21
Implementa una estructura ```Vehiculo``` y define métodos para cambiar el color y obtener el modelo. Crea también las estructuras ```Furgoneta``` y ```Moto``` y haz uso de traits para la implementación de sus métodos.

## Ejercicio 14.22
Crea una estructura para modelar una ```Fecha```.

## Ejercicio 14.23
Añade a la estructura del ejercicio 14.22 la posibilidad de restar días.

## Ejercicio 14.24
Implementa un método en la estructura del ejercicio 14.22 que permita convertir una ```Fecha``` en un ```String```.

## Ejercicio 14.25
Sobrecarga el trait ```Add``` para sumar un número de días a una fecha determinada. El resultado debe ser de tipo ```Fecha```.

## Ejercicio 14.26
Sobrecarga el operador ```*``` para multiplicar una variable de tipo ```Empleado``` por un número entero.
Esta implementación permitirá multiplicar el sueldo bruto anual de un empleado por un número entero.
Así, los responsables de equipo podrán estimar fácilmente la inversión en sueldos al considerar la ampliación de su equipo con empleados de perfiles similares.

## Ejercicio 14.27
Crea un tipo llamado ```Punto3D``` que modele un punto en un espacio de 3 dimensiones. Implementa el trait ```Sub``` para calcular la distancia entre dos puntos mediante el uso del operador de resta ```-```.

## Ejercicio 14.28
Ahora que ya sabes sobrecargar el trait ```Add```, intenta implementar este trait para solucionar el error ```no implementation for ‘f64 + f32’```, de modo que puedas sumar un valor de tipo ```f64``` con otro de tipo ```f32```.

## Ejercicio 14.29
Sobrecarga el operador ```/``` para dividir una estructura ```Fracción``` entre otra.

## Ejercicio 14.30
Crea una estructura ```Producto``` que modele 2 productos de un supermercado. Implementa el operador ```<``` para determinar si un producto es más barato que otro.

## Ejercicio 14.31
Rescata la estructura ```Fecha``` que creaste en el ejercicio 14.22. Sobrecarga el operador ```<``` para comprar si una ```Fecha``` es anterior a otra.

## Ejercicio 14.32
Recupera la estructura ```Libro``` que implementaste en el ejercicio 14.8, añádele el campo ```Autor``` y sobrecarga el operador ```==``` para comparar si 2 estructuras ```Libro``` tienen el mismo título y autor.

## Ejercicio 14.33
¿Tienes experiencia desarrollando software en lenguajes que admiten clases, herencia y polimorfismo?
Rescata alguna de tus implementaciones en esos lenguajes y trata de producir un código equivalente en Rust.
Será un ejercicio enriquecedor que te ayudará a sacar el máximo partido de los structs, los bloques ```impl``` y los traits.
