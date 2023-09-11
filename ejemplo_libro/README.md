# Ejemplo Cap 20

This code runs all three requests concurrently from within the call to block_on. Each one makes progress as the opportunity arises while the others are blocked, all on the calling thread.

Ejemplo sacado de Programming Rust 2nd Edition. Pag 926