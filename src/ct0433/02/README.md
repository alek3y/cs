# Curve parametriche

Una curva è una **funzione vettoriale** che associa un **parametro** $t$ ad un punto nel piano $m$-dimensionale:
$$
\begin{split}
r\colon &I \subseteq \mathbb{R} \to \mathbb{R}^m \\
&t \mapsto (r_1(t), r_2(t), ..., r_m(t))
\end{split}
$$

Per disegnare la curva va usato il **sostegno alla curva parametrica** $\gamma$, che corrisponde al grafico senza $t$:
$$
\gamma(r) = r(I) = \mathrm{Im}(r) \subset \mathrm{Codom}(r)
$$
infatti, se $r(t) \in \mathbb{R}^2$, il **sostegno** $\gamma(r) \subset \mathbb{R}^2$, mentre il [grafico](../../ct0432/03/README.md#grafico) $G(r) \subset \mathbb{R}^3$ perchè anche $t$ venga tracciata.

Per esempio, se $r(t) = (t^2 + t, 2t - 1)$ e $-1 \leq t \leq 1$:

![Esempio curva di parabola](assets/01.png)

si ottiene un'**arco di curva** di una _parabola capovolta_, perchè $t$ appartiene ad un **intervallo** $I \neq \mathbb{R}$.

Altri esempi includono:
- $r(t) = (r\cos(\theta) + x_0, r\sin(\theta) + y_0)$, cioè la circonferenza $(x - x_0)^2 + (y - y_0)^2 = r^2$.
- $r(t) = (a\cos(\theta) + x_0, b\sin(\theta) + y_0)$, cioè l'ellissi $\frac{(x - x_0)^2}{a^2} + \frac{(y - y_0)^2}{b^2} = 1$

## Proprietà

- **Limiti**:
$$
\lim_{t \to t_0} r(t) = s \in \mathbb{R}^m \Leftrightarrow \lim_{t \to t_0} r_i(t) = s_i, \forall i = 1, ..., m
$$
	e cioè che il limite del vettore è il vettore dei limiti dei componenti.

- **Continuità**:
$$
r(t) \in \mathcal{C}^0 \Leftrightarrow r_i(t) \in \mathcal{C}^0, \forall i = 1, ..., m
$$

- **Derivabilità**:
$$
r(t) \in C^1 \Leftrightarrow r_i(t) \in C^1, \forall i = 1, ..., m \\
\Downarrow \\
r'(t) = (r_1'(t), r_2'(t), ..., r_m'(t))
$$

	Di conseguenza, $r'(t_0)$ sarà il **vettore velocità** e $r''(t_0)$ sarà il **vettore accelerazione** entrambi al tempo $t_0$, come si può vedere su [questo esempio](https://www.desmos.com/calculator/fpbcyrlvvz).

	Trovando le norme $\|r'(t_0)\|$ e $\|r''(t_0)\|$ si trovano quindi la **velocità** e **accelerazione scalare** in $t_0$.

- **Integrabilità**:
$$
r(t) \in \mathcal{R} \Leftrightarrow r_i(t) \in \mathcal{R}, \forall i = 1, ..., m \\
\Downarrow \\
\int r(t) dt = \left(\int r_1(t) dt, \int r_2(t) dt, ..., \int r_m(t) dt\right)
$$
	dove $\mathcal{R}$ è l'insieme di tutte le funzioni integrabili in $I$ (secondo _Reimann_).

## Tipi di curve

Una curva parametrica $r(t)$ con $t \in I = [a, b]$, può essere:
- **Chiusa**, se la fine della curva **si riconnette** all'inizio, cioè:
$$
r(a) = r(b)
$$
- **Semplice**, se **non si interseca mai** (escludendo gli estremi per quando è _chiusa_), cioè:
$$
r(t_1) = r(t_2) \Leftrightarrow t_1 = t_2
$$
dove $t_1 \in I$ e $t_2 \in \mathring{I} = I \setminus \{a, b\}$.
