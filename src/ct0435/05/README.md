# Matrici

Una matrice,
$$
A = \begin{bmatrix}
a_{11} & a_{12} & \cdots & a_{1j} \\
a_{21} & a_{22} & \cdots & a_{2j} \\
\vdots & \vdots & \ddots & \vdots \\
a_{i1} & a_{i2} & \cdots & a_{ij} \\
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

- **Prodotto fra matrici**:

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
A \cdot B =
\begin{bmatrix}
a_1 \\ \vdots \\ a_n
\end{bmatrix}
\begin{bmatrix}
b_1 \\ \vdots \\ b_n
\end{bmatrix}
$$
	è quello di trasporne una delle due, quindi $A^T \cdot B$ è possibile.

	Del prodotto è importante notare che con le matrici **non vale la proprietà commutativa**.

	Inoltre, la matrice identità è l'elemento neutro del prodotto:
	$$A \times I = A$$

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

- **Traccia**

	La traccia di una matrice **quadrata** equivale alla somma di tutti gli elementi sulla diagonale:
	$$\mathrm{tr}(A) = \sum_{i=1}^n a_{ii}$$

## Matrice elementare

Una **matrice elementare** è un tipo speciale di matrice che si ottiene effettuando una singola **operazione elementare per riga** (_elementary row operation_) sulla **matrice identità**.

La matrice ottenuta si può considerare come un _blueprint_ che propagherà la stessa _operazione elementare_ come trasformazione di una matrice generica attraverso la moltiplicazione.

Per esempio, partendo da una matrice identità $3 \times 3$ e scambiando le prime due righe si ottiene:
$$
E =
\begin{bmatrix}
0 & 1 & 0 \\
1 & 0 & 0 \\
0 & 0 & 1
\end{bmatrix}
$$
che moltiplicato per una qualsiasi matrice, si ha:
$$
E \cdot
\begin{bmatrix}
3 & 4 \\ 3 & -2 \\ 4 & -2
\end{bmatrix} =
\begin{bmatrix}
3 & -2 \\ 3 & 4 \\ 4 & -2
\end{bmatrix}
$$

Questo tipo di matrice serve a rappresentare le [mosse di Gauss](../04/README.md#metodo-di-eliminazione-di-gauss), cioè le _operazioni elementari per riga_:
- Scambio due righe
- Moltiplico una riga per $c \in \mathbb{R}$
- Sommo due righe e metto il risultato su una delle due
