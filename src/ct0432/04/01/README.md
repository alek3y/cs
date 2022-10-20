# Teoremi

1. **Teorema di unicità**

	Che dice che un limite su un punto $x_0$ può assumere un solo valore $l \in \mathrm{Codom}(f)$.

	$$\lim_{x \to x_0} f(x) = l_1 \land \lim_{x \to x_0} f(x) = l_2 \Rightarrow l_1 = l_2$$
	perchè se fossero diversi, dovrebbe per forza esistere un intorno (delle $y$) di $l_1$ e un altro di $l_2$ che non condividono alcun punto, ma visto che per entrambi la _controimmagine_ (valori sulle $x$) è un intorno di $x_0$, allora se $x \in I_{x_0} \Rightarrow f(x) \in I_{l_1} \land f(x) \in I_{l_2}$, cosa assurda, visto che abbiamo dato per scontato che $I_{l_1} \cap I_{l_2} = \emptyset$.

2. **Teorema di permanenza del segno**

	Che dice che se il risultato di un limite è positivo, allora esiste un intorno di $x_0$ per cui $f(x)$ è positiva su tutti i punti dell'intorno.

	$$l > 0 \Rightarrow \exists I_{x_0} : f(x) > 0, \forall x \in I_{x_0}$$

2. **Teorema del confronto**

	Che dice che se in un intorno di $x_0$,
	$$f(x) \leq g(x) \leq h(x)$$
	dove $f(x)$ e $h(x)$ tendono a $l$, allora:
	$$\lim_{x \to x_0} f(x) = l$$

	Per esempio, se $g(x) = \frac{1}{x + 1}$ e $0 \leq g(x) \leq \frac{1}{x}$, allora $\lim_{x \to +\infty} g(x) = 0$.
