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

- $a_{ij}$ è chiamato **termine** (o componente), ed è l'elemento sulla riga $i$ e sulla colonna $j$
- $(a_{ij})$ definisce una matrice in cui ogni termine è indentificato da $a_{ij}$
- $A^j$ è il vettore che contiene gli elementi sulla colonna $j$
- $A_i$ è il vettore sulla riga $i$
- Se la matrice è **quadrata**, quindi se ha dimensioni $n \times n$, può essere:
	- _Simmetrica_ (sulla diagonale), se $a_{ij} = a_{ji}, \forall i, j \in [1, n]$
	- _Diagonale_, se $a_{ij} = 0, \forall i, j \in [1, n] : i \neq j$, per cui è anche _simmetrica_
	- _Identica_ (o _identità_ o _unita_), se è _diagonale_ e $a_{ii} = 1, \forall i \in [1, n]$
	- _Triangolare_ (inferiore, perchè hanno elementi solo in basso), se $a_{ij} = 0, \forall i, j \in [1, n] : i < j$
- La matrice **zero** (chiamata $O$), è una matrice in cui ogni elemento è nullo

## Operazioni

- **Addizione**:

	È possibile fare l'addizione solamente se le due matrici hanno la **stessa dimensione** $m \times n$:
	$$A = (a_{ij}) \land B = (b_{ij}) \Rightarrow A + B = (a_{ij} + b_{ij})$$

- **Prodotto per uno scalare** $c \in \mathbb{R}$:

	$$c \cdot A = (c \cdot a_{ij})$$

- **Prodotto fra matrici**:

	Il prodotto esiste solo se la prima ha **numero di colonne uguale al numero di righe** della seconda, per esempio:
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

	Del prodotto è importante notare che con le matrici **non vale la proprietà commutativa**.

	Inoltre, la matrice identità è l'elemento neutro del prodotto:
	$$A \times I = A$$

- **Traccia**

	La traccia di una matrice **quadrata** equivale alla somma di tutti gli elementi sulla diagonale:
	$$\mathrm{tr}(A) = \sum_{i=1}^n a_{ii}$$

- **Trasposta**

	La matrice trasposta consiste nello scambiare la colonna per la riga su ogni elemento della matrice, per cui **viene capovolta sulla diagonale**.

	Per esempio,
$$
\begin{bmatrix}
2 & 3 & 0 \\
1 & 3 & 5 \\
4 & 2 & 0
\end{bmatrix}^T =
\begin{bmatrix}
2 & 1 & 4 \\
3 & 3 & 2 \\
0 & 5 & 0
\end{bmatrix}
$$
