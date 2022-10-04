# Overflow

L'overflow si verifica quando un numero è troppo grande per essere rappresentato su un numero limitato di bit.

Per esempio nell'operazione $1101_2 + 1001_2 = 10110_2$ il bit più significativo sulla posizione $4$ cade dal riporto, ma essendo limitati da $4$ bit si ha overflow.

Nel caso di operazioni su complemento a due invece, l'overflow capita se gli ultimi due riporti sono _discordi_, e cioè diversi.
Per esempio, $1010_2 + 1101_2 = 0111_2$ ha gli ultimi due riporti $1, 0$ perchè $-6 - 3 = -9$ che non ci sta sui $4$ bit e quindi è in overflow.
