# Algebra delle derivate

Le operazioni principali tra due funzioni $f$ e $g$, derivabili in $x_0$, comprendono:

- **Derivata della somma**

	$$\mathrm{D}(f + g)(x_0) = f'(x_0) + g'(x_0)$$

- **Derivata del prodotto**

	$$\mathrm{D}(f \cdot g)(x_0) = f'(x_0)g(x_0) + f(x_0)g'(x_0)$$
	che sarebbe la **regola di Leibniz**.

- **Derivata del rapporto**

	$$\mathrm{D}\left(\frac{f}{g}\right)(x_0) = \frac{f'(x_0)g(x_0) - f(x_0)g'(x_0)}{g^2(x_0)}$$
	con $g(x_0) \neq 0$.

- **Derivata di una combinazione lineare**

	$$\mathrm{D}(af + bg)(x_0) = af'(x_0) + bg'(x_0)$$
	con $a, b \in \mathbb{R}$.

- **Derivata di una funzione composta**

	$$\mathrm{D}(f \circ g)(x_0) = f'(g(x_0)) \cdot g'(x_0)$$
	con $f$ derivabile in $g(x_0)$ e $g$ derivabile in $x_0$.

- **Derivata di una funzione composta esponenziale**

	$$\mathrm{D}(f^g)(x_0) = f(x_0)^{g(x_0)} \cdot \left(g'(x_0)\ln(f(x_0)) + \frac{g(x_0) \cdot f'(x_0)}{f(x_0)}\right)$$
	con $f$ derivabile in $g(x_0)$ e $g$ derivabile in $x_0$.

- **Derivata della funzione inversa**

	$$\mathrm{D}(f^{-1})(y_0) = \frac{1}{f'(f^{-1}(y_0))}$$
	dove $y_0 \in \mathrm{Dom}(f^{-1}) = \mathrm{Codom}(f)$ è il punto su cui $f^{-1}$ è derivabile.
