# Sistemi lineari

Un **sistema lineare** è un insieme di equazioni in forma di **iperpiani** (piani di $n$ dimensioni) di cui si vuole trovare l'intersezione, e viene espresso come:
$$Ax = b$$
dove $A$ è la matrice dei **coefficienti**, $x$ il vettore delle **incognite** e $b$ il vettore dei **termini noti**, per esempio:
$$
\begin{cases}
ax + by = e \\
cx + dy = f
\end{cases} \Rightarrow
\begin{bmatrix}
a & b \\
c & d
\end{bmatrix}
\begin{bmatrix}x \\ y\end{bmatrix} =
\begin{bmatrix}e \\ f\end{bmatrix}
$$

Accostando $A$ a $b$ otteniamo la **matrice completa di sistema**:
$$
A|b =
\left[\begin{array}{cc|c}
a & b & e \\
c & d & f
\end{array}\right]
$$

<!-- TODO: È importante?
Nel caso di $\mathbb{R}^2$, ci sono due variabili $x$ e $y$ e quindi l'_iperpiano_ è una retta:
- Se sono **parallele** (cioè quando $\cos\theta = \pm 1$, dove $\theta$ è l'angolo tra le rette), il sistema **non ha soluzioni**
- Se sono **coincidenti** (cioè quando $\cos\theta = 0$) ha **infinite soluzioni**
- Se sono **non parallele** (cioè quando $\cos\theta \not\in \{\pm 1, 0\}$) ha **una soluzione** che sarà un punto
-->

## Teorema di Rouché-Capelli

Con questo teorema è possibile stabilire se un sistema lineare ammette soluzioni e, nel caso, quantificarle tramite il rango della matrice associata al sistema.

Infatti, dato un sistema lineare $Ax = b$, si ha che:
- Se $\mathrm{rank}(A) < \mathrm{rank}(A|b)$, allora **non esistono soluzioni**
- Se $\mathrm{rank}(A) = \mathrm{rank}(A|b)$, allora **ammette soluzioni** con $n$ _numero di incognite_:
	- Se $\mathrm{rank}(A) = \mathrm{rank}(A|b) = n$, allora esiste **una sola soluzione**
	- Se $\mathrm{rank}(A) = \mathrm{rank}(A|b) < n$, allora ha **infinite soluzioni**

## Metodo della matrice inversa

Sapendo che $Ax = b$, allora se $A$ è **invertibile** (cioè $A$ è quadrata e $\det(A) \neq 0$) **allora esistono soluzioni**:
$$x = A^{-1}b$$

### Esempio

Partendo dal sistema
$$
\begin{cases}
4x + 2y = 5 \\
3x + y = -1
\end{cases} \Rightarrow
A = \begin{bmatrix}
4 & 2 \\
3 & 1
\end{bmatrix}
$$
esistono soluzioni perchè $\dim(A) = 2 \times 2$ e $\det(A) = -2 \neq 0$.

La matrice dei cofattori e l'aggiunta di $A$ saranno:
$$
C = \begin{bmatrix}
1 & -3 \\
-2 & 4
\end{bmatrix} \Rightarrow
C^T = \begin{bmatrix}
1 & -2 \\
-3 & 4
\end{bmatrix}
$$
per cui:
$$
A^{-1} = -\frac{1}{2} C^T =
\begin{bmatrix}
-\frac{1}{2} & 1 \\
\frac{3}{2} & -2
\end{bmatrix} \Rightarrow
x = \begin{bmatrix}
-\frac{1}{2} & 1 \\
\frac{3}{2} & -2
\end{bmatrix}
\begin{bmatrix}5 \\ -1\end{bmatrix} =
\begin{bmatrix}
-\frac{7}{2} \\
\frac{19}{2}
\end{bmatrix}
$$

## Metodo di eliminazione di Gauss

Tramite i passaggi del **metodo di Gauss** è possibile semplificare il risolvimento di sistemi lineari:
- **Scambiare le righe**:
$$
\left[\begin{array}{cc|c}
a & b & e \\
c & d & f
\end{array}\right] \xrightarrow{R_1 \leftrightarrow R_2}
\left[\begin{array}{cc|c}
c & d & f \\
a & b & e
\end{array}\right]
$$

- **Moltiplicare per uno scalare**:
$$
\left[\begin{array}{cc|c}
a & b & e \\
c & d & f
\end{array}\right] \xrightarrow{3R_2 \to R_2}
\left[\begin{array}{cc|c}
a & b & e \\
3c & 3d & 3f
\end{array}\right]
$$

- **Sommare una riga al multiplo di un'altra**:
$$
\left[\begin{array}{cc|c}
a & b & e \\
c & d & f
\end{array}\right] \xrightarrow{R_1 - 2R_2 \to R_1}
\left[\begin{array}{cc|c}
a-2c & b-2d & e-2f \\
c & d & f
\end{array}\right]
$$

### Esempio

Partendo con il sistema lineare
$$
\begin{cases}
3x + 2y = 5 \\
x + 4y = 3
\end{cases} \Rightarrow
A|b =
\left[\begin{array}{cc|c}
3 & 2 & 5 \\
1 & 4 & 3
\end{array}\right]
$$
si può trasformare in _matrice dei coefficienti_ a cui si applicheranno i passaggi:
1. Scambiare le due righe:
$$
\left[\begin{array}{cc|c}
3 & 2 & 5 \\
1 & 4 & 3
\end{array}\right] \xrightarrow{R_1 \leftrightarrow R_2}
\left[\begin{array}{cc|c}
1 & 4 & 3 \\
3 & 2 & 5
\end{array}\right]
$$
2. Moltiplicare la prima riga per $3$;
$$
\left[\begin{array}{cc|c}
1 & 4 & 3 \\
3 & 2 & 5
\end{array}\right] \xrightarrow{3R_1 \to R_1}
\left[\begin{array}{cc|c}
3 & 12 & 9 \\
3 & 2 & 5
\end{array}\right]
$$
3. Sottrarre la prima alla seconda:
$$
\left[\begin{array}{cc|c}
3 & 12 & 9 \\
3 & 2 & 5
\end{array}\right] \xrightarrow{R_2 - R_1 \to R_2}
\left[\begin{array}{cc|c}
3 & 12 & 9 \\
0 & -10 & -4
\end{array}\right]
$$
4. Semplificare la prima per $3$ e la seconda per $-2$:
$$
\left[\begin{array}{cc|c}
3 & 12 & 9 \\
0 & -10 & -4
\end{array}\right] \xrightarrow{\frac{1}{3}R_1 \to R_1,\; -\frac{1}{2}R_2 \to R_2}
\left[\begin{array}{cc|c}
1 & 4 & 3 \\
0 & 5 & 2
\end{array}\right]
$$
5. Ricostruire il sistema:
$$
\left[\begin{array}{cc|c}
1 & 4 & 3 \\
0 & 5 & 2
\end{array}\right] \Rightarrow
\begin{cases}
x + 4y = 3 \\
5y = 2
\end{cases} \Rightarrow
\begin{cases}
x = \frac{7}{5} \\
y = \frac{2}{5}
\end{cases}
$$

## Metodo di Cramer

L'utilizzo di questo metodo su **sistemi lineari quadrati** (di tante equazioni quante incognite) permette di trovare soluzioni quando $\det(A) \neq 0$.

Lo scopo del metodo è trovare tante matrici quante sono le incognite, per esempio $D_x$, $D_y$ e $D_z$ nel caso di:
$$
\begin{cases}
x + 2y + 3z = 3 \\
-x + 3y + 2z = 1 \\
-2x + y = -2
\end{cases} \Rightarrow
A = \begin{bmatrix}
1 & 2 & 3 \\
-1 & 3 & 2 \\
-2 & 1 & 0
\end{bmatrix},\;
b = \begin{bmatrix}3 \\ 1 \\ -2\end{bmatrix}
$$
_esistono soluzioni_ perchè $\det(A) = 5 \neq 0$, allora rimpiazzando $b$ al posto della colonna di $x$, $y$ e $z$:
$$
\begin{alignat*}{2}
D_x &= \det\left(\begin{bmatrix}
3 & 2 & 3 \\
1 & 3 & 2 \\
-2 & 1 & 0
\end{bmatrix}\right) = 7 &&\enspace\Rightarrow\enspace x = \frac{D_x}{\det(A)} = \frac{7}{5} \\
D_y &= \det\left(\begin{bmatrix}
1 & 3 & 3 \\
-1 & 1 & 2 \\
-2 & -2 & 0
\end{bmatrix}\right) = 4 &&\enspace\Rightarrow\enspace y = \frac{D_y}{\det(A)} = \frac{4}{5} \\
D_z &= \det\left(\begin{bmatrix}
1 & 2 & 3 \\
-1 & 3 & 1 \\
-2 & 1 & -2
\end{bmatrix}\right) = 0 &&\enspace\Rightarrow\enspace z = \frac{D_z}{\det(A)} = 0
\end{alignat*}
$$
