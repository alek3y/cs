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
y(t_0) = y_0 \\
y'(t_0) = v_0
\end{cases}
$$

## Lineari a coefficienti costanti

Un'equazione è lineare a **coefficienti costanti** quando:
$$
y'' + ay' + by = f(t)
$$
con $a, b \in \mathbb{R}$.

Viene detta **omogenea** se $f(t) = 0$ e **completa** se $f(t) \neq 0$.

### Lineari omogenee

Se $f(t) = 0$, si può assumere che ogni soluzione sia in forma $y(t) = e^{rt}$, per cui
$$
r^2e^{rt} + are^{rt} + be^{rt} = 0 \\
\Downarrow \\
e^{rt}(r^2 + ar + b) = 0
$$
deve avere soluzioni.

Dato che $e^{rt} > 0, \forall r, t \in \mathbb{R}$, rimane l'**equazione caratteristica**:
$$
r^2 + ar + b = 0 \\
\Downarrow \\
\Delta = a^2 - 4b
$$

- Se $\Delta > 0$, allora si trova $r_{1,2}$ da cui si ricava:
$$y(t) = c_1e^{r_1t} + c_2e^{r_2t}$$
come _combinazione lineare_ di $y_1 = e^{r_1t}$ e $y_2 = e^{r_2t}$, dato che:
$$
L(y_1) = 0 \land L(y_2) = 0 \Rightarrow L(c_1y_1 + c_2y_2) = 0
$$
con $0 = f(t)$ (perchè _omogenea_), e $L(y) = y'' + ay' + by$. \
\
Per esempio, con $z'' + 2z' - 3z = 0$ si ha che:
$$r^2 + 2r - 3 = 0 \Leftrightarrow z(t) = c_1e^{-3t} + c_2e^t$$
perchè $\Delta = 16 > 0$ e $r_{1,2} = -3, 1$.

- Se $\Delta = 0$, allora $r_1 = r_2$ e si ha:
$$y(t) = c_1e^{r_1t} + c_2te^{r_1t}$$
dato che $y_2 = te^{r_1t}$ è indipendente da $y_1$ (per generare lo spazio delle soluzioni) e $L(y_2) = 0$. \
\
Per esempio, con $z'' + 2\sqrt{3}z' + 3z = 0$ si ha:
$$r^2 + 2\sqrt{3}r + 3 = 0 \Leftrightarrow z(t) = c_1e^{-\sqrt{3}t} + c_2te^{-\sqrt{3}t}$$

- Se $\Delta < 0$, le soluzioni appartengono a $\mathbb{C}$, per cui:
$$r_1 = \alpha + i\beta,\; r_2 = \alpha - i\beta$$
da cui si ricavano le soluzioni **complesse**:
$$
\tilde{y}_1(t) = e^{(\alpha + i\beta)t},\; \tilde{y}_2(t) = e^{(\alpha - i\beta)t} \\
\Downarrow \\
\tilde{y}(t) = \tilde{c}_1\tilde{y}_1(t) + \tilde{c}_2\tilde{y}_2(t) = \tilde{c}_1e^{\alpha t}(\cos(\beta t) + i\sin(\beta t)) + \tilde{c}_2e^{\alpha t}(\cos(\beta t) - i\sin(\beta t)) \\
$$
Si può manipolare $\tilde{y}(t)$ per ottenere due **soluzioni reali**, **ponendo** $\tilde{c}_1 = \tilde{c}_2 = \frac{1}{2}$, $\tilde{c}_1 = -\tilde{c}_2 = \frac{i}{2}$, sostituendo e semplificando, in modo da formare la seguente _combinazione lineare_:
$$
y(t) = c_1e^{\alpha t}\cos(\beta t) + c_2e^{\alpha t}\sin(\beta t)
$$
Per esempio, con $z'' + 2z' + 3z = 0$ si ha:
$$r^2 + 2r + 3 = 0 \Leftrightarrow z(t) = c_1e^{-t}\cos(\sqrt{2}t) + c_2e^{-t}\sin(\sqrt{2}t)$$
dato che $r_1 = -1 + i\sqrt{2}, r_2 = -1 - i\sqrt{2} \Rightarrow \alpha = -1, \beta = \sqrt{2}$.

Di conseguenza, le soluzioni da unire in _combinazione lineare_ sono:

| | $y_1$ | $y_2$ |
|-|:-:|:-:|
| $\Delta > 0$ | $e^{r_1t}$ | $e^{r_2t}$ |
| $\Delta = 0$ | $e^{rt}$ | $te^{rt}$ |
| $\Delta < 0$ | $e^{\alpha t}\cos(\beta t)$ | $e^{\alpha t}\sin(\beta t)$ |

