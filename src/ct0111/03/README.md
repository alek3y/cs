# Variabili casuali

Una **variabile casuale** (o _aleatoria_) è una funzione $X\colon \Omega \to \R$, che associa gli eventi ad un valore finito.
Questo **semplifica** gli eventi _complicati_ di interesse, ricavandoci una **proprietà** $X$ su cui operare.

La **probabilità** che $X$ assuma un **valore di interesse** riportato nell'insieme $A$, sarà:
$$
P_X(A) = P(X \in A) = P(\{\omega \in \Omega \mid X(\omega) \in A\})
$$

## Discrete

Una _variabile casuale_ è detta **discreta** se l'[immagine](../../ct0432/03/01/README.md#immagine) $X(\Omega) \subseteq \mathbb{N}$.

La **funzione di probabilità**[^1] di un valore _discreto_ $x$ assunto da $X$ sarà $P_X(x) = P(X = x)$, da cui:
$$
P(X \in A) = \sum_{x \in A} P_X(x)
$$
cioè la somma di tutte le probabilità dei valori di interesse contenuti in $A$.

Per esempio dato $S = \text{"somma di due dadi"}$, allora:
$$
P_S(s) = \begin{cases}
\frac{1}{36} & \text{se } s = 2, 12 \\
\frac{2}{36} & \text{se } s = 3, 11 \\
\frac{3}{36} & \text{se } s = 4, 10 \\
\frac{4}{36} & \text{se } s = 5, 9 \\
\frac{5}{36} & \text{se } s = 6, 8 \\
\frac{6}{36} & \text{se } s = 7 \\
0 & \text{altrimenti}
\end{cases}
$$

## Continue

Una _variabile casuale_ è detta **continua** invece, se $X(\Omega) \subseteq \mathbb{R}$ ed è [non numerabile](../../ct0434/04/01/README.md#biezione).

La **probabilità** di assumere un qualsiasi valore nell'insieme continuo $A$, sarà:
$$
P(X \in A) = \int_A f(x)dx
$$
dove la funzione $f(x)$ è detta **densità di probabilità**[^2], infatti un $y = f(x_1)$ _delinea_ quanto più è plausibile che una certa _variable casuale_ $X$ cada vicino ad $x_1$ rispetto ad un altro $x_2$.

Per esempio, se la [curva](https://www.desmos.com/calculator/ilw5sfzh7x) della densità è $f(x) = 2e^{-2x}$ allora $P(X \in (1, 2)) = \int_1^2 f(x)dx \approx 0.117$.

[^1]: [_Probability mass function_](https://en.wikipedia.org/wiki/Probability_mass_function)

[^2]: [_Probability density function_](https://en.wikipedia.org/wiki/Probability_density_function)
