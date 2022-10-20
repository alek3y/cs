# Forme canoniche

Scrivendo le equazioni in queste due forme sarà possibile essere certi del tempo di passaggio del segnale sul circuito.

Sia $E$, l'equazione che produce la seguente tabella di verità:

| A | B | E |
|:-:|:-:|:-:|
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 0 |
| 1 | 1 | 1 |

1. **Somma di prodotti**

	Partendo dalla tabella di verità, si costruisce un equazione **mintermine** degli ingressi per ogni $1$ in uscita.

	Per ricavarsi $E$, basta quindi creare due equazioni per tutti i casi con $E = 1$ e sommarle:
	$$E = (\overline A \cdot B) + (A \cdot B)$$
	e quindi servono tutti i _mintermini_ a $0$ perchè $E = 0$.

2. **Prodotto di somme**

	In questo caso, si costruisce un equazione **maxtermine** degli ingressi per ogni $0$ in uscita.

	Per cui, basterà fare il prodotto delle equazioni dei casi $E = 0$:
	$$E = (A + B) \cdot (\overline A + B)$$
	e quindi basta un solo _maxtermine_ a $0$ perchè $E = 0$.
