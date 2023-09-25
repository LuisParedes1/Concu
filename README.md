# Arquitectura Cliente-Servidor

El Cliente es aquel que solicita request.

El Servidor es aquel que responde a los request

**Working**: A request is sent from client to server in the form of a web URL as HTTP GET or POST or PUT or DELETE request. After that, a response comes back from the server in the form of a resource which can be anything like HTML, XML, Image, or JSON. But now JSON is the most popular format being used in Web Services.

![Cliente-Servidor](image.png)

# API REST

- GET: The HTTP GET method is used to read (or retrieve) a representation of a resource.
- POST: The POST verb is most often utilized to create new resources. In particular, it’s used to create subordinate resources. That is, subordinate to some other (e.g. parent) resource.
- PUT: It is used for updating the capabilities.
- PATCH: It is used to modify capabilities.
- DELETE: It is used to delete a resource identified by a URI.

Mas info [aca](https://www.geeksforgeeks.org/rest-api-introduction/)

Response Codes [aca](https://httpstatusdogs.com/)

# Notion

Mas info sobre REST (en React) [aca](https://mis-notas.notion.site/REST-6fcee850bc4640148cdb148f85fe7936?pvs=4)

# Notas

`block_on` sirve como adaptador entre el mundo async y sync.

Para poder hacer llamado a una funcion `async` desde una `sync` se debe usar `block_on` ya que una funcion `async` siempre devuelve un Future (asi tenga await adentro)

---

Debido a que cada `block_on` realiza un `move` entonces el cliente deberia crearse al inicio de la funcion `async` y luego hacer los multiples requests.

Es decir, cuando termina la funcion `block_on`, el cliente creado se dropea y se pierde la conexion establecida.
