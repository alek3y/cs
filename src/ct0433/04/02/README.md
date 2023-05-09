# Cambio di variabili

Per funzioni in $\mathbb{R}$, se $x = \phi(t)$ con $\phi \in C^1$ e invertibile:
$$
\int_a^b f(x) dx = \int_{\phi^{-1}(a)}^{\phi^{-1}(b)} f(\phi(t))\phi'(t) dt
$$

In $\mathbb{R}^2$ invece, va trasformato il dominio $D$ con una [trasformazione](../../../ct0435/08/README.md) $T \in C^1$ invertibile:
$$
\begin{split}
T\colon D' \subset \mathbb{R}^2 &\to D \subset \mathbb{R}^2 \\
(u, v) &\mapsto (x, y) = (g(u, v), h(u, v))
\end{split}
$$
la cui **matrice Jacobiana** sarà la matrice $J_T$ delle _derivate parziali_:
$$
J_T(u, v) = \begin{pmatrix}
\frac{\partial g}{\partial u}(u, v) & \frac{\partial g}{\partial v}(u, v) \\
\frac{\partial h}{\partial u}(u, v) & \frac{\partial h}{\partial v}(u, v)
\end{pmatrix}
$$
i cui punti $(u, v)$ per cui $\det(J_T(u, v)) = 0$ vengono detti **punti critici**.

Inoltre, se per ogni $(u, v) \in D'$ il $\det(J_T(u, v)) \neq 0$, allora $T$ è detto **diffeomorfismo** e $T^{-1} \in C^1$.

Per esempio, se la trasformazione è in [coordinate polari](../../02/03/README.md#coordinate-polari):
$$
\begin{split}
T\colon [0, +\infty[ \times [0, 2\pi[ &\to \mathbb{R}^2 \\
(p, \theta) &\mapsto (x, y) = (p\cos(\theta), p\sin(\theta))
\end{split} \\
\Downarrow \\
J_T(p, \theta) = \begin{pmatrix}
\cos(\theta) & -p\sin(\theta) \\
\sin(\theta) & p\cos(\theta)
\end{pmatrix} \Rightarrow \det(J_T(p, \theta)) = p
$$
e quindi $(u, v) = (0, \theta)$ (i.e. l'origine) è un _punto critico_.

La **sostituzione** avviene quindi con:
$$
\iint_D f(x, y) dx dy = \iint_{D'} f(g(u, v), h(u, v)) |\det(J_T)|\ du\ dv
$$

Per esempio, se $D = \{(x, y) \in \mathbb{R}^2 : x^2 + 4y^2 \leq 1 \land x \leq 0 \land y \geq 0\}$, allora l'ellissi nel secondo quadrante può essere rappresentata da:
$$
\begin{split}
T\colon [0, 1] \times \left[\frac{\pi}{2}, \pi\right] &\to D \\
(p, \theta) &\mapsto (x, y) = \left(p\cos(\theta), p\frac{1}{2}\sin(\theta)\right)
\end{split} \\
\Downarrow \\
\begin{split}
\iint_D y^2 + 1 dx dy &=
\iint_{[0, 1] \times \left[\frac{\pi}{2}, \pi\right]} \left(\left(p\frac{1}{2}\sin(\theta)\right)^2 + 1\right) \left|\frac{1}{2}p\right| dp\ d\theta = \\
&= \int_{\frac{\pi}{2}}^{\pi} \int_0^1 \frac{1}{8}p^3\sin^2(\theta) + \frac{1}{2}p\ dp\ d\theta =
\frac{1}{32} \int_{\frac{\pi}{2}}^{\pi} \sin^2(\theta) + 8\ d\theta = \frac{17\pi}{128}
\end{split}
$$
