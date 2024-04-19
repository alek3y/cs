# Dijkstra

L'algoritmo trova i **cammini minimi** da una **singola sorgente** $s$ a tutti i nodi di un grafo $G$ con **pesi positivi**, effettuando `relax` sui vicini dei nodi estratti con distanza minore, similarmente a [quello di Prim](../../02/03/README.md):
```c
dijkstra(G, w, s)
  d, 𝜋 = init_ss(G, s)
  Q = G.V
  while Q.heap_size > 0
    u = extract_min(Q)
    for each v in neighbors(G, u)
      relax(u, v, w, d, 𝜋)
  return (d, 𝜋)
```

La **complessità** dipende da `Q`, infatti se $G$ è **denso** conviene usare un **array** perchè $m \approx n^2$:
$$
T(n, m) = \underbrace{O(n)}_{\texttt{init\_ss}} + n \cdot \hspace{-0.8em} \underbrace{O(n)}_{\texttt{extract\_min}} \hspace{-0.8em} + \underbrace{\sum\limits_{i = 1}^n \text{out-deg}(i)}_{m} \cdot \underbrace{O(1)}_{\texttt{relax}} = O(n^2)
$$
altrimenti se $G$ è **sparso** conviene una **coda di minima priorità** perchè $m \approx n$:
$$
T(n, m) = \underbrace{O(n)}_{\texttt{init\_ss}} + n \cdot \underbrace{O(\log n)}_{\texttt{extract\_min}} + \sum\limits_{i = 1}^n \text{out-deg}(i) \cdot \underbrace{O(\log n)}_{\texttt{relax}} = O(m\log n)
$$

## Correttezza

L'algoritmo è **corretto**, perchè alla fine si ha che $\forall u \in V, d_u = \delta(s, u)$ e $G_\pi$ è un _albero di cammini minimi_.

Si può dimostrare la prima affermazione **supponendo** _per assurdo_ che esista un $u \in V$ per cui $d_u \neq \delta(s, u)$ e che sia **il primo nodo** per cui capita.
In particolare, attraverso le seguenti osservazioni:
1. $u \neq s$, perchè $\delta(s, s) \neq -\infty$ perchè non ci sono pesi negativi, quindi $d_s = \delta(s, s) = 0$
2. All'estrazione di $u$ l'insieme $V \setminus Q \neq \emptyset$, perchè conterrà almeno $s$
3. Si può raggiungere $u$ da $s$, ovvero $\delta(s, u) \neq \infty$, perchè altrimenti $\delta(s, u) = \infty = d_u$ è contro l'ipotesi
4. Nello stato dell'algoritmo **dopo** l'estrazione di $s \in V \setminus Q$ e **prima** di $u \in Q$:

	Per il punto _3_ esiste un $p = (s, ..., u)$ **minimo**, e dato un $(x, y)$ in $p$ per cui $x \in V \setminus Q$ e $y \in Q$:
	1. $d_x = \delta(s, x)$ perchè $u$ è il primo per cui l'ipotesi assurda vale
	2. La `relax` su $(x, y)$ causa $d_y = \delta(s, x) + w(x, y) = \delta(s, y)$ per la [convergenza](../README.md#proprietà)
	3. Dato che si sta per estrarre $u$, allora $d_u \leq d_y$ perchè anche $y \in Q$
	4. $\delta(s, y) \leq \delta(s, u)$ perchè $(s, ..., y)$ è sottocammino di $p$ e non ci sono pesi negativi
	5. $\delta(s, u) \leq d_u$ per il [limite inferiore](../README.md#proprietà)

	Di conseguenza, con le osservazioni _4.5_, _4.3_, _4.2_, _4.4_ rispettivamente:
	$$
	\begin{split}
	\delta(s, u) \leq d_u &\leq d_y \\
	&= \delta(s, y) \\
	&\leq \delta(s, u)
	\end{split}
	$$
	ovvero che $\delta(s, u) \leq d_u \leq \delta(s, u)$ che però è **assurdo**, perchè va contro l'ipotesi.
