
# Capítulo 13
* Libro «[Curso de Programación con Rust](https://amzn.to/3WEqWZJ)» (Autor: [@EliezerLopez](https://linkedin.com/in/eliezerlopez) · Grupo ANAYA).
* Cursos en video y web de soporte: [eliezerlopez.rs](https://eliezerlopez.rs).

## Ejercicio 13.01
Escribe un programa que imprima los números del 1 al 42 usando ```loop```. Crea 2 versiones alternativas: una que utilice bucle ```while``` y otra con el bucle ```for```.

## Ejercicio 13.02
Calcula la suma de todos los números del 1 al 108 utilizando el bucle ```for```.

## Ejercicio 13.03
Imagina que necesitas implementar un programa que registre continuamente temperaturas. El programa debe almacenar solo la temperatura más baja que se registre hasta el momento.
1.	Lee las temperaturas una tras otra desde el teclado con un bucle ```loop```.
2.	Si la temperatura es menor que la temperatura guardada hasta el momento, actualiza la temperatura guardada.
3.	Muestra la temperatura actualizada en pantalla.

## Ejercicio 13.04
Imprime la tabla de multiplicar del 7 utilizando los bucles ```for``` y ```while```.

## Ejercicio 13.05
Utiliza ```while let``` para recorrer una lista de temperaturas y encontrar la primera que sea negativa.

## Ejercicio 13.06
Crea un vector para almacenar tu sueldo neto mensual para cada mes del año. Con bucles anidados, recorre el vector y haz la media para cada trimestre. Muestra la información en pantalla.

## Ejercicio 13.07
Crea un programa que recoja continuamente valores numéricos positivos por teclado y los almacene en un vector dinámico.
En cada iteración, calcula la media y muéstrala en pantalla. Si el usuario introduce valores negativos, muestra un mensaje error y solicita de nuevo el valor.
Si el usuario introduce el valor -1 el programa finaliza.

## Ejercicio 13.08
Calcula la suma de los números impares entre 1 y 108 usando ```while let```.

## Ejercicio 13.09
Crea un vector de 16 temperaturas. Rellénalo con valores proporcionados por el usuario, y utiliza un bucle ```while let``` para recorrer el vector y contar cuántos valores están por encima de 30 grados.

## Ejercicio 13.10
Imprime los números pares del 1 al 108 en orden inverso.

## Ejercicio 13.11
Crea un vector de 20 nombres de personas. Cuenta cuántos nombres tienen exactamente 5 letras utilizando un bucle ```while let```.

## Ejercicio 13.12
Eres entrenador de un equipo de baloncesto.
Implementa un programa que solicite el nombre y la edad de los jugadores, almacene la información en un vector de tuplas (```nombre```, ```edad```) y muestre en pantalla el nombre del jugador más joven.

## Ejercicio 13.13
Tu hija posee una mente brillante y se está preparando para participar en un concurso de deletreo.
Para ayudarla a practicar, deseas desarrollar un programa que, dada una palabra ingresada por teclado, la deletree mostrando una letra por línea.

## Ejercicio 13.14
Estás al frente de la dirección de un nuevo supermercado en la ciudad. Calcula el precio medio de tus 20 productos más vendidos.

## Ejercicio 13.15
Crea una función que reciba una palabra y devuelva sus letras ordenadas en orden alfabético.

## Ejercicio 13.16
Calcula la suma de los dígitos de un número proporcionado por teclado.

## Ejercicio 13.17
Crea una lista de 8 apellidos vascos y cuenta cuántas de ellos tienen más de 3 vocales.

## Ejercicio 13.18
Lee una frase por teclado, reemplaza todas sus vocales por un asterisco ```*``` y muestra el resultado en pantalla.

## Ejercicio 13.19
Después de varios meses al frente la dirección de tu nuevo supermercado, te has dado cuenta de la alta carga impositiva en nuestro país.
A continuación, crea un vector que almacene 20 precios sin impuestos añadidos y muestra en pantalla cuánto pagaría un consumidor cualquiera si el IVA fuera del 10% en lugar del 21%.
Calcula el ahorro total de que supondría sobre una cesta con estos 20 productos.

## Ejercicio 13.20
Implementa un programa que muestre los primeros 42 números primos que existen.
Implementa tres soluciones: una con el bucle ```for```, otra con el bucle ```while``` y otra con el bucle ```loop```. ¿Cuál crees que es la opción más adecuada?

## Ejercicio 13.21
Ya sabes cómo generar números aleatorios. Escribe un programa que genere una cantidad de números aleatorios determinada por el usuario.
Recuerda utilizar Rust Playground en lugar de tu editor de código fuente local para evitar –por el momento– la instalación de paquetes. 

## Ejercicio 13.22
Solicita una frase al usuario y almacena cada palabra en un vector de Strings.
Utiliza la estructura ```while let``` para recorrer la lista hasta encontrar una palabra que contenga la letra «z». Muestra esa palabra en pantalla. Gestiona correctamente, utilizando opcionales, la posibilidad de que ninguna palabra de la frase contenga la letra «z».

## Ejercicio 13.23
Imprime en pantalla todos los múltiplos de 3 entre 4 y 42.

## Ejercicio 13.24
Suma todos los números que son divisibles entre 3 y se encuentran en el rango de 1 a 108, ambos inclusive.

## Ejercicio 13.25
Implementa un programa que solicite 10 frases por teclado, y las almacene en un vector dinámico. Cuenta el total de veces que aparece la palabra Rust y muéstralo en pantalla.

## Ejercicio 13.26
Genera 25 números aleatorios, elévalos al cuadrado y divídelos entre 7. Ten presente en todo momento los tipos de datos con los que necesitas trabajar. Implementa este programa en Rust Playground y muestra los resultados en pantalla.

## Ejercicio 13.27
Toma una referencia inmutable a cualquiera de los vectores que has creado anteriormente, recorre cada uno de sus elementos e imprímelos en pantalla desde la referencia tomada.

## Ejercicio 13.28
Crea un vector de 20 elementos de tipo ```Semáforo``` (enumerado). Cada elemento estará inicializado con diferentes colores. Cuenta el número de semáforos que están en verde y muéstralo en pantalla.

## Ejercicio 13.29
En capítulos anteriores implementaste una función que convierte una temperatura de grados Celsius a Fahrenheit.
Modifica esta función para aceptar un array de 23 temperaturas en grados Celsius y devolver un array de temperaturas en grados Fahrenheit. Elige el bucle más adecuado para recorrer el array.

## Ejercicio 13.30
Escribe un programa que solicite nombres de usuario por teclado hasta que encuentre uno que empiece por la letra «S». Utiliza ```while let``` en la implementación de tu solución.

## Ejercicio 13.31
Almacena todos los nombres del ejercicio anterior en un vector de Strings. Recórrelo para imprimir todos los nombres en orden inverso.

## Ejercicio 13.32
Recoge 50 temperaturas por teclado. Utiliza la estructura ```while let``` para detectar cuántos valores están por encima de 0º, cuántos valores se encuentran bajo 0º y cuántos valores coinciden exactamente con 0º.
Muestra la información centrada en pantalla, con un ancho de 40.

## Ejercicio 13.33
Recoge valores por teclado hasta que el usuario introduzca un número primo, y muestra en pantalla cuántos iteraciones han sido necesarias hasta introducir dicho valor.

## Ejercicio 13.34
Suma todos los números pares entre 23 y 108 y réstale al resultado el producto de todos los números impartes entre 15 y 42.

## Ejercicio 13.35
Crea un programa que, dado un número proporcionado por el usuario, muestre su tabla de multiplicar del 1 al 10.

## Ejercicio 13.36
Implementa un programa que solicite una frase al usuario y muestre en pantalla la misma frase pero con las vocales triplicadas.
Por ejemplo, la entrada «Lenguaje de programación Rust» devolvería la salida «Leeenguuuaaajeee deee prooograaamaaaciiiooon Ruuust».

## Ejercicio 13.37
Te encantaría independizarte, pero el precio de la vivienda está por las nubes, así que quieres hacer un estudio de los precios de la vivienda en tu zona.
Crea un enumerado con datos asociados capaz de almacenar tres tipos de viviendas: ```piso```, ```adosado``` y ```unifamiliar```.
Para cada tipo necesitarás almacenar la dirección en la que se encuentra la vivienda y su precio.
Crea un vector con 20 enumerados donde almacenarás la información, y calcula el precio medio para cada tipo de propiedad. Muestra el precio medio en pantalla.

## Ejercicio 13.38
Mejora el programa anterior para que solicite el tipo de vivienda preferido mediante el teclado y muestre en pantalla todas las direcciones de las viviendas de ese tipo.

## Ejercicio 13.39
Usa ```while let``` para recorrer una lista de palabras y encontrar la primera que sea un palíndromo.
