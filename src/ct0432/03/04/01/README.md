# Potenze

$$
\begin{split}
f\colon &D \to \mathbb{R}\newline
&x \mapsto f(x) = x^n
\end{split}
$$
con $n \in \mathbb{R}$, per cui valgono le proprietà:
- $x^{n+m} = x^n \cdot x^m$
- $(x^n)^m = x^{nm}$
- $n < m \Rightarrow \begin{cases} x^n < x^m & \text{se } x > 1 \\ x^n > x^m & \text{se } 0 < x < 1 \end{cases}$, che nel secondo caso è possibile trasformare l'argomento in modo che segui il primo caso: $\left(\frac{1}{x}\right)^{-n} > \left(\frac{1}{x}\right)^{-m} \Rightarrow -n > -m \Rightarrow n < m$.
- $f$ si dice **polinomio di grado** $n$ se $f(x) = a_0 + a_1x + ... + a_nx^n = \sum_{i=0}^n a_ix^i$

Il dominio sarà $D = \mathbb{R}$ e l'immagine:
$$
\mathrm{Im}(f) =
\begin{cases}
[0, +\infty) & \text{se } n \text{ pari} \\
\mathbb{R} & \text{se } n \text{ dispari}
\end{cases}
$$

## Esponente naturale

Nel caso in cui $n \in \mathbb{N}$, la funzione equivale a $x^n = \prod^n x = x \cdot x \cdot ... \cdot x$.

Per cui, se $n$ è:
- **pari**, allora $f$ è **pari** e **strettamente crescente** con $f(x) \geq 0$
- **dispari**, allora $f$ è **dispari** e **strettamente crescente**

## Esponente negativo

La funzione $f(x) = x^{-n} = \frac{1}{x^n}$ avrà dominio $D = \mathbb{R} \setminus \{0\}$ e immagine:
$$
\mathrm{Im}(f) =
\begin{cases}
(0, +\infty) & \text{se } n \text{ pari} \\
\mathbb{R} \setminus \{0\} & \text{se } n \text{ dispari}
\end{cases}
$$

## Esponente $\frac{1}{n}$

La funzione $f(x) = x^{\frac{1}{n}} = \sqrt[n]{x}$, avrà proprietà:
$$
\begin{cases}
D = [0, +\infty), & \mathrm{Im}(f) = [0, +\infty) & \text{se } n \text{ pari} \\
D = \mathbb{R}, & \mathrm{Im}(f) = \mathbb{R} & \text{se } n \text{ dispari}
\end{cases}
$$

## Esponente in $\mathbb{R}$

Quando l'esponente $\alpha \in \mathbb{R}$, il modo per calcolarne il valore è di tendere al numero razionale in $\mathbb{Q}$ più vicino:
$$
f(x) = x^\alpha =
\begin{cases}
\sup(\{x^q : q \leq \alpha, q \in \mathbb{Q}\}) & \text{se } x \geq 1 \\
\inf(\{x^q : q \leq \alpha, q \in \mathbb{Q}\}) & \text{se } 0 < x < 1
\end{cases}
$$

Un'altra tecnica, è quella di ridefinire $x^\alpha$:
$$x^\alpha = e^{\alpha \cdot \ln x}$$
cosa che **impone** che $x > 0$, motivo per cui diventa proprietà nella maggior parte dei casi.

## Esponente razionale

La funzione $f(x) = x^{\frac{m}{n}} = \sqrt[n]{x^m}$, con $\frac{m}{n} \in \mathbb{Q}$ avrà dominio $D = [0, +\infty)$ per la proprietà $x > 0$, e immagine $\mathrm{Im}(f) = [0, +\infty)$, che escluderanno lo $0$ quando $\frac{m}{n} < 0$.
