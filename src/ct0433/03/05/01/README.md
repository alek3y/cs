# Continuità

Una funzione $f$ è **continua** in $x_0 \in \mathrm{Dom}(f) \subset \mathbb{R}^n$ se:
$$
\lim_{x \to x_0} f(x) = f(x_0)
$$

## Esempi

- $$
f(x, y) = \begin{cases}
f_1(x, y) = x^2 + y^2 - 1 & \text{se } y > 2x \\
f_2(x, y) = -2x - y & \text{se } y \leq 2x
\end{cases}
$$
	Sappiamo che $f_{1,2} \in C^0$, allora bisogna capire se $f \in C^0$ quando
$$
\begin{cases}
y = 2x \\
f_1(x, y) = f_2(x, y)
\end{cases} \Leftrightarrow
\begin{cases}
y = 2x \\
x^2 + 4x^2 - 1 = -2x - 2x
\end{cases} \Rightarrow
x_{1,2} = \frac{1}{5}, -1
$$
	allora $f$ è continua solamente sui punti $(\frac{1}{5}, 2 \cdot \left(\frac{1}{5}\right))$ e $(-1, 2 \cdot (-1))$ lungo $y = 2x$.

- $$
f(x, y) = \begin{cases}
f_1(x, y) = (x^2 + y^2 - 1)(x + y^2 - 2) & \text{se } xy \neq 0 \\
f_2(x, y) = 0 & \text{se } xy = 0
\end{cases}
$$
	Perchè $f \in C^0$ allora:
$$
\begin{cases}
xy = 0 \\
(x^2 + y^2 - 1)(x + y^2 - 2) = 0
\end{cases} \\
\Downarrow \\
x = 0 \Leftrightarrow (y^2 - 1)(y^2 - 2) = 0 \Leftrightarrow y = \pm 1, \pm\sqrt{2} \\
\lor \\
y = 0 \Leftrightarrow (x^2 - 1)(x - 2) = 0 \Leftrightarrow x = \pm 1, 2
$$
