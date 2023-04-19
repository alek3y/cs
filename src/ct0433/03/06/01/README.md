# Piano tangente

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
