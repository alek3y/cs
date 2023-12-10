# Variabili congiunte

Oltre alle [variabili casuali](../03/README.md) prese **singolarmente**, si può essere interessati anche a **due o più** variabili insieme.

## Discrete

La **probabilità congiunta** di due variabili $X$ e $Y$ **discrete** è:
$$
p(x, y) = P(X = x, Y = y) = P(X = x \land Y = y)
$$
da cui si ricavano le singole **funzioni di probabilità marginali** delle due variabili:
$$
p_X(x) = \sum_y p(x, y)\ \land\ p_Y(y) = \sum_x p(x, y)
$$

Per esempio, dati $X = \{0, 1\}$ e $Y = \{0, 1, 2\}$ con _distribuzione congiunta_, allora:
$$
\begin{array}{c|ccc|c}
& & Y & & \\
X & 0 & 1 & 2 & p_X(x) \\ \hline
0 & 0.30 & 0.15 & 0.05 & 0.50 \\
1 & 0.20 & 0.15 & 0.15 & 0.50 \\ \hline
p_Y(y) & 0.50 & 0.30 & 0.20 & 1
\end{array}
$$
e quindi $P(X < Y) = p(0, 1) + p(0, 2) + p(1, 2) = 0.15 + 0.05 + 0.15 = 0.35$.

## Funzione di ripartizione

La **funzione di ripartizione congiunta** per le variabili $X$ e $Y$, è definita come:
$$
F(x, y) = P(X \leq x, Y \leq y)
$$
che può essere calcolata rispetto ad una variabile, e.g. $F_X(x) = P(X \leq x, Y < \infty) = \lim\limits_{y \to \infty} F(x, y)$.

Analogamente al caso _singolo_, se $X$ e $Y$ sono **congiunte continue** allora $f(x, y) = \frac{\partial^2 F(x, y)}{\partial x \partial y}$.

## Continue

La **probabilità** di due variabili **continue** è:
$$
P((X, Y) \in A \times B) = P(X \in A, Y \in B) = \int_B \left(\int_A f(x, y) dx\right) dy
$$
dove la _funzione di densità_ $f(x, y)$ rispetta le **proprietà** per cui:
- $f(x, y) \geq 0, \forall (x, y) \in \mathbb{R}^2$
- $\iint_{\mathbb{R}^2} f(x, y) dx dy = 1$

In questo si ricavano le **funzioni di densità marginali** come:
$$
f_X(x) = \int_{\mathbb{R}} f(x, y) dy\ \land\ f_Y(y) = \int_{\mathbb{R}} f(x, y) dx
$$

Per esempio, data la _densità congiunta_ $f(x, y) = 2e^{-x}e^{-2y}$ per $x, y > 0$, allora:
$$
P(X > 1, Y < 1) = \int_0^1 \left(\int_1^\infty 2e^{-x}e^{-2y} dx\right) dy = \frac{e^2 - 1}{e^3}
$$
oppure:
$$
P(X < Y) = \iint_{\{(x, y) \mid x < y\}} 2e^{-x}e^{-2y} dx dy =
\int_0^\infty \left(\int_0^y 2e^{-x}e^{-2y} dx\right) dy = \frac{1}{3}
$$

## Variabili indipendenti

Si possono definire due variabili **congiunte** come **indipendenti** se:
$$
P(X \in A, Y \in B) = P(X \in A) P(Y \in B)
$$
che si può estendere alla _funzione di ripartizione_ $F(x, y) = F_X(x)F_Y(y)$ e quindi alle _marginali_:
$$
p(x, y) = p_X(x)p_Y(y)\ \land\ f(x, y) = f_X(x)f_Y(y)
$$

Per esempio, dato $f(x, y) = 24xy$ con $0 < x, y, x+y < 1$ si può affermare che $f(x, y) \neq f_X(x)f_Y(y)$ dato che il dominio di $f(x, y)$ dipende da entrambe $X$ e $Y$ mentre $f_X(x)f_Y(y)$ avrebbe dominio $X \times Y$.

Inoltre, conoscere le _marginali_ sapendo che sono _indipendenti_ permette di **ricavare** la _probabilità congiunta_.
Per esempio, se $p_X(0) = 0.5$ e $p_Y(1) = 0.2$ allora $p(0, 1) = 0.1$, e così via per ogni $X$ e $Y$.

## Distribuzioni condizionate

La **funzione di probabilità condizionata** di due variabili **discrete** $X$ e $Y$ è definita come:
$$
p_{X | Y}(x | y) = \frac{p(x, y)}{p_Y(y)}
$$
che se sono **indipendenti** diventa $p_{X | Y}(x | y) = p_X(x)$.

Per esempio, dati $X = \{0, 1\}$ e $Y = \{0, 1\}$:
$$
\begin{array}{c|cc|c}
& Y & & \\
X & 0 & 1 & p_X(x) \\ \hline
0 & 0.4 & 0.2 & 0.6\\
1 & 0.1 & 0.3 & 0.4 \\ \hline
p_Y(y) & 0.5 & 0.5 & 1
\end{array}
$$
allora $p_{X | Y}(0 | 1) = \frac{0.2}{0.5} = 0.4 \neq p_X(0)$ e quindi $X$ e $Y$ non sono _indipendenti_.

Nel caso le due variabili $X$ e $Y$ siano **continue** invece, la _probabilità condizionata_ è definita come:
$$
f_{X | Y}(x | y) = \frac{f(x, y)}{f_Y(y)}
\Rightarrow
P(X \in A | Y = y) = \int_A f_{X | Y}(x | y) dx
$$
che anche in questo caso se **indipendenti** diventa $f_{X | Y}(x | y) = f_X(x)$.

## Valore atteso

Come per variabili _singole_, una trasformazione $g(X, Y)$ di $(X, Y)$ permette di ricalcolarne il **valore atteso**:
$$
E(g(X, Y)) = \sum_y \sum_x g(x, y) p(x, y)
$$
nel caso **discreto**, mentre nel caso **continuo**:
$$
E(g(X, Y)) = \iint_{\mathbb{R}^2} g(x, y) f(x, y) dx dy
$$
da cui $E(X + Y) = E(X) + E(Y)$ e nel caso siano **indipendenti** $E(X \cdot Y) = E(X)E(Y)$.
