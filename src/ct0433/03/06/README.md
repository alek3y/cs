# Derivate

Si dice **derivata parziale** di una funzione $f\colon \mathbb{R}^2 \to \mathbb{R}$:
$$
\frac{\partial f}{\partial x}(x_0, y_0) =
\lim_{h \to 0} \frac{f(x_0 + h, y_0) + f(x_0, y_0)}{h}
$$
cioè il _coefficiente angolare_ di $f$ sul punto $x_0$ intorno alle $x$ su una sezione $y_0$.

Per esempio, se $f(x, y) = 2xy^2 + x^3$ allora $\frac{\partial f}{\partial x}(x, y) = 2y^2 + 3x^2$ e $\frac{\partial f}{\partial y}(x, y) = 4xy$.

## Vettore gradiente

Il **vettore gradiente** di una funzione $f\colon \mathbb{R}^n \to \mathbb{R}$ corrisponde a:
$$
\nabla f(x_1, x_2, ..., x_n) = \begin{pmatrix}
\frac{\partial f}{\partial x_1} \\[0.5em]
\frac{\partial f}{\partial x_2} \\
\vdots \\
\frac{\partial f}{\partial x_n}
\end{pmatrix}
$$
cioè il vettore che, partendo da un punto $(x_1, ..., x_n)$, indica la direzione di **salita più rapida**.
Questo non corrisponde al _vettore tangente_, perchè indica la direzione che il punto deve prendere nel dominio di $f$.
