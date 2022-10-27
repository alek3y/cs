# Teoremi

1. **Teorema degli zeri**

	Anche detto **teorema di Bolzano**, diche che avendo una funzione $f\colon [a, b] \to \mathbb{R}$ continua:
	$$f(a) < 0 < f(b), \text{ o anche } f(a) \cdot f(b) < 0$$
	e quindi $f(a)$ e $f(b)$ hanno segno opposto ($f(a) \cdot f(b) > 0$ se entrambi negativi o positivi), allora:
	$$\exists c \in [a, b] : f(c) = 0$$
	cioè che se $f$ è continua allora passerà per forza su $y = 0$.

2. **Teorema dei valori intermedi**

	Per cui se la funzione $f\colon [a, b] \to \mathbb{R}$ è continua, allora i valori che assume coprono tutti i valori nell'intervallo $[f(a), f(b)] \subseteq \mathbb{R}$, che corrisponde a $\mathrm{Im}(f)$.

3. **Teorema di Weierstrass**

	Riguarda l'esistenza di **massimi** e **minimi**, per una funzione $f\colon A \to \mathbb{R}$, dove $A$ è un **intervallo chiuso e limitato**, per cui:
	$$\exists c, d \in A : f(c) \leq f(x) \leq f(d), \forall x \in A$$

## Esempi

- $f(x) = x + \log_e(x)$

	Ha dominio $D = (0, +\infty)$, e va mostrato che la funzione ha almeno uno zero in $D$, quindi:
	$$x + \log_e(x) = 0$$

	Basta trovare due valori $a$ e $b$ che hanno $f(a)$ e $f(b)$ con segno opposto:
	$$\lim_{x \to +\infty} f(x) = +\infty \Rightarrow \exists a \in D : f(a) > 0$$
	$$\lim_{x \to 0^+} f(x) = -\infty \Rightarrow \exists b \in D : f(b) < 0$$

	Quindi, dato che la funzione è continua e $f(a) \cdot f(b) \leq 0$, allora:
	$$\exists c \in D : f(c) = 0$$
