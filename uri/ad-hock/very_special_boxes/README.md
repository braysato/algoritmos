# Very Special Boxes - 1290

## Problema

Special Box Company (SBC) is a small family-owned and family-run business which produces decorated carton boxes for wrapping gifts. The boxes are hand-made, produced individually from ﬁne materials. When accepting an order from a client, they always produce a few more boxes than needed, to keep a stock of boxes to be sold in the future, if needed. Over the years their stock has been growing, with boxes all over the place, and they decided they needed to organize it a bit more. They have therefore made a list registering the dimensions of every box in their stock.

SBC has just received an order from a client that must be delivered tomorrow, so there is no time to produce new boxes. The client wants a certain number N of boxes all of the same size; each box will be used to pack one item of dimensions X, Y and Z. As the carton used in the boxes is very thin, you may assume that a box of size (X, Y, Z) would ﬁt perfectly the item the client wants to wrap. If there are not at least N boxes that ﬁt perfectly, the client wants N boxes that ﬁt the items as tightly as possible. The box size that ﬁts the items as tightly as possible is the one which minimizes the empty space when the item is put inside the box. An item can be rotated in any direction to be accomodated inside a box; therefore, a box of size (X, Y, Z) is as good as a box of size (Y, Z, X), for example.

Can you help SBC ﬁnding whether they can fulﬁll the customer order?

### Input

The input consists of several test cases. The ﬁrst line of a test case contains two integers N and M, indicating respectively the number of boxes the client needs to buy (1 ≤ N ≤ 1500) and the number of boxes in the stock list (1 ≤ M ≤ 1500). The second line contains three integers X, Y and Z, representing the dimensions of the item the client wants to wrap (0 < X, Y, Z ≤ 50). Each of the next M lines contains three integers A, B and C representing the dimensions of a box in the stock list (0 < A, B, C ≤ 50). A test case with N = 0 indicates the end of the input.

### Output

For each test case in the input your program must produce one line, containing either:
- the single word 'impossible', in case it is not possible to fulﬁll the client's order (because there are not at least N boxes of the same size in stock that can contain the item); or
- one integer V, which speciﬁes the volume of empty space left when one of the N items is packed in one of the boxes chosen.

### Sample Input
```
1 1
2 4 3
2 3 4
2 6
3 1 3
7 4 7
10 8 2
2 8 10
6 2 9
7 7 4
6 2 9
1 1
3 3 3
1 1 1
0 0
```

### Sample Output
```
0
99
impossible
```

## Análisis

1. **Normalización**: Ordenar las dimensiones tanto del item como de cada caja para manejar rotaciones.
2. **Validación**: Una caja (A, B, C) contiene al item (X, Y, Z) si A >= X, B >= Y, C >= Z (tras ordenar).
3. **Agrupación**: Contar cuántas cajas hay de cada tamaño (dimensiones ordenadas).
4. **Selección**: Entre los grupos con al menos N cajas válidas, elegir el de menor volumen.
5. **Resultado**: Espacio vacío = Volumen_caja - Volumen_item

## Complejidad

- Tiempo: O(M log M) para ordenar y agrupar las cajas
- Espacio: O(M) para almacenar los conteos
