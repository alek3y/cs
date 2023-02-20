# Secondo ordine

Una equazione differenziale di secondo ordine si dice **lineare** se ha forma:
$$
a_2(t)y'' + a_1(t)y' + a_0(t)y = g(t) \\
\Downarrow \\
y'' + a(t)y' + b(t)y = f(t)
$$
con $a_2(t) \neq 0$, perchè altrimenti non è equazione differenziale.

L'_integrale generale_ sarà composto da due parametri $c_1$ e $c_2$, di conseguenza il _problema di Cauchy_ sarà formato da **due** _condizioni iniziali_:
$$
\begin{cases}
y'' + a(t)y' + b(t)y = f(t) \\
y'(t_0) = y_1 \\
y(t_0) = y_0
\end{cases}
$$

## Omogenee a coefficienti costanti

Se $f(t) = 0$ e i coefficienti sono costanti, l'equazione differenziale sarà **omogenea a coefficienti costanti**:
$$y'' + ay' + by = 0$$

Per risolverla, si può assumere che ogni soluzione sia in forma $y(t) = e^{rt}$, per cui
$$
r^2e^{rt} + are^{rt} + be^{rt} = 0 \\
\Downarrow \\
e^{rt}(r^2 + ar + b) = 0
$$
deve avere soluzioni.

Dato che $e^{rt} > 0, \forall r, t \in \mathbb{R}$:
$$
r^2 + ar + b = 0 \\
\Downarrow \\
\Delta = a^2 - 4b
$$

- Se $\Delta > 0$, allora si trova $r_{1,2}$ da cui si ricava:
$$y = c_1e^{r_1t} + c_2e^{r_2t}$$
come _combinazione lineare_ di $y_1 = e^{r_1t}$ e $y_2 = e^{r_2t}$, dato che:
$$
L(y_1) = 0 \land L(y_2) = 0 \Rightarrow L(c_1y_1 + c_2y_2) = 0
$$
con $0 = f(t)$ (perchè _omogenea_), e $L(y) = y'' + ay' + by$.
- Se $\Delta = 0$
- Se $\Delta < 0$
