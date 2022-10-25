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
