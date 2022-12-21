# Rango

Il **rango** o **caratteristica** di una matrice indica il **massimo numero di righe** [linearmente indipendenti](#TODO).
Questo vale anche per le colonne, infatti:
$$\mathrm{rank}(A) = \mathrm{rank}(A^T)$$

Un metodo per calcolare il rango è quello di trasformare una matrice in **scala a forma ridotta** tramite le _operazioni elementari per riga_, per esempio:
$$
\begin{alignat*}{2}
A = \begin{bmatrix}
2 & 4 & 1 & 3 \\
-1 & -2 & 1 & 0 \\
0 & 0 & 2 & 2 \\
3 & 6 & 2 & 5
\end{bmatrix}
\xrightarrow{R_4 - R_1 \to R_4}
&\begin{bmatrix}
2 & 4 & 1 & 3 \\
-1 & -2 & 1 & 0 \\
0 & 0 & 2 & 2 \\
1 & 2 & 1 & 2
\end{bmatrix}
&
\xrightarrow{R_4 + R_2 \to R_4}
&\begin{bmatrix}
2 & 4 & 1 & 3 \\
-1 & -2 & 1 & 0 \\
0 & 0 & 2 & 2 \\
0 & 0 & 2 & 2
\end{bmatrix}
\\
\xrightarrow{R_4 - R_3 \to R_4}
&\begin{bmatrix}
2 & 4 & 1 & 3 \\
-1 & -2 & 1 & 0 \\
0 & 0 & 2 & 2 \\
0 & 0 & 0 & 0
\end{bmatrix}
&
\xrightarrow{R_2 + \frac{1}{2}R_1 \to R_2}
&\begin{bmatrix}
2 & 4 & 1 & 3 \\
0 & 0 & \frac{3}{2} & \frac{3}{2} \\
0 & 0 & 2 & 2 \\
0 & 0 & 0 & 0
\end{bmatrix}
\\
&&
\xrightarrow{R_3 - \frac{4}{3}R_2 \to R_3}
&\begin{bmatrix}
2 & 4 & 1 & 3 \\
0 & 0 & \frac{3}{2} & \frac{3}{2} \\
0 & 0 & 0 & 0 \\
0 & 0 & 0 & 0
\end{bmatrix}
\end{alignat*}
$$
per cui la matrice avrà rango $\mathrm{rank}(A) = 2$ perchè ci sono solo due righe **non nulle**.
