# Circuiti sequenziali

I circuiti sequenziali permettono di calcolare funzioni che non dipendono solo dagli input ma anche dallo stato del circuito.

## S-R Latch

Con questo _latch_ è possibile memorizzare un singolo bit di memoria in modo _asincrono_.

![Circuito del S-R latch](assets/01.png)

Quindi, la tabella di verità per il componente sarà:

| $S$ | $R$ | $Q$ | $\overline Q$ |
|:-:|:-:|:-:|:-:|
| 0 | 0 | / | / |
| 0 | 1 | 0 | 1 |
| 1 | 0 | 1 | 0 |
| 1 | 1 | ? | ? |

per cui, quando $S = R = 0$ gli input non avranno effetto sullo stato del circuito, mentre se $S = R = 1$, il circuito avrà comportamento indefinito.

## Clock

Il **clock** (CLK) è un componente che rende un _circuito combinatorio_ **sincrono** rispetto al suo periodo, cioè il tempo minimo richiesto dal circuito perchè diventi stabile.

Un esempio di circuito sincrono è il **D latch**, che corrisponde al _S-R latch_ ma con il clock.

![Circuito del D latch](assets/02.png)

Che avrà tabella di verità:

| $CLK$ | $D$ | $Q$ | $\overline Q$ |
|:-:|:-:|:-:|:-:|
| 0 | 0 | / | / |
| 0 | 1 | / | / |
| 1 | 0 | 0 | 1 |
| 1 | 1 | 1 | 0 |
