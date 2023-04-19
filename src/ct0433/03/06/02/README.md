# Derivata direzionale

Si dice **derivata direzionale** di $f$ rispetto al **versore** $\vec{v}$ ([vettore unità](../../../ct0435/02/01/README.md#vettore-unità)) nel punto $\vec{x}_0$ il limite:
$$
D_{\vec{v}}(\vec{x}_0) =
\lim_{t \to 0} \frac{f(\vec{x}_0 + t\vec{v}) - f(\vec{x}_0)}{t} =
\nabla f^T(\vec{x}_0) \cdot \vec{v} =
\sum_{i = 1}^n \frac{\partial f}{\partial x_i}(\vec{x}_0) \cdot v_i
$$
cioè la _derivata_ su $\vec{x}_0$ di $f$ **rispetto alla sezione** verticale definita dalla [retta](../../../ct0435/03/README.md#retta) $\vec{x}_0 + t\vec{v}$.

Infatti, se $\vec{v} = \hat{x} = (1, 0)$, cioè la sezione lungo l'asse $x$, e $\vec{x}_0 = (x, y)$:
$$
D_{\hat{x}}(x, y) = \frac{\partial f}{\partial x}(x, y) \cdot 1 + \frac{\partial f}{\partial y}(x, y) \cdot 0 = \frac{\partial f}{\partial x}(x, y)
$$

Per esempio, se $f(x, y) = y^3 - \sin(xy)$, $\vec{v} = \left(\frac{\sqrt{2}}{2}, -\frac{\sqrt{2}}{2}\right)$ e $\vec{x}_0 = (\pi, 1)$ la sezione corrisponde ai valori di $f$ che assume quando $x$ e $y$ appartengono alla retta $\vec{x}_0 + t\vec{v} = \left(\pi + \frac{\sqrt{2}}{2}t, 1 - \frac{\sqrt{2}}{2}t\right)$, per cui:
$$
\begin{split}
D_{\vec{v}}(\vec{x}_0) &= \nabla f^T (\pi, 1) \cdot \begin{pmatrix}\frac{\sqrt{2}}{2} \\ -\frac{\sqrt{2}}{2}\end{pmatrix} =
(-1 \cdot \cos(\pi \cdot 1), 3 \cdot 1^2 -\pi\cos(\pi \cdot 1)) \cdot
\begin{pmatrix}\frac{\sqrt{2}}{2} \\ -\frac{\sqrt{2}}{2}\end{pmatrix} = \\
&= (1, 3 + \pi) \cdot \begin{pmatrix}\frac{\sqrt{2}}{2} \\ -\frac{\sqrt{2}}{2}\end{pmatrix} =
-\sqrt{2} - \frac{\sqrt{2}}{2}\pi
\end{split}
$$

## Direzione di massima crescita

Il _vettore unità_ $\vec{v}$ descrive la **direzione di massima crescita** su un punto $\vec{x}_0$ quando $D_{\vec{v}}(\vec{x}_0) = \nabla f^T(\vec{x}_0) \cdot v$ assume il valore più grande, cioè quando $\cos(\alpha) = 1$ (sul [prodotto scalare](../../../ct0435/02/01/README.md#operazioni)).

Di conseguenza, se il $\vec{v}$ più grande ha $\alpha = 0$ con $\nabla f(\vec{x}_0)$, allora:
$$
\vec{v} = \frac{\nabla f(\vec{x}_0)}{\|\nabla f(\vec{x}_0)\|}
$$
dove $-\vec{v}$ è la **direzione di minima crescita**.

Per esempio, la _direzione di massima crescita_ su $\vec{x}_0 = (\pi, 1)$ se $f(x, y) = y^3 - \sin(xy)$ è:
$$
\vec{v} = \frac{\nabla f(\pi, 1)}{\|\nabla f(\pi, 1)\|} = \frac{1}{\sqrt{1 + (3 + \pi)^2}} \cdot \begin{pmatrix}1 \\ 3 + \pi\end{pmatrix}
\Leftarrow \nabla f(x, y) = \begin{pmatrix}-y\cos(xy) \\ 3y^2 - x\cos(xy)\end{pmatrix}
$$
