# Matrici

Una matrice,
$$
A = \begin{bmatrix}
a_{11} & a_{12} & \cdots & a_{1n} \\
a_{21} & a_{22} & \cdots & a_{2n} \\
\vdots & \vdots & \ddots & \vdots \\
a_{m1} & a_{m2} & \cdots & a_{mn} \\
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

## Operazioni semplici

- **Addizione**:

	È possibile fare l'addizione solamente se le due matrici hanno la **stessa dimensione** $m \times n$:
	$$A = (a_{ij}) \land B = (b_{ij}) \Rightarrow A + B = (a_{ij} + b_{ij})$$

- **Prodotto per uno scalare** $c \in \mathbb{R}$:

	$$c \cdot A = (c \cdot a_{ij})$$

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

- **Traccia**

	La traccia di una matrice **quadrata** equivale alla somma di tutti gli elementi sulla diagonale:
	$$\mathrm{tr}(A) = \sum_{i=1}^n a_{ii}$$

- **Prodotto hadamard** (o _element-wise product_):

	Consiste nel prodotto fra gli elementi di due matrici di uguali dimensioni:
$$
A \circ B = A \odot B =
\begin{bmatrix}
a_{11} & a_{12} \\
a_{21} & a_{22}
\end{bmatrix}
\circ
\begin{bmatrix}
b_{11} & b_{12} \\
b_{21} & b_{22}
\end{bmatrix} =
\begin{bmatrix}
a_{11}b_{11} & a_{12}b_{12} \\
a_{21}b_{21} & a_{22}b_{22} \\
\end{bmatrix}
$$
