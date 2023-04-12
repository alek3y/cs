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
D_{\vec{v}}(\vec{x}_0) = \lim_{t \to 0} \frac{f(\vec{x}_0 + t\vec{v}) - f(\vec{x}_0)}{t}
$$
cioè la _derivata_ di $f$ **rispetto alla sezione** verticale definita dalla [retta](../../../ct0435/03/README.md#retta) $\vec{x}_0 + t\vec{v}$.

Infatti, $\frac{\partial f}{\partial x}(x, y) = D_{\hat{x}}(x, y)$ dove $\hat{x} = (1, 0)$.

Per esempio, se $f(x, y) = y^3 - \sin(xy)$, $\vec{v} = \left(\frac{\sqrt{2}}{2}, -\frac{\sqrt{2}}{2}\right)$ e $\vec{x}_0 = (\pi, 1)$ la sezione corrisponde ai valori di $f$ che assume quando $x$ e $y$ appartengono alla retta $\vec{x}_0 + t\vec{v} = \left(\pi + \frac{\sqrt{2}}{2}t, 1 - \frac{\sqrt{2}}{2}t\right)$, cioè:
$$
g(t) = f(\vec{x}_0 + t\vec{v}) = f\left(\pi + \frac{\sqrt{2}}{2}t, 1 - \frac{\sqrt{2}}{2}t\right)
$$
da cui si trova che:
$$
D_{\vec{v}}(\vec{x}_0) = \frac{dg}{dt}(0) = g'(0) = -\sqrt{2} - \frac{\sqrt{2}}{2}\pi
$$
