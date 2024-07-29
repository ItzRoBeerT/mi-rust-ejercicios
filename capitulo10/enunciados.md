# Capítulo 10
* Libro «[Curso de Programación con Rust](https://amzn.to/3WEqWZJ)» (Autor: [@EliezerLopez](https://linkedin.com/in/eliezerlopez) · Grupo ANAYA).
* Cursos en video y web de soporte: [eliezerlopez.rs](https://eliezerlopez.rs).

## Ejercicio 10.01
Crea un ```String``` vacío y asígnale una cadena más tarde.

## Ejercicio 10.02
Declara un ```String``` con el contenido «Curso de programación con Rust».

## Ejercicio 10.03
Concatena 2 ```String``` y muestra el resultado.

## Ejercicio 10.04
Declara un enumerado llamado ```Color``` con variantes ```Rojo```, ```Verde``` y ```Azul```.

## Ejercicio 10.05
Crea un vector vacío de enteros.

## Ejercicio 10.06
Crea un vector vacío de ```String```.

## Ejercicio 10.07
Crea un vector con 8 números inicializados a 16 con la macro ```vec!```. Añádele los números 23 y 42.

## Ejercicio 10.08
Crea un vector con 9 caracteres. Elimina los 8 primeros caracteres.

## Ejercicio 10.09
Convierte una cadena de texto de tipo ```&str``` en un ```String```.

## Ejercicio 10.10
Convierte una cadena de texto de tipo ```String``` en un ```&str```.

## Ejercicio 10.11
Añade los números 4, 8, 15, 16, 23 y 42 a un vector vacío llamado ```valenzetti```.

## Ejercicio 10.12
Implementa un enumerado llamado ```Figura``` con las variantes ```Circulo```, ```Cuadrado``` y ```Triangulo```.

## Ejercicio 10.13
Declara una variable de tipo ```Option<i64>``` y asígnale un número grande.

## Ejercicio 10.14
Crea una variable de tipo ```Option<Vec<i8>>``` y asígnale un vector vacío.

## Ejercicio 10.15
Declara un vector de 5 elementos de tipo ```u64``` e invierte el orden de sus elementos.

## Ejercicio 10.16
Inicializa un vector de ```String``` vacío y añade 3 nombres.

## Ejercicio 10.17
Implementa un enumerado ```PermisosArchivo``` con las variantes ```Leer```, ```Escribir``` y ```Ejecutar```.

## Ejercicio 10.18
Comprueba si un vector está vacío.

## Ejercicio 10.19
Escribe tu nombre en un ```String``` e invierte el orden de las letras.

## Ejercicio 10.20
Define un enumerado ```RespuestaApi``` con las variantes ```Exito``` y ```Error(String)```. Crea variables de este tipo.

## Ejercicio 10.21
Declara una variable de tipo ```Option<bool>``` y asígnale un valor verdadero.

## Ejercicio 10.22
Crea un enumerado ```Suscripcion``` con variantes ```Mensual```, ```Trimestral```, y ```Anual```. Restringe el tipo de dato que pueden almacenar estas variantes a valores reales.

## Ejercicio 10.23
Calcula la suma de todos los elementos de un vector de 10 valores reales de tipo ```f32```. Inserta posteriormente en el vector el número ```108.0``` en el índice ```4```.

## Ejercicio 10.24
Crea 4 vectores que almacenan los 3 sueldos de una persona en cada trimestre del año.
Utiliza el método el método ```extend()``` para concatenar toda la información por orden de Enero a Diciembre. El método ```extend()``` permite concatenar un vector de alto nivel a otro de la siguiente forma ```vector1.extend(vector2)```.

## Ejercicio 10.25
Declara una variable de tipo ```Result<bool, &str>``` y asigna un valor booleano. Comprueba si contiene o no error.

## Ejercicio 10.26
Crea un vector de alto nivel y utilízalo para realizar alguna operación. Asigna el vector desde el identificador a un nuevo identificador utilizando el símbolo de igual ```=```. Vuelve a realizar operaciones con ambos identificadores. ¿Qué ocurre?

## Ejercicio 10.27
Convierte todos los caracteres de la frase «Rust, el Lenguaje de Programación» en minúsculas. Sustituye posteriormente todos los espacios por un guion bajo ```_```. 

## Ejercicio 10.28
Borra el contenido del ```String``` anterior.

## Ejercicio 10.29
Implementa una función que reciba un ```String``` y devuelva la primera letra de la cadena. ¿Qué ocurre si el ```String``` está vacío?

## Ejercicio 10.30
Elimina el primer elemento de un vector.

## Ejercicio 10.31
Define un enumerado llamado ```Impuesto``` con 5 variantes: ```ProductosTipoA```, ```ProductosTipoB```, ```ProductosTipoC```.
Crea datos asociados a estas variables para almacenar el importe del impuesto (0.15, 0.21, 0.30) y su representación escrita («15%», «21%», «30%») haciendo uso de ```String```.

## Ejercicio 10.32
Crea un vector impuestos que almacene 5 valores del enumerado ```Impuesto``` que acabas de crear.

## Ejercicio 10.33
Implementa un método ```subir_impuestos()``` para el enumerado ```Impuesto``` que asigne siempre la variante ```ProductosTipoC``` al enumerado desde el que se llama.
Úsalo para los primeros 3 productos del vector creado en el ejercicio anterior.

## Ejercicio 10.34
Añade un nuevo valor al vector impuestos (de tipo ```Impuesto```) usando el método ```push()```.

## Ejercicio 10.35
Crea un vector de tuplas que modele los resultados de un partido de futbol.

## Ejercicio 10.36
Declara una variable de tipo ```Result<u16, &str>``` y asigna un número negativo.

## Ejercicio 10.37
Una matriz no es más que un vector bidimensional que puedes crear de la siguiente forma: ```let matriz: Vec<Vec<tipo>> = vec![vec![0; columnas]; filas];```.
Crea una matriz de 3x4 que almacene valores de tipo entero. Toma el que se encuentra en la posición ```[1, 2]```, súmalo al que se encuentra en la posición ```[2, 3]``` y almacena el resultado en una nueva variable.

## Ejercicio 10.38
Declara una variable de tipo ```Option<&str>``` y asígnale una referencia a una subcadena.

## Ejercicio 10.39
Los métodos ```len()``` y ```chars().count()``` se utilizan con mucha frecuencia a la hora de trabajar con cadenas. En el siguiente programa, ¿por qué ```longitud1``` recibe el valor 16 y ```longitud2``` el valor 17?
```
fn main(){
    let cadena = String::from("Mañanas con Rust");
    
    let longitud1 = cadena.chars().count();
    let longitud2 = cadena.len();
    
    println!("Longitud 1: {}", longitud1);
    println!("Longitud 2: {}", longitud2);
}
```
