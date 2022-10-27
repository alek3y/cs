# Funzioni continue

Una funzione $f\colon A \subseteq \mathbb{R} \to \mathbb{R}$ si dice **continua su un punto** $x_0 \in A$, se:
$$\lim_{x \to x_0} f(x) = f(x_0)$$
che vale se $x_0$ è di **accumulazione** per $A$ (cioè che ogni intorno su $x_0$ ha punti appartenenti ad $A$), ma anche se è un punto **isolato** di $A$ (perchè non ha punti nell'intorno di $x_0$).

Di conseguenza, la funzione $f$ si dice **continua** se è continua su **ogni punto del suo dominio**.

## Determinare la continuità

Per determinare se la funzione è continua sul punto $x_0$ basterà tenderci da sinistra e da destra:
$$f \text{ è continua} \Leftrightarrow \lim_{x \to x_0^-} f(x) = \lim_{x \to x_0^+} f(x) = f(x_0) \in \mathbb{R}$$

Per esempio, con $\mathrm{sgn}(x)$ non é continua su $x = 0$ perchè
$$\lim_{x \to 0^-} \mathrm{sgn}(x) \neq \lim_{x \to 0^+} \mathrm{sgn}(x)$$
mentre $f(x) = \begin{cases}1 & \text{se } x > 0 \\ -1 & \text{se } x < 0\end{cases}$ è continua perchè $D = \mathbb{R} \setminus \{0\}$ e quindi $0 \not\in \mathrm{Dom}(f)$.