dove $\alpha = \mathrm{Re}(r_1) = \mathrm{Re}(r_2)$ e $\beta = \mathrm{Im}(r_1) = -\mathrm{Im}(r_2)$.

### Lineari complete

Quando $f(t) \neq 0$, si utilizza il **metodo di somiglianza** per trovare una **soluzione particolare** $\bar{y}$ da sommare all'_integrale generale_ dell'**omogenea associata**:
$$
y(t) = z(t) + \bar{y}(t) = c_1z_1(t) + c_2z_2(t) + \bar{y}(t)
$$

Per somiglianza, $\bar{y}$ dipenderà dalla forma di $f(t)$:

- **Polinomiale** di grado $n$:

	Perchè assomigli al polinomio va cercata:
$$
\bar{y} = p_n(t) =
\sum_{i=0}^n \bar{a}_it^{n-i} =
\bar{a}_0t^n + \bar{a}_1t^{n-1} + \bar{a}_2t^{n-2} + ... + \bar{a}_{n-1}t + \bar{a}_n
$$

	Nel caso in cui $b = 0$, bisogna invece cercare $\bar{y} = p_{n+1}(t)$, e se anche $a = 0$, va cercato $\bar{y} = p_{n+2}(t)$.

	Per esempio con $y'' + 3y' - y = 2t^2 - 1$, si cerca:
$$
\bar{y} = p_2(t) = \bar{a}t^2 + \bar{b}t + \bar{c} \\
\Downarrow \\
L(\bar{y}) = -\bar{a}t^2 + (6\bar{a} - \bar{b})t + (2\bar{a} + 3\bar{b} - \bar{c}) = 2t^2 - 1 \\
\Updownarrow \\
\begin{cases}
-\bar{a} = 2 \\
6\bar{a} - \bar{b} = 0 \\
2\bar{a} + 3\bar{b} - \bar{c} = -1
\end{cases} \Rightarrow
\begin{cases}
\bar{a} = -2 \\
\bar{b} = -12 \\
\bar{c} = -39
\end{cases} \\
\Downarrow \\
\bar{y} = -2t^2 - 12t - 39
$$

	Mentre con $y'' + 3y' = 2t^2 - 1$, la funzione da cercare diventa $\bar{y} = p_{2+1}(t) = \bar{a}t^3 + \bar{b}t^2 + \bar{c}t + \bar{d}$.

- **Esponenziale** in forma $f(t) = ke^{nt}$ con $k, n \in \mathbb{R}$:

	In questo caso invece, va cercata:
$$
\bar{y} = \bar{a}e^{nt}
$$

	Se però questa forma è **già presente** nelle soluzioni dell'_omogenea associata_, va **moltiplicata per** $t$ finché non compare più tra le soluzioni trovate con $f(t) = 0$.

	Per esempio, con $y'' + 2y' + y = e^{-t}$:
$$
\bar{y} = \bar{a}t^2e^{-t} = \frac{1}{2}t^2e^{-t}
$$
	perchè entrambe $z_1 = e^{-t}$ e $z_2 = te^{-t}$ sono soluzioni dell'_omogenea associata_.

- **Trigonometrica** in forma $f(t) = k_1 \sin(\omega t) + k_2 \cos(\omega t)$ con $k_{1,2} \in \mathbb{R}$ (incluso lo zero):

	La funzione particolare da cercare sarà:
$$
\bar{y} = \bar{a}\sin(\omega t) + \bar{b}\cos(\omega t)
$$

	Come l'_esponenziale_, se compare nell'_integrale generale_ dell'_omogenea associata_, va **moltiplicata per** $t$.

	Per esempio, con $y'' + y = 2\cos(t)$:
$$
\bar{y} = t(\bar{a}\cos(t) + \bar{b}\sin(t)) = t\sin(t)
$$
	perchè $\Delta < 0$.

### Sovrapposizione degli effetti

Nel caso in cui l'_equazione differenziale_ sia in forma:
$$
y'' + ay' + by = f_1(t) + f_2(t)
$$
la **soluzione particolare** risulterà essere semplicemente:
$$
\bar{y}(t) = \bar{y}_1(t) + \bar{y}_2(t) \
$$
dove $\bar{y}_1$ e $\bar{y}_2$ sono le soluzioni particolari dell'equazione associata a $f_1(t)$ e $f_2(t)$.
