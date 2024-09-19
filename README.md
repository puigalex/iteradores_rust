# Tarea 2 Iteradores
En esta tarea se implementan iteradores para recorrer vectores de datos para obtener otro iterador el cual solo contiene valores los cuales son mayores a todos los datos que tienen a su izquierda. Incluido hay dos proyectos:
- iteradores: Contiene la implementación de los iteradores como se menciona en la tarea.
- cli_iteradores: Es el mismo codigo y funcionalidad con la diferencia de la implementación de clap para recibir argumentos por medio de consola.

## Comandos para ejecutar:
### iteradores
```bash
   cargo run
```

### cli_iteradores
```bash
cargo run -- --textos foo bar zoo art
```

```bash
cargo run -- --numeros 1 2 4 2 1 5 0
```

El codigo de la tarea utiliza corrutinas para implementar los iteradores basandose en el repositorio de la clase [repo](https://git.da.vidflor.es/dflores/test_generators.git
