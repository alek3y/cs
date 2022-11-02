# Sistemi lineari

Un **sistema lineare** è un insieme di equazioni, cioè degli **iperpiani** (piani di $n$ dimensioni) di cui si vuole trovare l'intersezione.

Nel caso di $\mathbb{R}^2$, ci sono due variabili $x$ e $y$ e quindi l'_iperpiano_ è una retta:
- Se sono **parallele** (cioè quando $\cos\theta = \pm 1$, dove $\theta$ è l'angolo tra le rette), il sistema **non ha soluzioni**
- Se sono **coincidenti** (cioè quando $\cos\theta = 0$) ha **infinite soluzioni**
- Se sono **non parallele** (cioè quando $\cos\theta \not\in \{\pm 1, 0\}$) ha **una soluzione** che sarà un punto

## Metodo di eliminazione di Gauss

Il **metodo di eliminazione Gauss** permette di semplificare il processo di risolvimento di sistemi lineari.

I passaggi possono consistere nel:
- **Scambiare le righe**:
$$
\begin{cases}
a_1x + b_1y = c_1 \\
a_2x + b_2y = c_2
\end{cases} \Rightarrow
\begin{cases}
a_2x + b_2y = c_2 \\
a_1x + b_1y = c_1
\end{cases}
$$
- **Moltplicare per uno scalare**:
	$$ax + by = c \Rightarrow dax + dby = dc$$
- **Sommare**:
$$
\begin{matrix}
a_1x + b_1y = c_1 \\
a_2x + b_2y = c_2
\end{matrix} \Rightarrow
(a_1 + a_2)x + (b_1 + b_2)y = c_1 + c_2
$$

Questi passaggi si possono effettuare anche con le **matrici** considerando solo i **coefficienti**, dato che:
$$
\begin{cases}
a_1x + b_1y = c_1 \\
a_2x + b_2y = c_2
\end{cases} \Rightarrow
\begin{bmatrix}
a_1 & b_1 \\ a_2 & b_2
\end{bmatrix} \cdot
\begin{bmatrix}
x \\ y
\end{bmatrix} =
\begin{bmatrix}
c_1 \\ c_2
\end{bmatrix}
$$

Per esempio, partendo con:
$$
\begin{cases}
3x + 2y = 5 \\
x + 4y = 3
\end{cases} \Rightarrow
\begin{bmatrix}
3 & 2 & 5 \\
1 & 4 & 3
\end{bmatrix}
$$
per cui si trasforma il sistema in una **matrice dei coefficienti** a cui si applicheranno i passaggi:
1. Scambiare le due righe:
$$
\begin{bmatrix}
3 & 2 & 5 \\
1 & 4 & 3
\end{bmatrix} \Rightarrow
\begin{bmatrix}
1 & 4 & 3 \\
3 & 2 & 5
\end{bmatrix}
$$
2. Moltiplicare la prima riga per $3$
$$
\begin{bmatrix}
1 & 4 & 3 \\
3 & 2 & 5
\end{bmatrix} \Rightarrow
\begin{bmatrix}
3 & 12 & 9 \\
3 & 2 & 5
\end{bmatrix}
$$
3. Sottrarre la seconda alla prima:
$$
\begin{bmatrix}
3 & 12 & 9 \\
3 & 2 & 5
\end{bmatrix} \Rightarrow
\begin{bmatrix}
3 & 12 & 9 \\
0 & -10 & -4
\end{bmatrix}
$$
4. Semplificare la prima per $3$ e la seconda per $2$
$$
\begin{bmatrix}
3 & 12 & 9 \\
0 & -10 & -4
\end{bmatrix} \Rightarrow
\begin{bmatrix}
1 & 4 & 3 \\
0 & -5 & -2
\end{bmatrix}
$$
5. Ricostruire il sistema
$$
\begin{bmatrix}
1 & 4 & 3 \\
0 & -5 & -2
\end{bmatrix} \Rightarrow
\begin{cases}
x + 4y = 3 \\
-5y = -2
\end{cases} \Rightarrow
\begin{cases}
x = \frac{7}{5} \\
y = \frac{2}{5}
\end{cases}
$$
