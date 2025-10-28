# 1876 - Rabiola

## Problema

Cada año, los residentes de la ciudad de Pipacicaba organizan el campeonato municipal de cometas. En este lugar, usan un tipo especial de enredadera para usar como Rabiola de la cometa. La enredadera está formada por una sola hebra formada por hojas normales y hojas adherentes. Las hojas normales están representadas por la letra 'o' y las hojas adherentes por la letra 'x'. 

Para aprovechar los diversos colores de las enredaderas, cada cometa del campeonato puede usar solo una sola cinta. Cada hoja adhesiva se pega en un solo punto de la base de la cometa. Con esto, se forman filetes de hojas normales. Los dos filetes más externos normalmente no tienen dobleces. Los filetes internos, siempre formados por un número par de hojas, están doblados.

**Ejemplo de enredadera:** `ooxooooooxo`

Después de pegar las hojas adhesivas, la Rabiola se ve así con los filetes formados.

**Entrada:**
- Múltiples casos de prueba
- Cada caso: una palabra compuesta solo por las letras 'o' y 'x' que representa una enredadera
- La palabra tiene no más de 100 caracteres
- La entrada termina con el fin del archivo

**Salida:**
- Para cada caso de prueba, imprimir un entero N que es el tamaño del filete de Rabiola más grande formado por esta enredadera

**Ejemplos:**
```
Entrada: oxooooxo    → Salida: 2
Entrada: ooxooooooxo → Salida: 3
Entrada: oooxooooxo  → Salida: 3
```

## Análisis del Problema

**Comprensión:**
1. Una enredadera consiste en hojas normales ('o') y hojas adhesivas ('x')
2. Las hojas adhesivas se pegan a la cometa, dividiendo la enredadera en segmentos
3. Los segmentos son las secuencias de 'o' entre las 'x'
4. Los segmentos externos (primero y último) NO se doblan
5. Los segmentos internos SE DOBLAN (por la mitad)

**Regla de doblado:**
- Segmentos externos: tamaño = número de 'o'
- Segmentos internos: tamaño visible = número de 'o' / 2 (se doblan)

**Ejemplo `ooxooooooxo`:**
```
Segmentos: [oo] [oooooo] [o]
           └─┘   └────┘   └┘
         externo interno externo

Segmento 1 (externo): 2 hojas → tamaño = 2
Segmento 2 (interno): 6 hojas → tamaño = 6/2 = 3 (doblado)
Segmento 3 (externo): 1 hoja → tamaño = 1

Máximo: 3
```

**Ejemplo `oxooooxo`:**
```
Segmentos: [o] [oooo] [o]
           └┘  └──┘   └┘
        externo interno externo

Segmento 1 (externo): 1 hoja → tamaño = 1
Segmento 2 (interno): 4 hojas → tamaño = 4/2 = 2 (doblado)
Segmento 3 (externo): 1 hoja → tamaño = 1

Máximo: 2
```

## Enfoque de Solución

1. Dividir la enredadera en segmentos (secuencias de 'o' entre 'x')
2. Identificar cuáles son segmentos externos (primero y último) y cuáles internos
3. Para cada segmento:
   - Si es externo: tamaño = longitud del segmento
   - Si es interno: tamaño = longitud del segmento / 2
4. Retornar el máximo tamaño entre todos los segmentos

**Casos especiales:**
- Si no hay 'x': todo es un segmento externo
- Si hay solo un segmento: es externo
- Segmentos vacíos al inicio o final deben ignorarse

## Complejidad

- **Tiempo:** O(n) donde n es la longitud de la enredadera
- **Espacio:** O(n) para almacenar los segmentos

