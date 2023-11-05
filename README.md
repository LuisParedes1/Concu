# Channels

Seguir:

> Do not communicate by sharing memory; instead, share memory by communicating.

Reglas en Rust

- Un canal tiene dos extremos: un emisor (`tx`) y un receptor (`rx`)
- Una parte del codigo invoca metodos sobre el extremo emisor (`tx`) y otra parte del codigo chequea el extremo de recepcion (`rx`) por la existencia de mensajes. Generalmente en threads distintos
- Puedo tener multiples productores (haciendo `.clone()` del transmisor) y un unico consumidor (receptor)
- A travez de canales transfiero el ownership del elemento que envio por el canal (esta es la parte de “share memory by communicating”)

# Modelo de Actores

Actores es la primitiva principal del modelo de actores. Encapsulan comportamiento y estado.

Esta compuesta por

- Direccion: adonde enviarle mensajes
- Casilla de correos (mailbox): un FIFO de los ultimos mensajes recibidos

El actor supervisor (actor especial) puede crear otros actores hijo.

**Actores**

- La unica forma de cambiar el estado a travez de un mensaje
- Los actores se comunican solamente usando mensajes
- Maneja un mensaje a la vez
- Son aislados los actores(no comparten memoria)
- La unica manera de referenciar a un actor es a travez de su direccion

**Mensajes**

- Los mensajes son procesados por los actores de manera asincronica (se acumulan en el mailbox y el actor receptor los va sacando y procesando)
- Los mensajes son estructuras simples inmutables

## Framework `Actix`

Actores en Rust, `Actix` requiere:

- Estructura `MyActor` que implementa el trait `Actor`(unicamente referenciados por direccion)
- Estructura `Ping` que implementa el trait `Message` (estructuras inmutables)
- Implementar `Handler<Ping>` para ese tipo de mensaje de manera que el actor sepa como procesar el `Message`

### Maneras de mandar mansajes (en artix)

- `addr.do_send(Message)`: ignora errores en el envio de mensajes. Si la casilla de mensajes esta cerrada, se descarta. No retorna el resultado.
- `add.try_send(Message)`: Trata de enviar el mensaje inmediatamente. Si la casilla de mensajes esta llena o cerrada, retorna _`SendError`_
- `addr.send(Message)`: retorna un objeto _`future`_ que contiene el \*\*resultado del procesamiento del mensaje por parte del otro actor

# Notion

Mas info en el [Notion](https://mis-notas.notion.site/Semana-7-de79099a372f49598da4c4886eafb21e?pvs=4)
