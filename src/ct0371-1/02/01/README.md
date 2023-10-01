# Proprietà

- $$\Theta(g(n)) = O(g(n)) \cap \Omega(g(n))$$

- $$f(n) = O(g(n)) \Leftrightarrow \Omega(f(n)) = g(n)$$

	Assumendo $f(n) = O(g(n))$ vero, si ha:
	- **Ipotesi**: $\exists c > 0, n_0 \in \mathbb{N} : \forall n \geq n_0, f(n) \leq c \cdot g(n)$, quindi $\frac{1}{c} \cdot f(n) \leq g(n)$ dato che $c > 0$

	- **Tesi**: $\exists c' > 0, n_0' \in \mathbb{N} : \forall n \geq n_0', c' \cdot f(n) \leq g(n)$

	Per cui con $c' = \frac{1}{c}$ e $n_0' = n_0$ la [_condizione sufficiente_](../../ct0434/01/README.md#doppia-implicazione) è verificata e la _condizione necessaria_ è analoga.

- $$f(n) = O(g(n)) \land g(n) = O(h(n)) \Rightarrow f(n) = O(h(n))$$

	Assumendo la _condizione sufficiente_ per vera, si ha che:
	- **Ipotesi**:
		- $\exists c_1 > 0, n_1 \in \mathbb{N} : \forall n \geq n_1, f(n) \leq c_1 \cdot g(n)$
		- $\exists c_2 > 0, n_2 \in \mathbb{N} : \forall n \geq n_2, g(n) \leq c_2 \cdot h(n)$, quindi $c_1 \cdot g(n) \leq c_1c_2 \cdot h(n)$

	- **Tesi**: $\exists c_3 > 0, n_3 \in \mathbb{N} : \forall n \geq n_3, f(n) \leq c_3 h(n)$

	Per cui $f(n) \leq c_1 \cdot g(n) \leq c_1c_2 \cdot h(n)$ quindi $c_3 = c_1c_2$ e $n_3 = \max(n_1, n_2)$.
