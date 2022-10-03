# Operazioni tra funzioni

Tra due funzioni $f: D_f \to \mathbb{R}$ e $g: D_g \to \mathbb{R}$ è possibile effettuare le seguenti operazioni:

1. **Somma**

	$$h(x) = f(x) + g(x), \text{ con dominio } D_h = D_f \cap D_g$$

2. **Prodotto**

	$$h(x) = f(x) \cdot g(x), \text{ con dominio } D_h = D_f \cap D_g$$

3. **Rapporto**

	$$h(x) = \frac{f(x)}{g(x)} \text{ con dominio } D_h = \{x \in \mathbb{R} | x \in (D_f \cap D_g) \land g(x) \neq 0\}$$

4. **Composta**

	$$h(x) = (g \circ f)(x) = g(f(x)) \text{ con dominio } D_h = \{x \in \mathbb{R} | x \in D_f \land f(x) \in D_g\}$$

	Per esempio, $f(x) = \sqrt{x}$ e $g(x) = \sqrt{1-x}$, $h(x) = \sqrt{1 - \sqrt{x}}$ e il suo dominio $D_h = [0, 1]$.

## Inverso

L'inverso di una funzione $f: A \to B : x \mapsto y$, è possibile se $f$ **è biettiva** e viene descritta come:
$$
\begin{split}
f^{-1}\colon &B \to A\newline
&y \mapsto x = f^{-1}(y)
\end{split}
$$

Utilizzando la funzione inversa, con $f^{-1}(B) = A$ si ottiene la **controimmagine** e il **dominio** di $f$.

Mentre, $(f \circ f^{-1})(y) = y$ con $y \in B$ e $(f^{-1} \circ f)(x) = x$ con $x \in A$.
