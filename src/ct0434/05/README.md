# Principio di induzione

Il **principio di induzione** è una tecnica per provare che una **proprietà** è valida per **tutti i numeri** di $\mathbb{N}$.

La tecnica per dimostrare la proprietà $P(x)$ consiste nel:
- **Caso base**, per cui si dimostra che $P(0)$ è vero
- **Passo induttivo**, per cui si dimostra che $P(n) \Rightarrow P(n+1)$, cioè che se $P(n)$ è dato per vero (**ipotesi induttiva**), anche $P(n+1)$ deve esserlo

$$P(0) \land (P(n) \Rightarrow P(n+1)) \Rightarrow P(x), \hspace{1em} \forall x, n \in \mathbb{N}$$

Per esempio, si ha da dimostrare che con $S(n) = "\text{somma dei primi } n"$, $S(x) = \frac{x(x+1)}{2}$:
- Caso base: $S(0) = 0 = \frac{0 \cdot (0 + 1)}{2}$
- Passo induttivo:

	Sia l'_ipotesi induttiva_ $S(n) = \frac{n(n+1)}{2}$ scontata, con $n \in \mathbb{N}$.
$$
S(n + 1) = 0 + 1 + ... + n + (n + 1) = S(n) + (n + 1) = \\
= \frac{n(n + 1)}{2} + (n + 1) = \frac{n(n + 1) + 2(n + 1)}{2} = \\
\frac{(n + 2)(n + 1)}{2} = \frac{(n + 1)((n + 1) + 1)}{2}
$$

	E quindi è verificato che $S(n) = \frac{n(n + 1)}{2} \Rightarrow S(n + 1) = \frac{(n + 1)((n + 1) + 1)}{2}$

## Principio di induzione completo

Tramite il **principio di induzione completo** (o forte) è possibile semplificare il processo di dimostrazione visto che **si considerano veri** tutti i numeri dal caso base fino a $n$ nel passo induttivo.

Quindi, per dimostrare la proprietà $P(x)$ va dimostato:
- **Caso base**, per cui si dimostra se $P(0)$ è vero
- **Passo induttivo**, per cui si dimostra che se la proprietà vale per i primi $n$ numeri allora vale anche per $P(n+1)$, quindi $P(0) \land P(1) \land ... \land P(n) \Rightarrow P(n+1)$.

Per esempio, si ha da dimostrare che $P(x) = "n \text{ è scomponibile in fattori primi}"$ vale $\forall n \geq 2 \in \mathbb{N}$:
- Caso base: $n = 2 \Rightarrow P(2)$ vale perchè $2 = 2$ che è primo.
- Passo induttivo:

	Si suppone che quando $n \geq 2$, tutti i numeri tra $2$ ed $n$ siano scomponibili:
	$$\forall x \in \mathbb{N} : 2 \leq x \leq n, P(x)$$
	che è quindi l'_ipotesi induttiva_.

	Si vuole quindi dimostrare che la proprietà vale per $n + 1$, ed esistono due casi per cui dovrebbe valere:
	- $n + 1$ _è primo_, allora $P(n+1)$ vale perchè è già scomposto
	- $n + 1$ _non è primo_, allora
		$$\exists y, z \in \mathbb{N} : n + 1 = y \cdot z \land \{y, z\} \not\subseteq \{1, n+1\}$$
		per cui $2 \leq y \leq n \land 2 \leq z \leq n$. \
		Per _ipotesi induttiva_ $y$ e $z$ sono scomponibili in fattori primi e quindi lo è anche $n + 1$.
