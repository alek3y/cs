# Integrali doppi

Gli [integrali definiti](../../ct0432/10/03/README.md) in $\mathbb{R}$ possono estendersi in $\mathbb{R}^2$, con l'**integrale doppio** per una $f\colon [a, b] \times [c, d] \to \mathbb{R}$,
$$
\iint_{\mathrm{Dom}(f)} f(x, y) dx dy
$$
che può essere approssimato con dei _parallelepipedi rettangoli_ (ovvero rettangoli in tre dimensioni), dove $dx$ e $dy$ sono i lati della base, mentre $f(x, y)$ è l'altezza.

L'area sottostante a $f(x, y)$, valutata nella regione $a \leq x \leq b \land c \leq y \leq d$, cioè:
$$
\int_c^d \int_a^b f(x, y)dx dy
$$
può essere pensata come la somma di tutte le aree $\int_a^b f(x, y_0) dx$ per [ogni sezione](https://www.geogebra.org/3d/fxgpjye8) $y = y_0 \in [c, d]$.
