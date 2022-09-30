# Overflow

L'overflow si verifica quando un numero è troppo grande per essere rappresentato su un numero limitato di bit.

Per esempio nell'operazione $1101_2 + 1001_2 = 10110_2$ il bit più significativo sulla posizione $4$ cade dal riporto, ma essendo limitati da $4$ bit si ha overflow.

In base all'operazione sul complemento a due si possono riassumere i casi di overflow in:

| | Somma | Sottrazione |
|:-:|:-:|:-:|
| $+$ $+$ | Si (negativo) | No |
| $-$ $-$ | Si (positivo) | No |
| $+$ $-$ | No | Si (negativo) |
| $-$ $+$ | No | Si (positivo) |
