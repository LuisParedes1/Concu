# Semana 10 Concurrencia Distribuida (parte 2): Algoritmos de Elección del Lider

- En todos los casos va a ganar el proceso que tenga el mayor process ID
- Cualquier proceso puede empezar un proceso de eleccion
- El proceso de eleccion depende del algoritmo que usemos

  - Algoritmo Bully: Se envia mensajes a los procesos con process ID mayor y le respondo OK a los procesos con ID menor al mio.
  - Algoritmo Ring: Da la vuelta entera al ring y cada proceso carga su process ID

  Al determinar el coordinador, se envia un mensaje broadcast para que todos los nodos esten al tanto cual es el proceso lider

[Notion](https://mis-notas.notion.site/Semana-10-f173fe185b754ad0aa8e16ff02f897c8?pvs=4)
