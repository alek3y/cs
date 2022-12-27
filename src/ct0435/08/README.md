# Trasformazioni lineari

Una **trasformazione** (o _applicazione_) $f\colon V \to W$ con $V$ e $W$ spazi vettoriali sul campo $K$ è **lineare** se:
- $f(x + y) = f(x) + f(y)$,
- $f(ax) = af(x)$

per ogni $a \in K$ e $x, y \in V$.

La _trasformazione_ $f$ è detta:
- **Omomorfismo** (sempre), perchè con $\omega\colon V \cup W \to V \cup W$ (e.g. $\omega(x, y) = x + y$ o $\omega(x) = 2x$):
	$$f(\omega(x, y, ...)) = \omega(f(x), f(y), ...)$$

- **Isomorfismo**, quando $f$ è _biettiva_ e quindi esiste $f^{-1}\colon W \to V$.

- **Endomorfismo**, quando $\mathrm{Dom}(f) = \mathrm{Codom}(f)$, e quindi $f\colon V \to V$.

- **Automorfismo**, quando oltre ad essere un _endomorfismo_ esiste anche l'_inversa_.

## Nucleo e immagine

Il **nucleo** della funzione $f$ (_sottospazio_ di $V$) è definito come come l'insieme di punti la cui immagine è nulla:
$$\mathrm{Ker}(f) = \{\vec{x} \in V : f(\vec{x}) = \vec{0}\}$$
mentre l'[_immagine_](../../ct0432/03/01/README.md#immagine) (_sottospazio_ di $W$) corrisponde a $\mathrm{Im}(f) = \{f(x) \in W : x \in V\}$.

Se $\dim(V), \dim(W) \in \mathbb{N}$, il **teorema della dimensione** dice che:
$$\dim(\mathrm{Ker}(f)) + \dim(\mathrm{Im}(f)) = \dim(V)$$

## Biettività

Perchè $f$ sia _invertibile_ e quindi **biettiva**, la funzione deve essere:
- **Iniettiva**, quando $\dim(\mathrm{Ker}(f)) = 0$
- **Suriettiva**, quando $\dim(\mathrm{Im}(f)) = \dim(W)$

## Matrice associata alla trasformazione

Una trasformazione lineare del tipo,
$$
\begin{split}
t\colon &\mathbb{R}^2 \to \mathbb{R}^3 \\
&(x, y) \mapsto (x + y, 2x, x - y)
\end{split}
$$
è esprimibile dalla matrice dei **coefficienti**:
$$
\begin{bmatrix}
a & b \\
c & d \\
e & f
\end{bmatrix}
\begin{bmatrix}
x \\
y
\end{bmatrix} =
\begin{bmatrix}
x + y \\
2x \\
x - y
\end{bmatrix} =
\begin{bmatrix}
ax + by \\
cx + dy \\
ex + fy
\end{bmatrix}
\Leftrightarrow
\begin{bmatrix}
a & b \\
c & d \\
e & f
\end{bmatrix} =
\begin{bmatrix}
1 & 1 \\
2 & 0 \\
1 & -1
\end{bmatrix}
$$
che avrà dimensione $\dim(\mathrm{Codom}(t)) \times \dim(\mathrm{Dom}(t)) = \dim(\mathbb{R}^3) \times \dim(\mathbb{R}^2) = 3 \times 2$.
