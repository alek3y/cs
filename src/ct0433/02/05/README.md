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
