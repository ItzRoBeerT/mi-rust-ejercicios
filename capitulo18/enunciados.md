
# Capítulo 18
* Libro «[Curso de Programación con Rust](https://amzn.to/3WEqWZJ)» (Autor: [@EliezerLopez](https://linkedin.com/in/eliezerlopez) · Grupo ANAYA).
* Cursos en video y web de soporte: [eliezerlopez.rs](https://eliezerlopez.rs).

## Ejercicio 18.01
Define un struct genérico llamado ```Wrapper``` que contenga un solo elemento de cualquier tipo.

## Ejercicio 18.02
Implementa un método genérico en ```Wrapper``` que imprima el elemento.

## Ejercicio 18.03
Escribe una función genérica ```indice_de()``` que devuelva el índice de un valor contenido en un vector.

## Ejercicio 18.04
Implementa una función genérica que reproduzca el comportamiento de un slice y devuelva una sección de un vector.

## Ejercicio 18.05
Escribe una función genérica ```combinar()``` que combine 2 vectores ordenados en uno ordenado.

## Ejercicio 18.06
Verifica si un vector contiene un valor mediante la implementación de una función genérica llamada ```contiene()```.

## Ejercicio 18.07
Implementa una función genérica ```filtrar()``` que filtre elementos de un vector según una condición establecida.

## Ejercicio 18.08
Escribe una función genérica ```concatenar()``` que reciba 2 vectores y los concatene. ¿Qué ocurre si cada vector almacena un tipo de dato diferente?

## Ejercicio 18.09
Implementa una operación genérica de ```suma()``` que sea capaz de sumar cualquier combinación de dos valores de tipo ```i32``` y ```f64```.

## Ejercicio 18.10
Escribe una operación genérica de ```resta()``` que sea capaz de restar cualquier combinación de dos valores de tipo ```i32``` y ```f64```.

## Ejercicio 18.11
Codifica una operación genérica de ```producto()``` que sea capaz de multiplicar cualquier combinación de dos valores de tipo ```i32``` y ```f64```.

## Ejercicio 18.12
Implementa una operación genérica de ```division()``` que sea capaz de dividir cualquier combinación de dos valores de tipo ```i32``` y ```f64```. Devuelve el cociente y el resto encapsulados en un tipo de dato llamado ```ResultadoDivision```. Contempla con opcionales la posibilidad de que la operación de división y módulo no puedan realizarse.

## Ejercicio 18.13
Implementa una calculadora genérica. Solicita dos valores al usuario del programa y pregunta qué operación desea realizar. En base a su respuesta, utiliza alguna de las funciones genéricas implementadas en ejercicios anteriores.

## Ejercicio 18.14
Escribe una función genérica ```max()``` que devuelva el mayor de 2 valores.

## Ejercicio 18.15
Toma como base la función implementada en el ejercicio anterior e implementa una nueva función ```min()``` que devuelve al menor valor de 2 proporcionados.

## Ejercicio 18.16
Implementa una función genérica ```intercambiar()``` que reciba 2 variables e intercambie sus valores. Recuerda que deberás trabajar con referencias mutables.

## Ejercicio 18.17
Crea un enumerado genérico llamado ```Resultado``` que replique el comportamiento del enumerado ```Result```. Incluye las variantes ```Exito``` y ```Fallo```.

## Ejercicio 18.18
Define un método en el enumerado ```Resultado``` que verifique si es ```Exito```.

## Ejercicio 18.19
Implementa un método en el enumerado ```Resultado``` para verificar si es ```Fallo```.

## Ejercicio 18.20
Define un enumerado genérico ```Comando``` que represente diferentes comandos de Cargo.

## Ejercicio 18.21
Implementa un método para el enumerado genérico ```Comando``` que simule la ejecución del comando.

## Ejercicio 18.22
Define un struct genérico ```Rango``` que represente un rango de valores.

## Ejercicio 18.23
Crea un struct genérico llamado ```Matriz2D``` que modele una matriz de 2 dimensiones.

## Ejercicio 18.24
Implementa un método para multiplicar 2 matrices de tipo ```Matriz2D```.

## Ejercicio 18.25
Define un struct genérico ```Historial``` que mantenga un historial de cambios de un valor.

## Ejercicio 18.26
Define un método en ```Historial``` para registrar un nuevo valor.

## Ejercicio 18.27
Define un método que muestre el historial de cambios registrados en una instancia de tipo ```Historial```.

## Ejercicio 18.28
Implementa un struct genérico ```Tabla``` que almacene datos en formato tabla.

## Ejercicio 18.29
Añade a ```Tabla``` un método para agregar una fila.

## Ejercicio 18.30
Incluye en ```Tabla``` un método para insertar una columna.

## Ejercicio 18.31
Una tabla debe ser mostrada en pantalla con el formato de tabla correspondiente. Prepara tu struct genérico para poder mostrar el contenido de una tabla de la siguiente forma.
```
fn main(){

   // Declaración y definción de la variable "tabla"

   println!("{}", tabla);

}
```
## Ejercicio 18.32
Define un struct genérico ```Configuraciones``` que almacene configuraciones de diferentes tipos.

## Ejercicio 18.33
Implementa un método en ```Configuraciones``` para actualizar una configuración.

## Ejercicio 18.34
Define un enumerado genérico llamado ```Opciones``` con las variantes ```Primera```, ```Segunda``` y ```Tercera```.

## Ejercicio 18.35
Implementa una función genérica ```media()``` que calcule la media de un conjunto de valores sin importar su tipo. La función debe recibir como parámetro de entrada un vector de alto nivel.
