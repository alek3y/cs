# Numeri negativi

Le possibili rappresentazioni di numeri negativi in binario sono includono:

| Binario | Modulo e segno | Complemento a uno | Complemento a due |
|:-:|:-:|:-:|:-:|
| $000$ | $+0$ | $+0$ | $+0$ |
| $011$ | $+3$ | $+3$ | $+3$ |
| $100$ | $-0$ | $-3$ | $-4$ |
| $111$ | $-3$ | $-0$ | $-1$ |

Tra di esse, il **complemento a due** è la rappresentazione preferita per gli elaboratori, e dal primo _bit_ (_Most Significant Bit_) è possibile capire se il numero è negativo (per cui è $1$) o positivo (per cui è $0$).

Per trasformare un numero positivo in negativo a complemento a due, è sufficente **invertire i singoli bit** e **sommare 1**.
Per esempio, con $5_{10} = 0101_2$, $-5_{10}$ varrà $1010_2 + 1_2 = 1011_2$.
