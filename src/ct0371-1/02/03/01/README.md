# Dijkstra

<!-- TODO:
- Menziona che è molto simile all'algoritmo di prim, che usa coda di minima priorità su d[u]
- Menziona che serve per cammini minimi a singola sorgente/multiple destinazioni
- Menziona che invertendo gli archi e usando la destinazione come sorgente si può usare per sorgente multipla e destinazione singola
- Menziona che funziona solo con pesi positivi -->

```c
dijkstra(G, w, s)
	d, 𝜋 = init_ss(G, s)
	Q = G.V	// Coda di minima priorità in base a d
	while Q.heap_size > 0
		u = extract_min(Q)
		for each v in neighbors(G, u)
			relax(u, v, w, d, 𝜋)
	return (d, 𝜋)
```
