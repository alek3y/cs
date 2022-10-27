# Funzioni elementari

- $x^a$, $a^x$, $\log_a x$
- $\sin(x)$, $\tan(x)$
- $\arcsin(x)$, $\arctan(x)$
- $|x|$

Per decidere se una **funzione definita a tratti** è continua, bisogna controllare il punti di giunzione tra i vari casi.

Per esempio, la funzione $\mathrm{sgn}(\sin(x))$ è continua nei punti $\mathbb{R} \setminus \{k\pi\}$, perchè quando $\mathrm{sgn}(x_0) = 0$, $\lim_{x \to x_0^-} \mathrm{sgn}(x) \neq \lim_{x \to x_0^+} \mathrm{sgn}(x)$.

Un altro caso, è quello di trovare un parametro $a$ che renda la funzione definita a tratti continua, per esempio con:
$$f(x) = \begin{cases}x + 1 & \text{ se } x \leq 1 \\ 3 - 2ax^2 & \text{ se } x > 1\end{cases}$$
la funzione è continua se $\lim_{x \to 1^+} f(x) = f(1)$, e quindi se
$$3 - 2a \cdot 1^2 = 2 \Leftrightarrow 3 - 2a = 2 \Leftrightarrow a = \frac{1}{2}$$
