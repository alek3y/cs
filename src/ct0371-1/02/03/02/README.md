# Bellman-Ford

L'algoritmo trova i **cammini minimi** da una **singola sorgente** $s$ a tutti i nodi di un grafo $G$ con possibili **pesi negativi**, sempre che sia **senza cicli negativi**, effettuando $n-1$ `relax` sugli archi:
```c
bellman_ford(G, w, s)
  d, 𝜋 = init_ss(G, s)
  for i = 1 to G.V.length - 1
    for each (u, v) in G.E
      relax(u, v, w, d, 𝜋)
  for each (u, v) in G.E
    if d[v] > d[u] + w(u, v)
      return (false, d, 𝜋)  // Possiede cicli negativi se c'è ancora un cammino più corto
  return (true, d, 𝜋)
```

La **complessità**, data da l'`init_ss` da $\Theta(n)$ e dalla `relax` da $\Theta(1)$ chiamata $(n-1) \cdot m$ volte, sarà:
$$
T(n, m) = \Theta(n) + (n-1) \cdot \Theta(m) + \Theta(m) = \Theta(nm)
$$

## Correttezza

L'algoritmo è **corretto**, perchè se restituisce `true` si avrà che $\forall u \in V, d_u = \delta(s, u)$ e $G_\pi$ è un _albero di cammini minimi_, altrimenti se restituisce `false` significa che un _ciclo negativo_ è **raggiungibile** da $s$.

Della prima situazione si può dimostrare che:
- $\forall u \in V, d_u = \delta(s, u)$

	Se $\delta(s, u) = \infty$ allora alla fine, siccome $u$ non è raggiungibile, anche $d_u = \infty$ grazie alla `init_ss`.
	Altrimenti, il cammino minimo [semplice](../../01/README.md) $p = (n_0 = s, ..., n_k = u)$, che avrà al più $n-1$ archi, verrà esplorato un nodo $n_i$ alla volta portando $d_{n_{i+1}}$ a $\delta(s, n_{i+1})$ con la `relax`, per la [convergenza](../README.md#proprietà).

- $\forall (u, v) \in E, d_v \leq d_u + w(u, v)$, ovvero restituisce `true`

	Per il punto precedente e la [disuguaglianza triangolare](../README.md#proprietà), la proprietà è verificata.
