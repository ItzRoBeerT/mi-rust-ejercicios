# Capítulo 15
* Libro «[Curso de Programación con Rust](https://amzn.to/3WEqWZJ)» (Autor: [@EliezerLopez](https://linkedin.com/in/eliezerlopez) · Grupo ANAYA).
* Cursos en video y web de soporte: [eliezerlopez.rs](https://eliezerlopez.rs).

## Ejercicio 15.01
Verifica que cargo está instalado en tu sistema.

## Ejercicio 15.02
Crea un proyecto en cargo que muestre el siguiente refrán en pantalla: «Llevar agua a la mar, eso sería desvariar».

## Ejercicio 15.03
Modifica tu proyecto anterior y publica un refrán aleatorio cada vez que el programa se ejecute.

## Ejercicio 15.04
Escribe un buen manifiesto para el programa anterior. Incluye todos los metadatos necesarios para que tu proyecto esté listo para ser compartido con la comunidad.

## Ejercicio 15.05
Elimina una dependencia de tu generador de refranes y trata de compilar. Analiza el mensaje de error que te aparece.
⚠️ **Advertencia**: Si no has utilizado al menos una dependencia para desarrollar tu generador aleatorio de refranes, te recomiendo repetir el ejercicio 15.3.

## Ejercicio 15.06
Clippy es una herramienta que ayuda a encontrar posibles errores y mejorar la calidad del código.
Verifica si Clippy está instalado en tu equipo con el comando ```rustup component list | grep clippy```. En caso contrario, puedes instalarlo ejecutando ```rustup component add clippy```.
Finalmente, ejecuta Clippy desde la terminal (dentro de tu generador de refranes aleatorios) con el comando ```cargo clippy``` y revisa e implementa sugerencias de cambios recomendadas.

## Ejercicio 15.07
La herramienta ```fmt``` formatea tu código de Rust de acuerdo con las convenciones de estilo oficiales del lenguaje. Comprueba si ```fmt``` está ya en tu equipo e instálalo si no lo está.

## Ejercicio 15.08
Implementa un programa que solicite 2 números por teclado, realice las operaciones de suma, resta, multiplicación, división y módulo (si aplica) y muestre los resultados formateados al centro.
Controla los posibles errores derivados por división entre cero y operaciones con números negativos.

## Ejercicio 15.09
Vuelve al capítulo 13 y convierte en proyecto el ejercicio 13.03. Escribe un manifiesto que incluya toda la información que veas oportuna.

## Ejercicio 15.10
Visita [Crates.io](https://crates.io) e incluye en tu proyecto la octava dependencia más descargada. Incluye en el manifiesto una versión más antigua, guarda los cambios y descarga las dependencias. Observa el archivo ```cargo.lock```.

## Ejercicio 15.11
Ejecuta el comando de cargo más adecuado para actualizar las dependencias del proyecto con el que trabajaste en el ejercicio anterior. Echa de nuevo un vistazo al archivo ```cargo.lock``` y observa qué modificaciones se han producido.

## Ejercicio 15.12
Crea un programa que solicite el valor de la temperatura actual de forma indefinida.
En cada interacción, muestra en pantalla la temperatura mínima registrada, la temperatura máxima registrada, y un número aleatorio generado al momento cuyo rango se encuentra entre las temperaturas mínima y máxima registradas.

## Ejercicio 15.13
Rescata el ejercicio 13.12 y convierte tu programa en un proyecto.

## Ejercicio 15.14
Implementa un proyecto en Rust que genere una cantidad de números aleatorios determinada por el usuario.
Esta cantidad debe ser introducida por teclado y los números aleatorios generados deben ser serializados a JSON. Intenta gestionar las dependencias utilizando únicamente Rust Analyzer.

## Ejercicio 15.15
Accede a [Crates.io](https://crates.io) e investiga para qué sirven los últimos 3 crates publicados en el registro oficial de paquetes de Rust.

## Ejercicio 15.16
Visita [Crates.io](https://crates.io) y trata de localizar algún crate que te permita tomar un ```String``` y escribir su contenido en un archivo de texto.
Vuelve al programa implementado en el ejercicio 13.22, transfórmalo a proyecto y utiliza el crate que has encontrado para registrar en un archivo de texto la frase introducida por el usuario.

## Ejercicio 15.17
Quieres aportar tu granito de arena a la comunidad, y has pensado en implementar un programa que realice todo tipo de operaciones matemáticas.
Revisa todas las funciones que has implementado entre los capítulos 3 y 14, y crea un nuevo proyecto que las incluya.
Implementa un menú que pregunte al usuario qué operación desea realizar, y solicite los valores necesarios en función de su respuesta.
Muestra el resultado de la operación en pantalla y vuelve a mostrar de nuevo el menú para preguntar por la siguiente operación. El programa finaliza cuando el usuario escribe -1.

## Ejercicio 15.18
La biblioteca ```regex``` para trabajar con expresiones regulares está disponible en el registro oficial de paquetes de Rust.
Crea un proyecto que solicite al usuario del programa una dirección de correo electrónico. El programa deberá utilizar ```regex``` para verificar si el correo electrónico es válido o no, y mostrar el resultado de la comprobación en pantalla.
