# Rette e piani

## Retta

L'**equazione parametrica della retta** sarà:
$$X = P + tA$$
dove $P$ è un **punto**, mentre $A$ è un **vettore** che definisce la direzione della retta.

Il **parametro** $t$ assumerà tutti i valori reali, e fa da scalare al vettore $A$ traslandolo sul punto $P$, creando così una retta.

La **forma cartesiana della retta** invece, sarà:
$$ax + by + c = 0$$
per cui quando:
- $a = 0$, è parallelo all'asse $x$
- $b = 0$, è parallelo all'asse $y$
- $c = 0$, passa per l'origine

### In forma cartesiana

Dall'equazione $X$ e la sua **forma parametrica**
$$
r:
\begin{cases}
x = X_0 = p_0 + a_0 \cdot t \\
y = X_1 = p_1 + a_1 \cdot t
\end{cases}
$$
è possibile ricavare l'equazione in **forma cartesiana**, isolando $t$ e sostituendolo.

Per esempio, se $X = (1 - 2t, 2 + t)$:
$$
r:
\begin{cases}
x = 1 - 2t \\
y = 2 + t
\end{cases} =
\begin{cases}
t = \frac{1 - x}{2} \\
y = 2 + \frac{1 - x}{2} = \frac{5 - x}{2}
\end{cases}
\Rightarrow
x + 2y - 5 = 0
$$
che si ricava dalla trasformazione implicita di $y$ dopo la sostituzione.

### Dalla forma cartesiana

È per cui possibile anche fare il contrario, portando l'equazione dalla **forma cartesiana** alla **forma parametrica** ponendo $x$ uguale a $t$.

Per esempio, se:
$$
2x - y + 5 = 0
\Rightarrow
r:
\begin{cases}
x = t \\
y = 2t + 5
\end{cases}
\Rightarrow
X = (t, 2t + 5)
$$

Se poi si vuole trovare anche il punto $P$ e il vettore $A$, basterà:
$$
r:
\begin{cases}
p_0 + a_0 \cdot t = 0 + 1 \cdot t \\
p_1 + a_1 \cdot t = 5 + 2 \cdot t
\end{cases}
\Rightarrow
P = (0, 5) \land A = (1, 2)
$$

### Passante per due punti

Per usare l'**equazione parametrica** per trovare la retta passante per due punti $B$ e $C$ serve un vettore su cui si trova la retta.

Un vettore che possiamo costruirci è quindi $\overrightarrow{BC}$:
$$\overrightarrow{BC} = C - B = (c_0 - b_0, c_1 - b_1)$$

Come punto invece, possiamo usare sia $B$ che $C$:
$$X = C + t \cdot \overrightarrow{BC}$$

## Piano

L'**equazione parametrica del piano** su $\mathbb{R}^3$ sarà:
$$X = P + tA + sB$$
dove $P$ è un **punto** su cui traslare il piano, mentre $A$ e $B$ sono i due **vettori** che indicano la direzione del piano.

La **forma cartesiana** del piano sarà:
$$ax + by + cz + d = 0$$
per cui quando:
- $a = 0$, è parallelo all'asse $x$
- $b = 0$, è parallelo all'asse $y$
- $c = 0$, è parallelo all'asse $z$
- $d = 0$, passa per l'origine

## In forma cartesiana

Il procedimento è analogo alla trasformazione in forma cartesiana della retta.

Per esempio, se si vuole trovare il piano che passa tra i punti $P$, $Q$ e $R$, bisognerà creare due vettori $A = \overrightarrow{PQ}$ e $B = \overrightarrow{PR}$ da usare sull'_equazione parametrica_ assieme ad uno dei tre punti, per poi isolare i parametri e trovare l'equazione implicita del piano.

## Dalla forma cartesiana

Anche in questo caso il processo è analogo alla trasformazione dalla forma cartesiana della retta, per cui basterà porre $x = t$, $z = s$ e isolare $y$ nell'equazione cartesiana.
