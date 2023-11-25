# Semana 11 Concurrencia Distribuida (parte 3): Transacciones

- Two Phare Algorithm

1. Prepare: El coordinador le envia a todos para que reserven recursos y cuando todos le envian Ok pasa a la fase de commit
2. Commit: El coordinador hace los cambios

Si cualquier proceso (incluido el coordinador) hace abort entonces se cancela la transaccion y se revierte al estado previo

[Notion](https://mis-notas.notion.site/Semana-11-3461eb849d40490bae3698871777805e?pvs=4)

# Temas

- Langs (Testing)
- Testing (Mocks)
