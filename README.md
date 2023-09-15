# Semana 3

Programacion Asincronica

Util para: 

* Cuando creo muchos threads tal que la memoria ocupada por ellos es significativa. Se puede usar Tareas asincrónicas de Rust para intercalar tareas en un  thread o en un pool de threads.
* Reducen el overhead de memoria ya que son mucho mas liveanas que los threads.
* Las tareas son más rápidas de crear, más eficiente de pasarle el control a ellas.

No util para:
* Computo Intensivo
* Mucho computo y poco I/O
* No me preocupa la cant de mem ocupada (de threads)


Mas info en el siguiente [Notion](https://mis-notas.notion.site/Semana-3-740b2a0ff9554b6ca3aae6b93da3dd44?pvs=4)

# Tarea

Realizar un pequeño utilitario que utilizando el crate reqwest imprima por pantalla los 10 tweets mas destacados de cada uno de los trending topics de Argentina usando las APIs de Twitter.

- No pude acceder a la API de Twitter asi que utilice la API de Spotify. Para una lista de artistas pedi sus canciones mas populares asincronicamente.

Ver codigo [aca](./tarea/)