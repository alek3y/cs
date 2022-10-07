# Funzioni elementari

## Potenza

$$
\begin{split}
f\colon &D \to \mathbb{R}\newline
&x \mapsto f(x) = x^n
\end{split}
$$
con $n \in \mathbb{R}$, per cui valgono le proprietà:
- $x^{n+m} = x^n \cdot x^m$
- $(x^n)^m = x^{nm}$
- $f$ si dice **polinomio di grado** $n$ se $f(x) = a_0 + a_1x + ... + a_nx^n = \sum_{i=0}^n a_ix^i$

Il dominio sarà $D = \mathbb{R}$ e l'immagine:
$$
\mathrm{Im}(f) =
\begin{cases}
[0, +\infty) & \text{se } n \text{ pari} \\
\mathbb{R} & \text{se } n \text{ dispari}
\end{cases}
$$

### Esponente naturale

Nel caso in cui $n \in \mathbb{N}$, la funzione equivale a $x^n = \prod^n x = x \cdot x \cdot ... \cdot x$.

Per cui, se $n$ è:
- **pari**, allora $f$ è **pari** e **strettamente crescente** con $f(x) \geq 0$
- **dispari**, allora $f$ è **dispari** e **strettamente crescente**

### Esponente negativo

La funzione $f(x) = x^{-n} = \frac{1}{x^n}$ avrà dominio $D = \mathbb{R} \setminus \{0\}$ e immagine:
$$
\mathrm{Im}(f) =
\begin{cases}
(0, +\infty) & \text{se } n \text{ pari} \\
\mathbb{R} \setminus \{0\} & \text{se } n \text{ dispari}
\end{cases}
$$
