# Matrici

Una matrice,
$$
A = \begin{bmatrix}
a_{11} & a_{12} & ... & a_{1j} \\
a_{21} & a_{22} & ... & a_{2j} \\
\vdots & \vdots & \ddots & \vdots \\
a_{i1} & a_{i2} & ... & a_{ij} \\
\end{bmatrix}
$$
ha dimensioni $m \times n$, con $m$ **righe** e $n$ **colonne**.

## Proprietà

- $a_{ij}$ è l'elemento sulla riga $i$ e sulla colonna $j$
- $A_i$ è il vettore che contiene gli elementi sulla riga $i$
- $A_j$ è il vettore sulla colonna $j$
- Se la matrice è quadrata, quindi se ha dimensioni $n \times n$, può essere:
	- _Simmetrica_ (sulla diagonale), se $a_{ij} = a_{ji}, \forall i, j \in [1, n]$
	- _Diagonale_, se $a_{ij} = 0, \forall i, j \in [1, n] : i \neq j$
	- _Identica_, se è _diagonale_ e $a_{ii} = 1, \forall i \in [1, n]$
	- _Triangolare_ (inferiore, perchè hanno elementi solo in basso), se $a_{ij} = 0, \forall i, j \in [1, n] : i < j$
