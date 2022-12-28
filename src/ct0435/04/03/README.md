# Matrice aggiunta e inversa

## Matrice aggiunta

La **matrice aggiunta** (_adjoint matrix_) di $A$ corrisponde alla **trasposta dei cofattori**:
$$\mathrm{adj}(A) = C^T$$

## Matrice inversa

La matrice inversa di una matrice $A$, è quella matrice per cui:
$$A \times A^{-1} = I$$

Se $A$ è **quadrata** e $\det(a) \neq 0$, allora l'inversa **esiste** e corrisponde a:
$$A^{-1} = \frac{1}{\det(A)} \cdot \mathrm{adj}(A)$$

### Esempio

Sia $A$ una matrice quadrata $3 \times 3$:
$$
A = \begin{bmatrix}
0 & 2 & 0 \\
6 & 0 & 8 \\
5 & 1 & 5
\end{bmatrix}
$$
allora per trovare l'inversa serve:

1. Il determinante:

$$
\det(A) = 0 - 2 \cdot \det\left(\begin{bmatrix}
6 & 8 \\
5 & 5
\end{bmatrix}\right) + 0 =
-2 \cdot (6 \cdot 5 - 8 \cdot 5) = 20
$$

2. La matrice dei cofattori:

$$
\begin{split}
C &= \begin{bmatrix}
M_{11} & -M_{12} & M_{13} \\
-M_{21} & M_{22} & -M_{23} \\
M_{31} & -M_{32} & M_{33}
\end{bmatrix} = \\
&= \begin{bmatrix}
0 \cdot 5 - 8 \cdot 1 & -(6 \cdot 5 - 8 \cdot 5) & 6 \cdot 1 - 0 \cdot 5 \\
-(2 \cdot 5 - 0 \cdot 1) & 0 \cdot 5 - 0 \cdot 5 & -(0 \cdot 1 - 2 \cdot 5) \\
2 \cdot 8 - 0 \cdot 0 & -(0 \cdot 8 - 0 \cdot 6) & 0 \cdot 0 - 2 \cdot 6
\end{bmatrix} =
\begin{bmatrix}
-8 & 10 & 6 \\
-10 & 0 & 10 \\
16 & 0 & -12
\end{bmatrix}
\end{split}
$$

3. La sua trasposta:

$$
C^T = \begin{bmatrix}
-8 & -10 & 16 \\
10 & 0 & 0 \\
6 & 10 & -12
\end{bmatrix}
$$

Per cui, infine:

$$
A^{-1} = \frac{1}{20} \cdot
\begin{bmatrix}
-8 & -10 & 16 \\
10 & 0 & 0 \\
6 & 10 & -12
\end{bmatrix} =
\frac{1}{10} \cdot
\begin{bmatrix}
-4 & -5 & 8 \\
5 & 0 & 0 \\
3 & 5 & -6
\end{bmatrix}
$$
