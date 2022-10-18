# Rette e piani

## Retta

L'**equazione parametrica della retta** sarà:
$$R = P + tA$$
dove $P$ è un **punto**, mentre $A$ è un **vettore** che definisce la direzione della retta.

Il **parametro** $t$ assumerà tutti i valori reali, e fa da scalare al vettore $A$ traslandolo sul punto $P$, creando così una retta.

### In forma cartesiana

Dall'equazione $R$ e la sua **forma parametrica**
$$
r:
\begin{cases}
x = R_0 = p_0 + a_0 \cdot t \\
y = R_1 = p_1 + a_1 \cdot t
\end{cases}
$$
è possibile ricavare l'equazione in **forma cartesiana**, isolando $t$ e sostituendolo:
$$
r:
\begin{cases}
t = \frac{x - p_0}{a_0} \\
y = p_1 + a_1 \cdot (\frac{x - p_0}{a_0})
\end{cases}
\Rightarrow
ax + by + c = 0
$$
con $a = \frac{a_1}{a_0}$, $b = -1$ e $c = -\frac{a_1}{a_0}p_0 + p_1$, ricavati dalla trasformazione in forma implicita di $y$.

### Dalla forma cartesiana

È per cui possibile anche fare il contrario, portando l'equazione dalla **forma cartesiana** alla **forma parametrica** ponendo $x$ uguale a $t$:
$$
ax + by + c = 0 \Rightarrow
r:
\begin{cases}
x = t \\
y = - \frac{at + c}{b}
\end{cases}
$$
da cui si ricava che
$$R = \left(t, -\frac{at + c}{b}\right) \Leftrightarrow (p_0 + a_0 \cdot t, p_1 + a_1 \cdot t) = \left(t, -\frac{at + c}{b}\right)$$
e quindi
$$
r:
\begin{cases}
p_0 + a_0 \cdot t = 0 + 1 \cdot t \\
p_1 + a_1 \cdot t =  -\frac{c}{b} - \frac{a}{b} \cdot t
\end{cases}
\Rightarrow
P = \left(0, -\frac{c}{b}\right) \land A = \left(1, -\frac{a}{b}\right)
$$

### Passante per due punti

Per usare l'**equazione parametrica** per trovare la retta passante per due punti $B$ e $C$ serve un vettore su cui si trova la retta.

Un vettore che possiamo costruirci è quindi $\overrightarrow{BC}$:
$$\overrightarrow{BC} = C - B = (c_0 - b_0, c_1 - b_1)$$

Come punto invece, possiamo usare sia $B$ che $C$:
$$R = C + t \cdot \overrightarrow{BC}$$
