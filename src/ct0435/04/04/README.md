# Matrice elementare

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
E \times
\begin{bmatrix}
3 & 4 \\ 3 & -2 \\ 4 & -2
\end{bmatrix} =
\begin{bmatrix}
3 & -2 \\ 3 & 4 \\ 4 & -2
\end{bmatrix}
$$

Questo tipo di matrice serve a rappresentare le [mosse di Gauss](../../05/README.md#metodo-di-eliminazione-di-gauss), cioè le _operazioni elementari per riga_:
- Scambio due righe
- Moltiplico una riga per $c \in \mathbb{R}$
- Sommo due righe e metto il risultato su una delle due
