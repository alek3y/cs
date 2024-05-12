# Floyd-Warshall

L'algoritmo trova le **distanze minime**, restituendole su $D^{(n)}$, tra **tutte le coppie** di nodi in $V = \{1, ..., n\}$, su un grafo $G$ _orientato_ con possibili **pesi negativi** riportati nella [matrice di adiacenza](../../01/01/README.md) $W$:
```c
floyd_warshall(W)
  n = W.rows = W.colums
  D = {}
  D[0] = W
  for k = 1 to n
    for i = 1 to n
      for j = 1 to n
        D[k][i,j] = min(D[k-1][i,k] + D[k-1][k,j], D[k-1][i,j])
  return D[n]
```

All'interno della matrice $W$, i **pesi** assumono valori:
$$
w_{ij} = \begin{cases}
0 & \text{se } i = j \\
\infty & \text{se } (i, j) \not\in E \\
w(i, j) & \text{altrimenti}
\end{cases}
$$

La **complessità** è data dai tre cicli annidati, quindi $T(n) = \Theta(n^3)$.

## Correttezza

L'algoritmo è **corretto** perchè alla fine, degli elementi della matrice $D^{(n)}$, si ha che $d^{(n)}_{ij} = \delta(i, j)$.

Si può dimostrare definendo $\mathscr{D}_{ij}$, cioè l'**insieme dei cammini** [semplici](../../01/README.md) tra $i$ e $j$, e $\mathscr{D}^{(k)}_{ij}$, cioè l'insieme dei cammini $p \in \mathscr{D}_{ij}$ i cui **nodi intermedi**, ovvero esclusi $i$ e $j$, hanno valore $n_i \leq k$.

Con questo si può notare che $\mathscr{D}^{(k-1)}_{ij} \subseteq \mathscr{D}^{(k)}_{ij}$ e che alla fine $\mathscr{D}^{(n)}_{ij} = \mathscr{D}_{ij}$ perchè **tutti i nodi** sono inclusi.

Si può quindi definire $d^{(k)}_{ij}$ come il **costo minimo** dei cammini tra $i$ e $j$ che includono **fino al $k$-esimo nodo**:
$$
d^{(k)}_{ij} = \min_{p \in \mathscr{D}^{(k)}_{ij}} w(p)
$$
per cui **alla fine** $d^{(n)}_{ij} = \delta(i, j)$ perchè $\mathscr{D}^{(n)}_{ij} = \mathscr{D}_{ij}$ e per la [definizione](../README.md) $\delta(i, j) = \min\limits_{p \in \mathscr{D}_{ij}} w(p)$.

### Generazione matrici

La correttezza del **corpo dei cicli** si può dimostrare **partizionando** $\mathscr{D}^{(k)}_{ij}$ in $\hat{\mathscr{D}}^{(k)}_{ij}$, ovvero l'insieme dei cammini $p \in \mathscr{D}^{(k)}_{ij}$ **passanti** per $k$, e in $\mathscr{D}^{(k-1)}_{ij}$, ovvero l'insieme dei cammini **non passanti** per $k$:
$$
\mathscr{D}^{(k)}_{ij} = \hat{\mathscr{D}}^{(k)}_{ij} \cup \mathscr{D}^{(k-1)}_{ij}
\hspace{0.5em}\land\hspace{0.5em}
\hat{\mathscr{D}}^{(k)}_{ij} \cap \mathscr{D}^{(k-1)}_{ij} = \emptyset
$$

Dalla definizione di $d^{(k)}_{ij}$ ottenuta precedentemente è quindi possibile ricavare l'espressione dell'algoritmo:
$$
\begin{split}
d^{(k)}_{ij} &= \min_{p \in \mathscr{D}^{(k)}_{ij}} w(p) = \\
&= \min\left(
\min_{p \in \hat{\mathscr{D}}^{(k)}_{ij}} w(p),
\min_{p \in \mathscr{D}^{(k-1)}_{ij}} w(p)
\right) = \\
&= \min\left(
\min_{p \in \mathscr{D}^{(k-1)}_{ik}} w(p) + \min_{p \in \mathscr{D}^{(k-1)}_{kj}} w(p),
\min_{p \in \mathscr{D}^{(k-1)}_{ij}} w(p)
\right) = \\
&= \min\left(d^{(k-1)}_{ik} + d^{(k-1)}_{kj}, d^{(k-1)}_{ij}\right)
\end{split}
$$
dato che, essendo _semplici_, i cammini _passanti_ per $k$ si possono **suddividere** in due _non passanti_ per $k$.

### Ottimizzazione

L'algoritmo si può ottimizzare **riducendo** il numero di matrici, tenendone una per l'iterazione corrente e una per la precedente.
Inoltre è anche possibile salvarne **una sola**, purchè si soddisfino le condizioni per cui:
1. $d^{(k)}_{ii} = 0,\; \forall i, k = 1, ..., n$

	Si può dimostrare per induzione su $k$:
	- **Caso base**, per $k = 0$: è verificato per la definizione di $W$, perchè $d^{(0)}_{ii} = w_{ii} = 0$
	- **Passo induttivo**, assumendo che valga fino a $k-1$:
		$$
		d^{(k)}_{ii} = \min\Bigl(\underbrace{d^{(k-1)}_{ik} + d^{(k-1)}_{ki}}_{\geq 0}, \underbrace{d^{(k-1)}_{ii}}_{= 0}\Bigr) = 0
		$$
		Inoltre, questo evidenzia che se $d^{(k)}_{ii} < 0$, allora sarebbero **presenti cicli negativi**.

2. $\begin{cases}d^{(k)}_{ik} = d^{(k-1)}_{ik} \\ d^{(k)}_{kj} = d^{(k-1)}_{kj}\end{cases},\; \forall i, j, k = 1, ..., n$

	Si può dimostrare partendo dalla definizione di $d^{(k)}_{ij}$, infatti:
	$$
	d^{(k)}_{ik} = \min\left(d^{(k-1)}_{ik}, d^{(k-1)}_{ik} + d^{(k-1)}_{kk}\right) = d^{(k-1)}_{ik}
	$$
