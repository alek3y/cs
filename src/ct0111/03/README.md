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

Per esempio, dato $S = \text{"somma di due dadi"}$, allora:
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
dove la funzione $f(x)$ è detta **densità di probabilità**[^2], infatti un $y = f(x_1)$ _delinea_ quanto più è plausibile che una certa _variabile casuale_ $X$ cada vicino ad $x_1$ rispetto ad un altro $x_2$.

Ogni _funzione di densità_ $f(x)$ rispetta le **proprietà** per cui:
- $f(x) \geq 0, \forall x \in \mathbb{R}$
- $\int_{\mathbb{R}} f(x)dx = 1$

Per esempio, se la [curva](https://www.desmos.com/calculator/ilw5sfzh7x) della densità è $f(x) = 2e^{-2x}$ allora $P(X \in (1, 2)) = \int_1^2 f(x)dx \approx 0.117$.

## Funzione di ripartizione

La **funzione di ripartizione**[^3] $F\colon \mathbb{R} \to [0, 1]$ di una _variabile casuale_ $X$ è definita come:
$$
F(x) = P(X \leq x)
$$
che **accumula** le probabilità con il crescere di $x$.

Se $X$ è **discreta**, $F(x) = \sum\limits_{i : x_i \leq x} P(X = x_i)$ e si ricava che:
$$P(X = x) = F(x) - F(x^-) = F(x) - \lim\limits_{t \to x^-} F(t)$$
cioè che la probabilità di $x$ è la [differenza dell'accumulo](https://www.desmos.com/calculator/vwpryzfhmj) su $x$ e il precedente.

Se è **continua** invece, $F(x) = \int_{-\infty}^{x} f(t) dt$, perciò $f(x) = \frac{d}{dx}F(x)$.

### Proprietà

Dalla definizione si ricavano le proprietà delle _funzioni di ripartizione_, per cui $F(x)$ è:
- [Crescente](../../ct0432/03/02/README.md#funzioni-monotone)
- [Continua a destra](../../ct0432/04/03/README.md#determinare-la-continuità), ovvero $\lim\limits_{x \to x_0^+} F(x) = F(x_0), \forall x_0 \in \mathbb{R}$
- $\lim\limits_{x \to -\infty} F(x) = 0$ e $\lim\limits_{x \to +\infty}F(x) = 1$

## Valore atteso

Il **valore atteso** di $X$ è la **media pesata** dalle probabilità dei valori assunti da $X$, per cui nel caso **discreto** è:
$$
E(X) = \sum_{i} x_i P(X = x_i)
$$
mentre nel caso **continuo** è pesata dalla _funzione di densità_ $f(x)$:
$$
E(X) = \int_{\mathbb{R}} xf(x) dx
$$

Quando una _variabile casuale_ $Y$ è esprimibile in termini di $X$ tale che $Y = g(X)$, allora si può porre $E(Y) = \sum_i g(x_i)P_X(x_i)$ se **discreta**, altrimenti $E(Y) = \int_{\mathbb{R}} g(x) f(x)dx$ se **continua**.

Per esempio, il _valore atteso_ di $S$ è $E(S) = 2\frac{1}{36} + 12\frac{1}{36} + 3\frac{2}{36} + 11\frac{2}{36} + 4\frac{3}{36} + ... = 7$.

### Proprietà

Il _valore atteso_ rispetta le proprietà per cui:
- $E(a) = a$
- $E(aX + b) = aE(X) + b$
- $E(X - E(X)) = 0$, dato che visualmente $X - E(X)$ sposta la _media_ di $X$ su $0$

## Varianza

La **varianza** di una _variabile casuale_ $X$ esprime la **media delle distanze** dei valori dal valore ideale $E(X)$:
$$
\mathrm{Var}(X) = E((X - E(X))^2) = E(X^2) - E(X)^2
$$
che nel caso **discreto** è:
$$
\mathrm{Var}(X) = \sum_i (x_i - E(X))^2 P_X(x_i) = \sum_i x_i^2 P_X(x_i) - E(X)^2
$$
mentre nel caso **continuo**:
$$
\mathrm{Var}(X) = \int_{\mathbb{R}} (x - E(X))^2f(x)dx = \int_{\mathbb{R}} x^2f(x)dx - E(X)^2
$$

Per esempio, la _varianza_ di $S$ è $\mathrm{Var}(S) = 2^2\frac{1}{36} + 12^2\frac{1}{36} + 3^2\frac{2}{36} + 11^2\frac{2}{36} + 4^2\frac{3}{36} + ... - 7^2 \approx 5.8$.

### Proprietà

Tra le proprietà della _varianza_, ci sono:
- $\mathrm{Var}(a) = 0$
- $\mathrm{Var}(aX + b) = a^2\mathrm{Var}(X)$

## Moda

Le **mode** della _variabile casuale_ $X$ sono tutti i valori di [massimo relativo](../../ct0432/05/04/README.md#proprietà-locali) della _funzione_ o _densità di probabilità_.

Per esempio, l'unica _moda_ di $S$ è su $7$ perchè è dove $P_S(s)$ assume il valore massimo.

Un altro esempio considera l'urna $\{1, 1, 2, 2, 3, 3, 4\}$ e $X = \text{"numero estratto"}$, da cui si ricavano le _mode_ in $x = 1, 2, 3$ perchè $P_X(x) = \frac{2}{7}$ che sono valori maggiori del rimanente $P_X(4) = \frac{1}{7}$.

## Mediana

Si definisce **mediana** il minimo valore $\mathrm{med}(X) = m$ per cui:
$$
F(m) = P(X \leq m) = \frac{1}{2}\ \land\ P(X \geq m) = \frac{1}{2}
$$

Nel caso **discreto** basta che $P(X \leq m) \geq \frac{1}{2}$ dato che $X = m$ potrebbe essere **impossibile**.

Per esempio, nel lancio di un dado $P(X = 1,2,3) = P(X = 4,5,6) = \frac{1}{2}$ per cui la _mediana_ dovrebbe essere un valore in $(3, 4)$, ma $X \not\in (3, 4)$ di conseguenza si sceglie come _mediana_ $m = 3$.

Nel caso **continuo** invece, si rispetta la condizione per cui $\int_{-\infty}^m f(x)dx = \int_m^\infty f(x)dx = \frac{1}{2}$.

## Quantili

Un **quantile di livello** $\alpha$ è il minimo valore $q_\alpha$ per cui, nel caso **discreto**:
$$
F(q_\alpha) = P(X \leq q_\alpha) = \alpha
$$
che nel caso **discreto** sarà sufficiente la condizione $F(q_\alpha) \geq \alpha$.

Servono a generalizzare divisioni del grafico come la _mediana_, che infatti è un _quantile_ di livello $\alpha = \frac{1}{2}$.

[^1]: [_Probability mass function_](https://en.wikipedia.org/wiki/Probability_mass_function)

[^2]: [_Probability density function_](https://en.wikipedia.org/wiki/Probability_density_function)

[^3]: [_Cumulative distribution function_](https://en.wikipedia.org/wiki/Cumulative_distribution_function)
