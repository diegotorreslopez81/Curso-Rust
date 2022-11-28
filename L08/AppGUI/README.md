## Ejercicio Final de Curso Introductorio a Rust
Como actividad formativa final para el Curso Introducción a Rust, se solicita desarrollar una aplicación de escritorio empleando el lenguaje Rust que tenga las siguientes funcionalidades:

- Incluir, Modificar, Eliminar, Consultar y Buscar una Entidad de Tipo Vivienda que tenga los siguientes atributos:
Calle, número, piso, código postal, metros cuadrados, cantidad de baños, cantidad de habitaciones, tipo de vivienda (Apartamento, Casa, …) (Usar un enumerado para representar el valor de este atributo)

- Los datos deben almacenarse en un archivo de CSV. 
- Se pueden utilizar los Crates de tercero para desarrollar la asignación.
- Se sugiere utilizar la aplicación entregada como ejemplo para la Lección 08 que incorpora unas funcionalidades similares.
- Para la parte de interfaz de usuario se sugiere utilizar las librerías FLTK. El participante es libre de seleccionar el tipo de librería gráfica que más le convenga.

### Importante
Este proyecto no es una actividad obligatoria para el curso, pero si se desea obtener un certificado de aprobación es necesario desarrollarlo en el tiempo que considere conveniente y hacerla llegar por correo electrónico a ramon.valera@gmail.com o a polkadothub@gmail.com .


### Getting Start

Primero de todo, configura tu entorno para aplicaciones gráficos. En caso de usar windows, sigue este tutorial: https://www.shogan.co.uk/how-tos/wsl2-gui-x-server-using-vcxsrv/

Una vez configurado y arrancado el servicio, ejecuta los siguientes comandos para arrancar el proyecto:

```bash
$ git clone git@github.com:diegotorreslopez81/Curso-Rust.git
$ cd .\Curso-Rust\L08\AppGUI\
$ cargo build
$ cargo run
```

Ahora ya puedes utilizar la aplicación:

![Screenshot](/images/screenshot.png)

### Funciones
- Crear: si rellenas los campos de la derecha, puedes crear un registro.
- Modificar: Si clicas en un registro, lo puedes modificar en los campos que se cargan a la derecha de la pantalla.
- Borrar: Si seleccionas un registro, lo puedes borrar.
- Filtro: Puedes filtrar por el ID del registro.
- Guardar fichero: Y no olvides "guardar" antes de acabar, para modificar los datos en el fichero JSON.
