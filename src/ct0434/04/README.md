# Funzioni

Parte dell'argomento è già stato trattato nella parte di [Calcolo 1](../../ct0432/03/README.md).

## Funzioni identità

Una funzione si dice **identità** di $A$ quando
$$
\begin{split}
i_A\colon &A \to A \\
&x \mapsto i_A(x) = x
\end{split}
$$
ed è quindi **biettiva**.

## Funzione inclusione

Si dice **inclusione** invece, quando porta $x$ da un dominio $A \subseteq B$ ad un codominio $B$:
$$
\begin{split}
j\colon &A \to B \\
&x \mapsto j(x) = x
\end{split}
$$
e quindi "porta" gli elementi da $A$ a $B$.

## Funzioni di arrotondamento

L'arrotondamento **in difetto** è espresso dalla funzione **floor**
$$
\begin{split}
\lfloor \rfloor\colon &\mathbb{R} \to \mathrm{Z} \\
&x \mapsto \lfloor x \rfloor = max(\{z \in \mathbb{Z} | z \leq x\})
\end{split}
$$

Mentre quello **in eccesso** è espresso dalla funzione **ceil**
$$
\begin{split}
\lceil \rceil\colon &\mathbb{R} \to \mathrm{Z} \\
&x \mapsto \lceil x \rceil = min(\{z \in \mathbb{Z} | x \leq z\})
\end{split}
$$

## Funzioni composte

$$
f\colon A \to B, \hspace{1em} g\colon B \to C
$$
$$
\begin{split}
h\colon &A \to C \\
&x \mapsto (g \circ f)(x) = g(f(x))
\end{split}
$$

Nelle funzioni composte non vale la proprietà **commutativa**, perchè $(g \circ f)(x) \neq (f \circ g)(x)$, ma vale la proprietà **associativa**, perchè $h \circ (g \circ f) = (h \circ g) \circ f$.

Quando $(f^2)(x) = (f \circ f)(x) = f(f(x)) = f(x)$, allora $f$ si dice **idempotente**.

Se le funzioni coinvolte sono **suriettive** o **iniettive** lo sarà anche la funzione composta tra le due.

## Esempio

$$
\begin{split}
f\colon &\mathbb{R} \to \mathbb{R} \\
&x \mapsto f(x) = ax + b, a \neq 0
\end{split}
$$

1. **Iniettiva**

	Perchè la funzione sia iniettiva, deve valere che
	$$\forall x_1, x_2 \in \mathbb{R}, f(x_1) = f(x_2) \Rightarrow x_1 = x_2$$
	di conseguenza si suppone che $f(x_1) = f(x_2)$.

	$$f(x_1) = f(x_2) \Leftrightarrow ax_1 + b = ax_2 + b \Leftrightarrow x_1 = x_2, a \neq 0$$
	e quindi lo è.

2. **Suriettiva**

	Perchè lo sia deve valere che
	$$\forall y \in \mathbb{R}, \exists x \in \mathbb{R} : y = f(x)$$
	per cui basta assicurarsi che per tutte le $y$ esista una $x$:
	$$y = ax + b \Leftrightarrow y - b = ax \Leftrightarrow x = \frac{y - b}{a}, a \neq 0$$
	e quindi lo è.

	Se $a = 0$, allora la proprietà non varrebbe per _tutte_ le $y$.
