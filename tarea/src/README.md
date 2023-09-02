# Implementar WordCount usando Fork-Join

Contar la cant de veces que se repite cada palabra en el archivo dado.

* Map 
    * Tomar de potencialmente varios archivos de texto, las lineas
    * De las lineas, las palabras
    * De las palabras sus frecuencias locales

* Reduce
    * Combinar las frecuencias locales en una unica tabla con todas las frecuencias