# Proprietà

- **Limiti**:
$$
\lim_{t \to t_0} r(t) = s \in \mathbb{R}^m \Leftrightarrow \lim_{t \to t_0} r_i(t) = s_i, \forall i = 1, ..., m
$$
	e cioè che il limite del vettore è il vettore dei limiti dei componenti.

- **Continuità**:
$$
r(t) \in \mathcal{C}^0 \Leftrightarrow r_i(t) \in \mathcal{C}^0, \forall i = 1, ..., m
$$

	Per esempio, con una curva definita a tratti:
$$
r(t) = \begin{cases}
r_1(t) = (\frac{4}{t}\cos(\pi t), -\frac{4}{t}\sin(\pi t)) & \text{se } 1 \leq t < 4 \\
r_2(t) = (-5t + 21, 0) & \text{se } 4 \leq t \leq 5
\end{cases}
$$
	$r_{1,2}(t)$ sono continue perchè _composizione di funzioni continue_, e $r(t)$ è continua perchè $r_1(4) = r_2(4)$.

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

