# Limiti

## Successioni

Un limite in $\mathbb{R}^n$ può essere definito su una [successione](../../../ct0432/08/README.md) di punti $\{x_k : k \in \mathbb{N}^+\}$, per cui:
$$
\lim_{k \to +\infty} x_k = x_0 \Leftrightarrow \lim_{k \to +\infty} \|x_k - x_0\| = 0
$$
allora il limite di una funzione $f\colon \mathbb{R}^n \to \mathbb{R}$ per una successione $\{x_k\}$ convergente in $x_0$, sarà:
$$
\lim_{k \to +\infty} f(x_k) = l \Leftrightarrow \lim_{x \to x_0} f(x) = l
$$

## Definizione

Più in generale, [secondo la definizione](../../../ct0432/04/README.md#definizione-epsilon-delta) **epsilon-delta**, il limite di una funzione $f\colon A \subset \mathbb{R}^n \to \mathbb{R}$ definita _almeno_ nell'intorno $U_r(x_0) \setminus x_0$ sarà:
$$
\lim_{x \to x_0} f(x) = l \in \mathbb{R} \\
\Updownarrow \\
\forall \epsilon > 0, \exists \delta > 0 : 0 < \| x - x_0 \| < \delta \Rightarrow |f(x) - l| < \epsilon
$$

Se $x \to \infty$, si prende un $M > 0$ (ovvero un punto in $\mathbb{R}$ distante $M$ dall'origine) per cui i punti $x$ che hanno $\|x\| > M$ (ovvero sono più distanti di $M$) portano $f(x) \to l$, cioè $\|x\| > M \Rightarrow \|f(x) - l \| < \epsilon$.

## Continuità

Una funzione $f$ è **continua** in $x_0 \in \mathrm{Dom}(f) \subset \mathbb{R}^n$ se:
$$
\lim_{x \to x_0} f(x) = f(x_0)
$$

Per esempio, se $f\colon \mathbb{R}^2 \to \mathbb{R}$ è:
$$
f(x, y) = \begin{cases}
f_1(x, y) = x^2 + y^2 - 1 & \text{se } y > 2x \\
f_2(x, y) = -2x - y & \text{se } y \leq 2x
\end{cases}
$$
sappiamo che $f_{1,2} \in C^0$, allora bisogna capire se $f \in C^0$ quando $y = 2x$:
$$
f_1(x, 2x) = f_2(x, 2x) \Leftrightarrow x^2 + 4x^2 - 1 = -2x - 2x \Leftrightarrow x_{1,2} = \frac{1}{5}, -1
$$
allora $f$ è continua solamente sui punti $(\frac{1}{5}, 2 \cdot \left(\frac{1}{5}\right))$ e $(-1, 2 \cdot (-1))$ lungo $y = 2x$.
