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
