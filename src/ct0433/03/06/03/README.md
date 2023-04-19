# Massimi e minimi

Avendo una funzione $f\colon A \subseteq \mathbb{R}^n \to \mathbb{R}$, un punto $\vec{x}_0 \in A$ (detto estremo) per $f$ è:
- **Massimo assoluto** (o _globale_) se $\forall \vec{x} \in A, f(\vec{x}) \leq f(\vec{x}_0)$
- **Minimo assoluto** se $\forall \vec{x} \in A, f(\vec{x}) \geq f(\vec{x}_0)$
- **Massimo relativo** (o _locale_) se $\exists r > 0 : \forall \vec{x} \in (U_r(\vec{x}_0) \cap A), f(\vec{x}) \leq f(\vec{x}_0)$
- **Minimo relativo** se $\exists r > 0 : \forall \vec{x} \in (U_r(\vec{x}_0) \cap A), f(\vec{x}) \geq f(\vec{x}_0)$

Inoltre, se $\vec{x}_0$ è uno dei precedenti, allora può anche essere:
- **Vincolante** se $\vec{x}_0 \in \partial A$, cioè appartenenti alla _frontiera_
- **Libero** (o _non vincolante_) se $\vec{x}_0 \in \mathring{A}$

Dal **teorema di Wierstrass**, se $A$ è [_chiuso_ e _limitato_](../03/README.md#proprietà) e $f \in C^0$ allora **esistono** _massimi_ e _minimi_ assoluti in $A$.

## Punti critici

Un punto $\vec{x}_0 \in A$ con $A = \mathrm{Dom}(f)$ è detto **critico** (o _stazionario_) se:
$$
\nabla f(\vec{x}_0) = \vec{0}
$$

Per il **teorma di Fermat**, se $\vec{x}_0 \in \mathring{A}$ è _massimo_ o _minimo locale_ e $f \in C^1(\{x_0\})$ allora $\vec{x}_0$ è _punto critico_.

Per esempio, se $f(x, y) = x^2 + 3(y - 1)^2$ con $\mathrm{Dom}(f) = \mathbb{R}^2$ si hanno _punti critici_ quando:
$$
\nabla f(x, y) = \begin{pmatrix}2x \\ 6(y - 1)\end{pmatrix} = \vec{0} \Leftrightarrow
\begin{cases}
2x = 0 \\
6(y - 1) = 0
\end{cases}
$$
ovvero sul punto $(0, 1)$ che è anche _minimo assoluto_, dato che altri $(x, y)$ danno valori più grandi.

Oppure, per esempio con $f(x, y) = x^2 - y^3$:
$$
\mathrm{Dom}(f) = \mathbb{R}^2 \Rightarrow \nabla f(x, y) = \begin{pmatrix}2x \\ -3y^2\end{pmatrix} = \vec{0}
$$
da cui $(0, 0)$ è punto _stazionario_.
Non è però _minimo_ perchè non lo è sulla _sezione_ $x = 0$ di $f$, cioè $-3y^2$.
