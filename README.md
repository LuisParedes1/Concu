# Channels

Seguir:


> Do not communicate by sharing memory; instead, share memory by communicating.


Reglas en Rust

- Un canal tiene dos extremos: un emisor (`tx`) y un receptor (`rx`)
- Una parte del codigo invoca metodos sobre el extremo emisor (`tx`) y otra parte del codigo chequea el extremo de recepcion (`rx`) por la existencia de mensajes. Generalmente en threads distintos
- Puedo tener multiples productores (haciendo `.clone` del transmisor) y un unico consumidor (receptor)
- A travez de canales transfiero el ownership del elemento que envio
por el canal (esta es la parte de “share memory by communicating”)

# Actores

Estructura:

- Estructura Actor
- Estructura Message
- Handle<Message> para que el actor procese el Message


# Notion

Mas info en el [Notion](https://mis-notas.notion.site/Semana-7-de79099a372f49598da4c4886eafb21e?pvs=4)
