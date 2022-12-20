# Matrice a scala

Una matrice si dice **a gradini** quando ogni riga inizia con almeno uno **zero in più** rispetto a quella precedente.

Ciò significa che la matrice **a gradini** con meno zeri sarà la matrice **triangolare superiore**:
$$
\begin{bmatrix}
a & b & c \\
0 & d & e \\
0 & 0 & f
\end{bmatrix}
$$

Il primo elemento diverso da zero su ogni riga è detto **pivot**, per esempio la matrice:
$$
\begin{bmatrix}
3 & 9 & 0 & 1 \\
0 & 0 & 5 & 4 \\
0 & 0 & 0 & 2
\end{bmatrix}
$$
ha come _pivot_ 3, 5 e 2.

## Forma ridotta

Una matrice si dice **a gradini in forma ridotta** quando il **pivot** di ogni riga è $1$ e la sua colonna ha tutti $0$.

Per passare da una _matrice in scala_ in _forma ridotta_ basta:
1. Dividere ogni riga per il valore del pivot, portando i pivot a $1$
2. Portare tutti i termini sulle colonne dei pivot a $0$, sottraendo le righe con un multiplo della riga del pivot

Per esempio:
$$
\begin{bmatrix}
2 & 5 & 6 & 7 \\
0 & 3 & 1 & 1 \\
0 & 0 & 2 & 0
\end{bmatrix}
\xrightarrow{\text{dividi ogni riga per i pivot}}
\begin{bmatrix}
1 & \frac{5}{2} & 3 & \frac{7}{2} \\
0 & 1 & \frac{1}{3} & \frac{1}{3} \\
0 & 0 & 1 & 0
\end{bmatrix}
\xrightarrow{\text{sottrai la (3ᵃ} \times \frac{1}{3} \text{) dalla 2ᵃ}}
\begin{bmatrix}
1 & \frac{5}{2} & 3 & \frac{7}{2} \\
0 & 1 & 0 & \frac{1}{3} \\
0 & 0 & 1 & 0
\end{bmatrix}
\rightarrow
\\
\xrightarrow{\text{sottrai la (3ᵃ} \times 3 \text{) dalla 1ᵃ}}
\begin{bmatrix}
1 & \frac{5}{2} & 0 & \frac{7}{2} \\
0 & 1 & 0 & \frac{1}{3} \\
0 & 0 & 1 & 0
\end{bmatrix}
\xrightarrow{\text{sottrai la (2ᵃ} \times \frac{5}{2} \text{) dalla 1ᵃ}}
\begin{bmatrix}
1 & 0 & 0 & \frac{8}{3} \\
0 & 1 & 0 & \frac{1}{3} \\
0 & 0 & 1 & 0
\end{bmatrix}
$$
