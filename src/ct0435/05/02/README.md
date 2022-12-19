# Determinante, minore e cofattori

Il **determinante** di una matrice **quadrata** $2 \times 2$ è dato da:
$$
\det(A) = \det\left(
\begin{bmatrix}
a & b \\
c & d
\end{bmatrix}
\right) = a \cdot d - b \cdot c
$$

Nel caso di matrici più grandi però, come nel caso di $3 \times 3$, si ha che:
$$
\begin{split}
\det\left(
\begin{bmatrix}
a & b & c \\
d & e & f \\
g & h & i
\end{bmatrix}
\right) &=
a \cdot M_{1,1} - b \cdot M_{1,2} + c \cdot M_{1,3} = \\
&= a \cdot \det\left(
\begin{bmatrix}
e & f \\
h & i
\end{bmatrix}
\right) -
b \cdot \det\left(
\begin{bmatrix}
d & f \\
g & i
\end{bmatrix}
\right) +
c \cdot \det\left(
\begin{bmatrix}
d & e \\
g & h
\end{bmatrix}
\right)
\end{split}
$$
dove $M_{1,j}$ è il **minore** sulla colonna $j$.

**In generale** quindi, quando la dimensione è più grande di $2 \times 2$:
$$\det(A) = \sum_{j = 1}^{n} a_{1,j} \cdot C_{1,j}$$
dove $C_{1,j}$ è il valore della **matrice dei cofattori** in riga $1$ e colonna $j$, mentre $n$ è il numero di colonne.

## Minore

Il **minore** $M_{i,j}$ di una matrice $A$ è il **determinante** della sottomatrice **senza** la riga $i$ e la colonna $j$.

Per esempio:
$$
A = \begin{bmatrix}
1 & 2 & 3 \\
4 & 5 & 6 \\
7 & 8 & 9
\end{bmatrix}
\Rightarrow
M_{1, 3} = \det\left(
\begin{bmatrix}
4 & 5 \\
7 & 8
\end{bmatrix}\right) = 4 \cdot 8 - 5 \cdot 7 = -3
$$

## Matrice dei cofattori

Ogni **cofattore** della matrice dei cofattori è dato da:
$$
C_{i,j} = (-1)^{i+j} \cdot M_{i,j}
$$
dove $M_{i,j}$ è il _minore_ di $A$ in riga $i$ e colonna $j$.

Di conseguenza, la matrice completa dei cofattori di $A$ sarà:
$$
C = \begin{bmatrix}
C_{11} & C_{12} & \cdots & C_{1n} \\
C_{21} & C_{22} & \cdots & C_{2n} \\
\vdots & \vdots & \ddots & \vdots \\
C_{n1} & C_{n2} & \cdots & C_{nn} \\
\end{bmatrix}
$$
