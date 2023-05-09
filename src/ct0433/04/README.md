# Integrali doppi

Gli [integrali definiti](../../ct0432/10/03/README.md) in $\mathbb{R}$ possono estendersi in $\mathbb{R}^2$, con l'**integrale doppio** per una $f\colon [a, b] \times [c, d] \to \mathbb{R}$,
$$
\iint_{\mathrm{Dom}(f)} f(x, y) dx dy
$$
che può essere approssimato con dei _parallelepipedi rettangoli_ (ovvero rettangoli in tre dimensioni), dove $dx$ e $dy$ sono i lati della base, mentre $f(x, y)$ è l'altezza.

L'area sottostante a $f(x, y)$, valutata nella regione $a \leq x \leq b \land c \leq y \leq d$, cioè:
$$
\int_c^d \int_a^b f(x, y) dx dy
$$
può essere pensata come la somma di tutte le aree $\int_a^b f(x, y_0) dx$ per [ogni sezione](https://www.geogebra.org/3d/fxgpjye8) $y = y_0 \in [c, d]$.

Lo stesso vale per le sezioni di $x$, per il **teorema di Fubini**:
$$
\int_c^d \int_a^b f(x, y) dx dy =
\int_a^b \int_c^d f(x, y) dy dx
$$

Per esempio, l'integrale _doppio_ su $[0, 1] \times [0, 2]$:
$$
\begin{split}
\iint_{[0, 1] \times [0, 2]} ye^{xy} dx dy &=
\int_0^2 \int_0^1 ye^{xy} dx dy =
\int_0^2 \left[e^{xy}\right]_0^1 dy = \\
&= \int_0^2 e^y - 1 dy = \left[e^y - y\right]_0^2 =
e^2 - 3
\end{split}
$$

Oppure, integrando prima rispetto a $y$:
$$
\begin{split}
\int_0^1 \int_0^2 ye^{xy} dy dx &=
\int_0^1 \left[\frac{y}{x}e^{xy} - \frac{1}{x^2}e^{xy}\right]_0^2 dx =
\int_0^1 \frac{2e^{2x}}{x} - \frac{e^{2x} - 1}{x^2} dx = \\
&= \lim_{t \to 0}\int_t^1 \frac{2e^{2x}}{x} - \frac{e^{2x} - 1}{x^2} dx =
e^2 - 3
\end{split}
$$

## A variabili separabili

Quando l'_integranda_ è a **variabili separabili**, allora:
$$
\int_a^b \int_c^d g(x)h(y) dy dx = \left(\int_a^b g(x) dx\right) \left(\int_c^d h(y) dy\right)
$$

Per esempio, se:
$$
\int_0^1 \int_0^2 ye^x dy dx =
\left(\int_0^1 e^x dx\right)\left(\int_0^2 y dy\right) =
\left[e^x\right]_0^1 \cdot \left[\frac{y^2}{2}\right]_0^2 = 2(e - 1)
$$
