# Algebra booleana

Una funzione logica può essere espressa come:
- **Tabella di verità**
- **Equazione**

Le operazioni logiche fondamentali sono quindi:
- **AND** ($\cdot$)
- **OR** ($+$)
- **NOT** ($\neg$)

## Proprietà

- **Identità**: $A + 0 = A$, $A \cdot 1 = A$
- **Nullo**: $A + 1 = 1$, $A \cdot 0 = 0$
- **Idempotente**: $A + A = A$, $A \cdot A = A$
- **Inverso**: $A + \neg A = 1$, $A \cdot \neg A = 0$
- **Commutativa**: $A + B = B + A$, $A \cdot B = B \cdot A$
- **Associativa**: $A + (B + C) = A + B + C$, $A \cdot (B \cdot C) = A \cdot B \cdot C$
- **Distributiva**: $A + (B \cdot C) = (A \cdot B) + (A \cdot C)$, $A \cdot (B + C) = (A + B) \cdot (A + C)$
- **De Morgan**: $\neg (A + B) = \neg A \cdot \neg B$, $\neg (A \cdot B) = \neg A + \neg B$

## Operatori universali

Dal **NAND** e dal **NOR** è possibile ricavarsi tutte le porte logiche, e sono quindi chiamati **operatori universali**.

Per esempio,
$$\neg A = \neg A + 0 = \neg (A \cdot \neg 0) = \neg (A \cdot 1)$$
che sarebbe il $\mathrm{nand}(A, 1)$.

## Forme canoniche

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
