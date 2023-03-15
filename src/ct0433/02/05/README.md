# Parametro d'arco

## Riparametrizzazione

Se due **archi di curva**
$$
\begin{split}
r_1\colon [a, b] &\to \mathbb{R}^m \\
t &\mapsto r_1(t)
\end{split} \\
\begin{split}
r_2\colon [c, d] &\to \mathbb{R}^m \\
s &\mapsto r_2(s)
\end{split}
$$
hanno lo **stesso sostegno** $\gamma$, allora esiste una funzione
$$
\begin{split}
t\colon [c, d] &\to [a, b] \\
s &\mapsto t = t(s)
\end{split}
$$
per cui $r_1(t(s)) = r_2(s)$ e quindi:
$$
l(\gamma) = \int_a^b \|r_1'(t)\| dt = \int_c^d \|r_2'(s)\| ds
$$

## Ascissa curvilinea

Come per la lunghezza $l(\gamma)$ da $a$ a $b$, l'**ascissa curvilinea** (o parametro d'arco) $s\colon [a, b] \to [0, l(\gamma)]$ per cui
$$
s(t) = \int_a^t \|r'(x)\| dx
$$
rappresenta la **lunghezza** da $r(a)$, cioè l'**inizio della curva**, a $r(t)$, cioè un **punto della curva**.

Si può quindi **riparametrizzare** una curva $r\colon [a, b] \to \mathbb{R}^m$ per ottenere la **parametrizzazione naturale**:
$$
\begin{split}
\tilde{r}\colon [0, l(\gamma)] &\to \mathbb{R}^m \\
s &\mapsto \tilde{r}(s) = r(t(s))
\end{split}
$$
dove $t(s)$ è la **funzione inversa** di $s$, cioè $t\colon [0, l(\gamma)] \to [a, b]$ ricavabile con la derivata inversa:
$$
t'(s) = \frac{1}{s'(t)} = \frac{1}{\|r'(t)\|} \Rightarrow
t(s) = \int \frac{1}{\|r'(t)\|} ds
$$

Quindi, attraverso questa riparametrizzazione, è possibile **trovare ogni punto** appartenente a $\gamma$, con una _velocità scalare_ costante $\tilde{v}(s) = \|\tilde{r}'(s)\| = 1$, specificando come **parametro** la **lunghezza** variabile $s$.

Per esempio, se $r(t) = (2t, 5t - 6)$ con $t \in [0, 1]$ allora:
$$
s(t) = \int_0^t \|r'(x)\| dx = \sqrt{29}t
$$
e, dato che $\|r'(t)\|$ è costante (per cui $\tilde{r}(s)$ può essere trovato), $t(s) = \int \frac{1}{\sqrt{29}} ds = \frac{s}{\sqrt{29}}$.
Di conseguenza, la _parametrizzazione naturale_ sarà:
$$
\tilde{r}(s) = r(t(s)) = \left(\frac{2s}{\sqrt{29}}, \frac{5s}{\sqrt{29}} - 6\right)
$$
