# Prodotto tra matrici
Il prodotto esiste **se e solo se** $A^T$ ha lo **stesso numero di righe** di $B$, per esempio:
$$
A \times B =
\begin{bmatrix}
0 & 3 & 5 \\ 5 & 5 & 2
\end{bmatrix}
\begin{bmatrix}
3 & 4 \\ 3 & -2 \\ 4 & -2
\end{bmatrix} =
\begin{bmatrix}
A_1 \cdot B^1 &
A_1 \cdot B^2 \\
A_2 \cdot B^1 &
A_2 \cdot B^2 \\
\end{bmatrix} = \\
= \begin{bmatrix}
0 + 9 + 20 & 0 - 6 - 10 \\
15 + 15 + 8 & 20 - 10 - 4
\end{bmatrix} =
\begin{bmatrix}
29 & -16 \\
38 & 6
\end{bmatrix}
$$

Infatti, l'unico modo di moltiplicare due matrici $1 \times n$,
$$
A \times B =
\begin{bmatrix}
a_1 \\ \vdots \\ a_n
\end{bmatrix}
\begin{bmatrix}
b_1 \\ \vdots \\ b_n
\end{bmatrix}
$$
è quello di trasporne una delle due, quindi $A^T \times B$ è possibile.

## Proprietà

Ci sono due proprietà importanti che riguardano il prodotto:
- Non vale la **proprietà commutativa**
- La matrice identità è l'**elemento neutro** del prodotto, infatti $A \times I = A$
