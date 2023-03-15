# Integrale di linea

Avendo una curva $r\colon [a, b] \to \mathbb{R}^m$, è possibile associare alla $(m+1)$-esima dimensione la funzione:
$$
\begin{split}
f\colon I \subseteq \mathbb{R}^m &\to \mathbb{R}^m \\
(x_1, x_2, ..., x_m) &\mapsto z = f(x_1, x_2, ..., x_m)
\end{split}
$$
che può essere pensata come l'**altezza** di $\gamma$ su $r(t)$, [diversa](../README.md) dall'altezza caratterizzata da $t$ sul grafico $G(r)$.
Infatti, le intersezioni di [curve _non semplici_](../02/README.md) avranno la stessa $z = f(r(t))$.

Per cui, l'**integrale di linea**
$$
\int_\gamma f(x_1, x_2, ..., x_m) ds = \int_a^b f(r(t)) \|r'(t)\| dt
$$
corrisponde all'**area sotto** $\gamma$ in $m+1$ dimensioni.

Per esempio, se il $\mathrm{Codom}(r) = \mathbb{R}^2$, allora $z = f(x, y)$ sarà l'altezza in tre dimensioni di $r$ e l'integrale corrisponde all'area sotto la curva tridimensionale.

In particolare, l'argomento $\|r'(t)\|$ [stira](https://it.wikipedia.org/wiki/Integrale_di_linea_di_prima_specie#/media/File:Line_integral_of_scalar_field.gif) la curva per renderla uniforme.

Per esempio, se $r(t) = (3t, 4t - 1, t + 5)$ e $f(x, y, z) = 3x - y + z$, allora l'area sotto la curva è:
$$
\int_\gamma f ds = \int_0^1 (9t - 4t + 1 + t + 5)\sqrt(9 + 16 + 1) dt = 9\sqrt{26}
$$
