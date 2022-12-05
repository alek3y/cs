# Integrali definiti

L'integrale **definito**,
$$\int_a^b f(x)dx$$
serve per calcolare l'**area** del trapezoide tra il grafico di $f$, l'asse $x$ e $x = a$ e $x = b$.

L'area può essere _approssimata_ attraverso:
- **Plurirettangoli inscritti**:

	Per cui l'area è approssimata dalla somma delle aree dei rettangoli inscritti nel grafico sottostante a $f$:
	$$I \approx s_n = \sum_{i = 1}^n h_i \Delta x$$
	dove $\Delta x = \frac{b - a}{n}$ è la **base** dei rettangoli, mentre $h_i = \underset{x \in I_i}{\min}(f(x))$, con $I_i$ che è l'intervallo sulle $x$ largo $\Delta x$, è l'**altezza** del rettangolo $i$-esimo.

- **Plurirettangoli circoscritti**

	Per cui è approssimata dai rettangoli inscritti:
	$$I \approx S_n = \sum_{i = 1}^n H_i \Delta x$$
	dove $H_i = \underset{x \in I_i}{\max}(f(x))$.

Per il [teorema del confronto](../../04/01/README.md), $\sup(s_n) = \inf(S_n) = I$ e quindi:
$$
\begin{array}{rcccl}
s_n & \leq & I & \leq & S_n \\
& \searrow & \overset{n \to +\infty \Rightarrow \Delta x \to 0}{\downarrow} & \swarrow & \\
&& \int_a^b f(x) dx &&
\end{array}
$$
