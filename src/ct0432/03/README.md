# Funzioni

La funzione $f$ è **funzione** da un insieme $A$ ad un insieme $B$ se $\forall x \in A, \exists!y \in B : x \rightarrow y$, dove $x \rightarrow y$ significa che da $x$ si va a $y$ tramite la funzione $f$.

Dato che $f$ va da $A$ a $B$, $A$ è chiamato **dominio** (o $D$), mentre $B$ è chiamato **codominio** (o $f(D)$).

La funzione verrà quindi descritta come
$$
\begin{split}
f\colon &A \rightarrow B\newline
&x \mapsto y = f(x)
\end{split}
$$
con $x \in A$ e $y \in B$, dove $x$ è detta **variabile indipendente** mentre $y$ **variabile dipendente**.

Per esempio, la seguente funzione:

![Esempio di funzione](assets/01.png)

## Grafico

Un **grafico** di una funzione $f: D \rightarrow \mathbb{R}$ viene indicato con $G(f)$ e corrisponde al sottoinsieme dei punti del piano cartesiano $\mathbb{R}^2$ messi in relazione da $f$:
$$G(f) = \{(x, y) \in \mathbb{R}^2 \left|\right. x \in D \land y = f(x)\}$$

## Immagine

L'immagine di una funzione $f: D \rightarrow \mathbb{R}$ verrà rappresentata da $f(D)$ o $\mathrm{Im}(f)$, ed è costituita dai valori assunti dalla funzione:
$$f(D) = \{y \in \mathbb{R} \left|\right. y = f(x) \text{ per qualche } x \in D\}$$

Per esempio, nel caso di $y = f(x) = mx + q$, che comprende tutte le rette **non verticali**, avrà l'immagine:
$$
f(D) =
\begin{cases}
\mathbb{R} & \text{se } m \neq 0 \\
\{q\} & \text{se } m = 0, \text{ retta orizzontale}
\end{cases}
$$
