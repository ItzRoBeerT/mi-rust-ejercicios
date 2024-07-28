
# Capítulo 17
* Libro «[Curso de Programación con Rust](https://amzn.to/3WEqWZJ)» (Autor: [@EliezerLopez](https://linkedin.com/in/eliezerlopez) · Grupo ANAYA).
* Cursos en video y web de soporte: [eliezerlopez.rs](https://eliezerlopez.rs).

## Ejercicio 17.01
Modifica los módulos que hemos implementado a lo largo de este capítulo y establece la visibilidad a nivel de crate en aquellos ítems que no serán utilizados fuera del crate.
Comprueba que tras cambiar la opción de visibilidad el programa compila y se ejecuta correctamente.

## Ejercicio 17.02
Declara una variable estática mutable ```CONTADOR``` (de tipo ```u32```) e implementa una función ```incrementar_contador()``` que la modifique.

## Ejercicio 17.03
Si te piden que te tires por un puente sin paracaídas… ¿lo haces? ¡Espero que no hayas implementado el ejercicio anterior!

## Ejercicio 17.04
Declara una variable estática ```CONFIG``` de tipo ```Option<String>``` e inicialízala con el valor ```None```. Inclúyela en el módulo ```configuracion.rs``` y trata de acceder a ella desde ```main()```.

## Ejercicio 17.05
Define un módulo ```servidor``` con la palabra reservada ```mod```.

## Ejercicio 17.06
Crea una función pública ```arrancar()``` dentro del módulo ```servidor```.

## Ejercicio 17.07
Define un submódulo ```configuración``` dentro del módulo ```servidor``` con mod.

## Ejercicio 17.08
Crea una función pública ```cargar()``` dentro del submódulo ```configuración```.

## Ejercicio 17.09
Crea un módulo llamado ```red``` como un archivo ```red.rs```. Define en su interior una función y llámala desde ```main.rs```.

## Ejercicio 17.10
Crea un módulo llamado ```matemáticas``` con una función pública ```sumar()```.

## Ejercicio 17.11
Crea un submódulo ```geometria``` dentro del módulo ```matematicas``` e incluye una función pública ```calcular_area()```.

## Ejercicio 17.12
Utiliza desde ```main()``` las funciones ```sumar()``` y ```calcular_area()``` definidas en el ejercicio anterior.

## Ejercicio 17.13
Completa toda la documentación relativa al módulo ```empleado.rs``` de tu gestor de empleados.

## Ejercicio 17.14
Convierte el módulo ```accionista``` en un módulo de archivo, e implementa los métodos del trait ```Informe``` para el tipo ```Accionista```. Resuelve todos los conflictos relacionados con la visibilidad de los elementos.

## Ejercicio 17.15
Estabas tan centrado en resolver los errores de compilación en el ejercicio anterior que te has olvidado de documentar el código. Échale un vistazo, anda…

## Ejercicio 17.16
Implementa un menú en la función principal que utilice métodos implementados en los módulos ```empleado.rs``` y ```accionista.rs```.

## Ejercicio 17.17
Sería genial ver cómo llevas hasta el momento tu gestor de empleados. Accede a [eliezerlopez.rs](https://eliezerlopez.rs) para contactar conmigo y cuéntamelo.

## Ejercicio 17.18
El reloj marca las 22h 35m mientras escribo estos ejercicios. Implementa un módulo llamado ```reloj``` que incluya tipos personalizados que permitan modelar un reloj digital.
*	El módulo debe incluir la estructura ```Hora```, la estructura ```Reloj``` y métodos para manipular y obtener la hora.
*	La estructura ```Hora``` dispondrá de los campos ```horas```, ```minutos``` y ```segundos``` con los métodos ```nueva()``` para crear una nueva instancia de ```Hora``` y ```mostrar()``` para imprimir la hora en formato ```HH:MM:SS```.
*	La estructura ```Reloj``` tendrá un campo llamado ```hora_actual``` de tipo ```Hora```, y métodos para incrementar la hora actual (```incrementar_hora()```, ```incrementar_minutos()```, ```incrementar_segundos()```) y ```mostrar()``` para imprimir la hora en formato ```HH:MM:SS```.
*	Define un trait que incluya el método ```mostrar()``` e impleméntalo en las estructuras ```Hora``` y ```Reloj```.
*	Controla la visibilidad. Los campos de las estructuras deben ser privados. Proporciona métodos públicos para acceder y modificar estos campos.
*	Utiliza todo lo que has implementado desde la función principal ```main()```.

## Ejercicio 17.19
Si has llegado hasta aquí, quiero darte la enhorabuena personalmente. Búscame en redes sociales como ```@EliezerLopez``` y cuéntame un chiste que me haga reír. Incluye el código ```#rust1719``` en tu mensaje.
