# Numeri negativi

Le possibili rappresentazioni di numeri negativi in binario sono includono:

| Binario | Modulo e segno | Complemento a uno | Complemento a due |
|:-:|:-:|:-:|:-:|
| $000$ | $+0$ | $+0$ | $+0$ |
| $011$ | $+3$ | $+3$ | $+3$ |
| $100$ | $-0$ | $-3$ | $-4$ |
| $111$ | $-3$ | $-0$ | $-1$ |

Tra di esse, il **complemento a due** è la rappresentazione preferita per gli elaboratori, e dal primo _bit_ (_Most Significant Bit_) è possibile capire se il numero è negativo (per cui è $1$) o positivo (per cui è $0$).

## Complementare a due

Ci sono due modi per complementare a due un numero:

1. Invertire **tutti i bit** e aggiungere $1_2$

	Per esempio, con $5_{10} = 0101_2$, $-5_{10}$ varrà $1010_2 + 1_2 = 1011_2$.

2. Invertire **i bit fino all'ultimo** $1$ verso destra escluso

	Per esempio, con $10_{10} = 01010_2$, $-10_{10}$ varrà $10110_2$.

## Conversione in base 10

Per convertire un numero in complemento a due in base 10, basterà dare peso negativo al primo bit (di segno) e effettuare la conversione normalmente.

Per esempio, con $1101_2 = 1 \cdot -2^3 + 1 \cdot 2^2 + 0 \cdot 2^1 + 1 \cdot 2^0 = -8 + 4 + 1 = -3$.

## Estensione dei bit

Per transformare un numero di $n$ bit, basta estendere il valore del bit di segno per la quantità di bit aggiunti.

Per esempio:
1. $1010_2 = 1111 \space 1010_2$
2. $0100_2 = 0000 \space 0100_2$
