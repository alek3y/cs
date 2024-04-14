# Cammini minimi

Tra le proprietà di un [cammino](../../../ct0371-2/02/README.md) $p = (n_0, ..., n_k)$ è definita la **lunghezza**:
$$
w(p) = \sum_{i = 1}^k w(n_{i-1}, n_i)
$$
l'**insieme di cammini** $\mathscr{C}(u, v)$, che contiene tutti i cammini da $u$ a $v$, e la **distanza**:
$$
\delta(u, v) = \begin{cases}
\min\limits_{p \in \mathscr{C}(u, v)} w(p) & \text{se } \mathscr{C}(u, v) \neq \emptyset \\
\infty & \text{se } \mathscr{C}(u, v) = \emptyset
\end{cases}
$$
che diventerebbe $-\infty$ se ci fosse un **ciclo negativo**, perchè allora esisterebbe sempre un $w(p)$ minore.
