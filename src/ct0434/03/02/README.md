# Relazioni di equivalenza

Una relazione si dice **di equivalenza** se soddisfa le proprietà:
- **Riflessiva**
- **Simmetrica**
- **Transitiva**

Per esempio, con $R = \{(x, y) | x + y \text{ è pari}\}$, abbiamo che è:
- Riflessiva, perchè $x + x = 2x$
- Simmetrica, perchè se $x + y$ è pari lo è anche $y + x$, per la proprietà commutativa
- Transitiva, perchè se $x + y$ e $y + z$ sono pari, allora $(x + y) + (y + z) - 2y = x + z$

## Classi di equivalenza

Si dicono **classi di equivalenza** quei insiemi che contengono tutti gli elementi di un insieme $A$ che soddisfano una relazione $R$ **di equivalenza** con un elemento conosciuto $a$, chiamato **rappresentante della classe di equivalenza**:
$$[a]_R = a/R = \{x \in A | aRx\}$$

Ogni classe di equivalenza **conterrà sempre** il suo **rappresentante**.

Per esempio, se $A = \{1, 2, 5, 6, 7, 9, 11\}$ e $R = \{(x, y) \in A^2 | (x - y) \text{ è divisibile per }5\}$, le classi di equivalenza saranno:
- $[1]_R = \{1, 6, 11\}$, perchè $1 - 1 = 0$ che è divisibile per $5$, $6 - 1 = 5$, $11 - 1 = 10$
- $[2]_R = \{2, 7\}$
- $[5]_R = \{5\}$
- $[9]_R = \{9\}$

L'insieme di tutte le classi di equivalenza si dice **insieme quoziente** di $A$ secondo $R$,
$$A/R = \{[a]_R | a \in A\}$$
e **forma una partizione** di $A$.
