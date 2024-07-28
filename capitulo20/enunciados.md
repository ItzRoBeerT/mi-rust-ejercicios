# Capítulo 20
* Libro «[Curso de Programación con Rust](https://amzn.to/3WEqWZJ)» (Autor: [@EliezerLopez](https://linkedin.com/in/eliezerlopez) · Grupo ANAYA).
* Cursos en video y web de soporte: [eliezerlopez.rs](https://eliezerlopez.rs).

## Ejercicio 20.01
¿Estas dos funciones son equivalentes?

Función 1:
```
fn amplia_tus_conocimientos() -> &'static str {

    let mensaje = "Visita eliezerlopez.rs y amplía tus conocimientos de C, C++ y Rust con nuevos cursos que año tras año preparo para ti.";

    &mensaje
}
```

Función 2:
```
fn amplia_tus_conocimientos() -> &'static str {

    "Visita eliezerlopez.rs y amplía tus conocimientos de C, C++ y Rust con nuevos cursos que año tras año preparo para ti."
}
```

¿Qué implicaciones tiene cada una de ellas? ¿Cuál de las dos opciones es más adecuada?

## Ejercicio 20.02
Crea un struct ```Direccion``` con anotaciones de ciclo de vida que encapsule los campos ```calle```, ```ciudad``` y ```codigo_postal```.

## Ejercicio 20.03
Implementa una función que acepte 2 referencias a enteros con anotaciones de ciclo de vida, y devuelva la referencia al mayor de los dos.

## Ejercicio 20.04
Crea una estructura ```Libro``` con anotaciones de ciclo de vida que contenga una referencia al título y al autor.

## Ejercicio 20.05
Define un struct ```Persona``` con anotaciones de ciclo de vida que contenga una referencia a su nombre y otra a su edad.

## Ejercicio 20.06
¿Tiene sentido referenciar la edad en el ejercicio anterior?

## Ejercicio 20.07
Implementa una función que tome una referencia a un vector de enteros y devuelva una referencia al primer elemento.

## Ejercicio 20.08
¿Te ha pedido el compilador que incluyas anotaciones de ciclo de vida en el ejercicio anterior? ¿Por qué motivo?

## Ejercicio 20.09
Define un struct ```Rectangulo```. Incluye en su interior referencias a sus vértices.

## Ejercicio 20.10
Implementa la una estructura ```Fecha```. ```Fecha``` debe almacenar en su interior referencias al día, mes y año. Implementa anotaciones de ciclo de vida si es necesario.

## Ejercicio 20.11
Crea un struct ```Coche``` que contenga referencias a la marca y al modelo.

## Ejercicio 20.12
Desarrolla una función que tome 2 referencias a 2 cadenas de tipo ```String``` y devuelva una referencia a la cadena más larga.

## Ejercicio 20.13
Implementa un struct ```Telefono```. En su interior debe contener referencias a su número y propietario.

## Ejercicio 20.14
Duplica tu gestor de empleados en un proyecto nuevo para hacer una serie de cambios. Trata de convertir todos los campos de las diferentes estructuras que has creado a referencias y adapta el resto del código. Te recomiendo que te prepares un buen café y reserves varias horas para resolución de este ejercicio.

## Ejercicio 20.15
Implementa un struct llamado ```Ciudad``` que almacene en su interior, a través de referencias, su nombre y su número de habitantes.

## Ejercicio 20.16
¿Estás utilizando las estructuras y funciones implementadas en los ejercicios anteriores para comprobar que todo compila y funciona correctamente? Vuelve atrás y asegúrate de que, hasta ahora, todo compila y se ejecuta.

## Ejercicio 20.17
Desarrolla la estructura ```Producto``` con los campos ```nombre``` y ```precio``` en su interior. Estos campos deben ser referencias a los valores originales.

## Ejercicio 20.18
Crea un struct llamado ```Direccion``` para almacenar la dirección de una persona. A continuación, implementa otro struct llamado ```Persona``` con los campos ```nombre``` (de tipo ```&String```) y ```direccion``` (de tipo ```&Direccion```). Instancia el tipo ```Persona``` y utilízalo desde la función principal.

## Ejercicio 20.19
Implementa un programa que llame a la función ```cadena_con_mas_caracteres()```. Esta función debe recibir un vector de ```String``` como parámetro de entrada, y debe devolver una referencia a la cadena con mayor número de caracteres.

## Ejercicio 20.20
Implementa el struct ```Ordenador``` con los campos ```procesador``` y ```memoria_ram```. Estos campos deben ser referencias a los valores originales. Crea un vector de 10 elementos de tipo ```Ordenador``` en la función principal ```main()```, y trata de mostrar en pantalla toda la información almacenada.

## Ejercicio 20.21
Define una función que tome una referencia a un vector de enteros y devuelva una referencia al primer elemento.

## Ejercicio 20.22
Implementa una función que reciba como parámetro de entrada una referencia a un vector de números flotantes y retorne una referencia al valor más cercano a la media.

## Ejercicio 20.23
Define un struct ```Aeropuerto``` que almacene en su interior referencias al nombre del aeropuerto y su código. Por ejemplo, el aeropuerto de Jerez de la Frontera se llama «Aeropuerto Internacional de Jerez La Parra» y su código es «XRY».

## Ejercicio 20.24
Implementa una función que reciba una variable de tipo ```String``` como parámetro de entrada y devuelva si longitud por referencia.




