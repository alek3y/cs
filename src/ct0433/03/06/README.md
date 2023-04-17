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

## Piano tangente

Il **piano tangente** su un punto $(x_0, y_0, f(x_0, y_0))$ corrisponde allo _spazio_ di tutti i vettori tangenti su quel punto, e quindi a tutte le direzioni che il punto può seguire.

Per trovare il piano in forma $z = ax + by + c$ passante per il punto, si pone che:
$$
\begin{cases}
f(x_0, y_0) = ax_0 + by_0 + c \\
\frac{\partial f}{\partial x}(x_0, y_0) = a \\
\frac{\partial f}{\partial y}(x_0, y_0) = b \\
\end{cases}
$$
da cui si ricava:
$$
T_{f,(x_0, y_0)}(x, y) = \frac{\partial f}{\partial x}(x_0, y_0)(x - x_0) + \frac{\partial f}{\partial y}(x_0, y_0)(y - y_0) + f(x_0, y_0)
$$

## Funzione differenziabile

Una funzione $f\colon A \subseteq \mathbb{R}^2 \to \mathbb{R}$ si dice **differenziabile** su un punto $(x_0, y_0) \in \mathring{A}$ se può essere **approssimata** (**a meno di un resto**) dal _piano tangente_ su un intorno abbastanza piccolo del punto.

Per cui, se $(x_0, y_0) + (h, k)$ è nell'intorno e soddisfa la condizione, allora:
$$
f(x_0 + h, y_0 + k) = T_{f,(x_0, y_0)}(x_0 + h, y_0 + k) + o(\|(h, k)\|)
$$
dove $o(\|(h, k)\|)$ è il **resto** di tipo [_infinitesimo superiore_](../../../ct0432/07/README.md#funzioni-infinitesime) di $\|(h, k)\|$.

Se $f$ è _differenziabile_ in $(x_0, y_0)$, allora $f$ è anche _derivabile_ e _continua_ su $(x_0, y_0)$. \
Inoltre, se $f \in C^1(U_r((x_0, y_0)))$, allora $f$ è _differenziabile_ in $(x_0, y_0)$.

Di conseguenza, se $\mathrm{Dom}(f)$ è [_aperto_](../03/README.md#proprietà) e $f \in C^1(\mathrm{Dom}(f))$ allora $f$ è _differenziabile_ in $\mathrm{Dom}(f)$.

Per esempio, se $f(x, y) = ye^x + x^2 + 2xy + y^3$ allora è _differenziabile_ se:
$$
f \in C^1 \Leftrightarrow \exists \frac{\partial f}{\partial x}, \frac{\partial f}{\partial y} \in C^0
$$
infatti, $\frac{\partial f}{\partial x}(x, y) = ye^x + 2x + 2y$ e $\frac{\partial f}{\partial y}(x, y) = e^x + 2x + 3y^2$ sono continue **nel loro dominio**.

## Derivata direzionale

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

## Massimi e minimi

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
