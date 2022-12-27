# Autovalori e autovettori

Un **autovettore** del'**applicazione lineare** $T\colon V \to V$ (_endomorfismo_), è quel vettore **non nullo** $v$ per cui:
$$T(v) = \lambda v$$
cioè che **non cambia direzione** su $V$ con la trasformazione $T$, ma viene scalato da $\lambda \in K$.

In questo caso, $\lambda$ è detto l'**autovalore** associato a $v$ della trasformazione $T$. \
Esso sarà anche l'_autovalore_ di tutti i multipli $u = kv$ con $k \in K^\ast$, infatti:
$$T(u) = T(kv) = kT(v) = \lambda kv = \lambda u$$

Se la trasformazione è espressa attraverso una **matrice** $A$ **associata alla trasformazione** $T$, allora quando:
$$Av = \lambda v$$
il vettore $v$ sarà un _autovettore_, mentre $\lambda$ sarà il suo _autovalore_.

## Polinomio caratteristico

Della matrice quadrata $A$ di dimensioni $n \times n$, si dice **polinomio caratteristico** il determinante:
$$p(\lambda) = \det(A - \lambda I)$$
dove $I$ è la matrice identità di dimensioni uguali ad $A$.

## Ricavare autovalori e autovettori

Sapendo che
$$Av = \lambda \vec{v} \Leftrightarrow A\vec{v} - \lambda \vec{v} = \vec{0} \Leftrightarrow (A - \lambda I) \vec{v} = \vec{0}$$
e conoscendo la matrice $A$ e un _autovalore_ $\lambda$, è possibile ricavare il corrispondente _autovettore_ $v$.

Mentre per trovare $\lambda$, basta trovare gli zeri del _polinomio caratteristico_:
$$\det(A - \lambda I) = 0$$
da cui si ricaveranno al massimo $n$ _autovalori_, dove $n$ è la dimensione di $A$ (i.e. $n \times n$).

### Esempio

Sia $T$ una trasformazione lineare rappresentata dalla matrice $A = \begin{bmatrix}2 & 1 \\ 3 & 4\end{bmatrix}$, allora:
$$
\det\left(
A - \begin{bmatrix}
\lambda & 0 \\
0 & \lambda
\end{bmatrix}
\right) = 0 \\
\Updownarrow \\
\det\left(
\begin{bmatrix}
2-\lambda & 1 \\
3 & 4-\lambda
\end{bmatrix}
\right) = 0 \\
\Updownarrow \\
(2 - \lambda)(4 - \lambda) - 1 \cdot 3 = 0 \\
\Updownarrow \\
\lambda^2 - 6\lambda + 5 = 0 \Rightarrow \lambda_{1, 2} = \frac{6 \pm \sqrt{36 - 4 \cdot 5}}{2} = 3 \pm 2
$$
per cui gli _autovettori_ $v_1$ e $v_2$ si divideranno in base all'_autovalore_ corrispondente:
- $\lambda_1 = 5$:
$$
(A - \lambda I) \begin{bmatrix}x \\ y\end{bmatrix} = \begin{bmatrix}0 \\ 0\end{bmatrix} \\
\Updownarrow \\
\left(\begin{bmatrix}
2 & 1 \\
3 & 4
\end{bmatrix} -
\begin{bmatrix}
5 & 0 \\
0 & 5
\end{bmatrix}\right)
\begin{bmatrix}x \\ y\end{bmatrix} = \begin{bmatrix}0 \\ 0\end{bmatrix} \\
\Updownarrow \\
\begin{bmatrix}
-3 & 1 \\
3 & -1
\end{bmatrix}
\begin{bmatrix}x \\ y\end{bmatrix} = \begin{bmatrix}0 \\ 0\end{bmatrix} \\
\Updownarrow \\
\begin{bmatrix}
-3x + y \\
3x - y
\end{bmatrix} = \begin{bmatrix}0 \\ 0\end{bmatrix} \\
\Downarrow \\
\begin{cases}
-3x + y = 0 \\
3x - y = 0
\end{cases} \Rightarrow y = 3x
\Rightarrow
v_1 = \begin{bmatrix}t \\ 3t\end{bmatrix}
$$

- $\lambda_2 = 1$:
$$
\left(\begin{bmatrix}
2 & 1 \\
3 & 4
\end{bmatrix} -
\begin{bmatrix}
1 & 0 \\
0 & 1
\end{bmatrix}\right)
\begin{bmatrix}x \\ y\end{bmatrix} = \begin{bmatrix}0 \\ 0\end{bmatrix} \\
\Updownarrow \\
\begin{bmatrix}
x + y \\
3x + 3y
\end{bmatrix} = \begin{bmatrix}0 \\ 0\end{bmatrix} \\
\Downarrow \\
y = -x \Rightarrow v_2 = \begin{bmatrix}t \\ -t\end{bmatrix}
$$
