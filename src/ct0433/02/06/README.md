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
\int_\gamma f ds = \int_0^1 (9t - 4t + 1 + t + 5)\sqrt{9 + 16 + 1} dt = 9\sqrt{26}
$$

## Esempio

Avendo $r(t) = (-2\cos(\pi t), 2t)$ con $t \in [1, -1[$, e l'altezza:
$$
f(x, y) = \frac{y^2}{\sqrt{4 + 4\pi^2 - \pi^2x^2}}
$$
va trovato:
$$
\int_{r} f ds = \int_{-1}^1 f(r(t)) \|r'(t)\| dt
$$

Perchè $f(r(t))$ sia possibile, va verificato che $\mathrm{Im}(r) \subseteq \mathrm{Dom}(f)$, quindi:
$$
4 + 4\pi^2 - \pi^2x^2 > 0 \Leftrightarrow x^2 < \frac{4}{\pi^2} + 4 \Rightarrow -2\sqrt{\frac{1}{\pi^2} + 1} < x < 2\sqrt{\frac{1}{\pi^2} + 1} \\
\Downarrow \\
\mathrm{Dom}(f) = \left]-2\sqrt{\frac{1}{\pi^2} + 1}, 2\sqrt{\frac{1}{\pi^2} + 1}\right[ \times \mathbb{R}
$$
Di conseguenza, $\mathrm{Im}(r) = [-2, 2] \subseteq \mathrm{Dom}(f)$ e quindi l'integrale di linea è calcolabile:
$$
\int_r f ds = \int_{-1}^1 \frac{4t^2}{\sqrt{4 + 4\pi^2 - 4\pi^2\cos^2(\pi t)}} \sqrt{4\pi^2 \sin^2(\pi t) + 4} dt = \int_{-1}^1 4t^2 dt = \frac{8}{3}
$$
