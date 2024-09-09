# Curso de Programación con Rust - Repositorio de Ejercicios

![libro-rust-anaya-eliezer-lopez-min](https://github.com/user-attachments/assets/43d38891-ae1f-4678-83f4-2f02951f937f)

Este es el repositorio de ejercicios del libro `Curso de Programación con Rust` de ANAYA Multimedia. Puedes adquirir un ejemplar del libro en [aquí](https://amzn.to/3ZlpQE9). Si necesitas ayuda para localizar el libro con envío a tu país, [contacta con el autor](https://eliezerlopez.carrd.co).

## Estructura

Solo aprenderás Rust si te enfrentas al compilador y aprendes a aplicar cada uno de los conceptos propios del lenguaje. Por eso he decidido, a la manera clásica de los libros técnicos, no proporcionar las soluciones de los ejercicios, instándote así a trabajar a fondo cada uno de ellos, depurando y comprendiendo su código con la ayuda del compilador. No obstante, es importante compartir diferentes enfoques en las soluciones. Por ello he preparado este repositorio en el que otros lectores como tú podrán subir sus soluciones y compartirlas.

Dentro de cada carpeta encontrarás un listado de propuestas con ejercicios en un archivo llamado `enunciados.md` –que te ayudarán a reforzar lo aprendido en cada capítulo del libro– junto a las soluciones proporcionadas diferentes miembros de la comunidad. Por ejemplo, encontrarás la solución propuesta por `Moss` para el ejercicio `08.18` en la ruta `capitulo08/08_18_moss.rs`.

## Cómo contribuir con tu solución

1. Haz un `fork` de este repositorio.
2. Ve a la carpeta correspondiente al capítulo y crea un archivo con extensión `.rs`por cada solución que quieras aportar.
El nombre del archivo debe seguir esta estructura: `<capitulo>/<numero_ejercicio>_<tu_usuario_de_github>.rs`.
Por ejemplo, si estás resolviendo el ejercicio 3 del capítulo 7 y tu nombre de usuario en GitHub es `Moss`, el archivo debe ubicarse en `capitulo07/07_03_moss.rs`.
Si necesitas subir varios archivos para una misma solución, entonces crea una subcarpeta con la misma estructura e incluye dentro del conjunto de archivos. 
3. Haz una Pull Request` con tu solución.

### Cómo enviar una Pull Request
1. Clona el repositorio de tu fork a tu máquina local.
```
git clone https://github.com/tu_usuario/mi-rust-ejercicios.git
cd mi-rust-ejercicios
```
2. Crea una nueva rama para trabajar en tu solución. Sustituye `XX` por el capítulo e `YY` por el ejercicio correspondiente.
```
git switch -c solucion-capitulo-XX-ejercicio-YY
```
3. Añade tu archivo `.rs` en la carpeta correcta siguiendo la estructura descrita previamente.
4. Guarda los cambios y súbelos a tu repositorio:
```
git add .
git commit -m "Añadir solución para el ejercicio XX del capítulo YY"
git push origin solucion-capitulo-XX-ejercicio-YY
```
5. Ve a tu fork en GitHub y verás un botón `Compare & pull request`. Haz clic en él y completa el formulario para crear la Pull Request.

Una vez creada la Pull Request, será revisada y recibirás comentarios si es necesario hacer alguna corrección.
