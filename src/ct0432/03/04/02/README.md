# Esponenziali e logaritmiche

## Esponenziali

$$
\begin{split}
f\colon &\mathbb{R} \rightarrow \mathbb{R}\newline
&x \mapsto a^x = \exp_a(x)
\end{split}
$$
con $a \in \mathbb{R}^+$ avrà i valori immagine $\mathrm{Im}(f) > 0$.

Se si restringe il suo **codominio** a $C = (0, +\infty)$, la funzione diventa **biettiva** e quindi può essere invertita a $f^{-1}(x) = \log_a(x)$.

## Logaritmiche

$$
\begin{split}
f\colon &(0, +\infty) \rightarrow \mathbb{R}\newline
&x \mapsto \log_a(x) \text{ con } a > 0 \land a \neq 1
\end{split}
$$
per cui $x \in D_f \Rightarrow x > 0$, e valgono le proprietà:
- $\log_a(x^y) = y\log_a(x) \neq \log_a^y(x)$
- $\log_a(x) + \log_a(y) = \log_a(xy)$
- $\log_a(x) - \log_a(y) = \log_a(\frac{x}{y})$
- $\log_{a^b}(x) = \frac{1}{b}\log_a(x)$
- $\log_a(x) = \frac{log_b(x)}{log_b(a)}$

Essendo funzione **inversa** di $a^x$,
$$(f \circ f^{-1})(x) = f(f^{-1}(x)) = a^{\log_a(x)} = x$$
$$(f^{-1} \circ f)(x) = f^{-1}(f(x)) = \log_a(a^x) = x\log_a(a) = x$$
