# Concetti

Tra i termini relativi ai grafi ci sono:

- **Adiacenza** e **incidenza**

	Due nodi $v$ e $u$ sono **adiacenti** se esiste un arco $(u, v) \in E$, che a sua volta è detto **incidente** a $v$ e $u$.

- **Intorno**

	L'**intorno** di un vertice $u \in V$ sono quei vertici che gli sono _adiacenti_:
	$$
	N(u) = \Set{v \in V | (u, v) \in E}
	$$

- **Densità**

	La **densità** di un grafo $0 \leq \delta(G) \leq 1$ indica il rapporto tra $|E|$ e il numero totale di possibili archi, per cui la funzione diventa $\delta(G) = \frac{m}{n^2}$ se _orientato_ e $\delta(G) = \frac{2m}{n(n-1)}$ se _non orientato_.

	Il grafo è detto **sparso** se $\delta(G) \to 0$ e quindi $|E| \approx |V|$, e **denso** se $\delta(G) \to 1$ ovvero se $|E| \approx |V^2|$.
	Se $\delta(E_n) = 0$ il grafo $E_n$ si dice **vuoto**, mentre se $\delta(K_n) = 1$ il grafo $K_n$ si dice **completo**.

- **Peso**

	Un grafo si dice **pesato** se $G = (V, E, w)$ dove $w$ è una **funzione** peso che assegna ai _vertici_, se $w\colon V \to \mathbb{R}$, o agli _archi_, se $w\colon E \to \mathbb{R}$, un valore reale chiamato **peso**.

- **Sottografo**

	Un grafo $G' = (V', E')$ è **sottografo** di $G = (V, E)$ se:
	$$
	V' \subseteq V \land E' \subseteq E \cap V'^2
	$$
	ed è detto **sottografo indotto** $E = E \cap V'^2$ con notazione $G' = G[V']$.

	Per esempio dal primo grafo, il secondo è _sottografo_ e il terzo è _sottografo indotto_:
	```dot process
	digraph {
		node [shape=circle]
		edge [arrowsize=0.8]

		1:e -> 2 [dir=back]
		1:w -> 4
		1 -> 3
		4 -> 3:w
		2 -> 3:e

		10 [label="1"]
		20 [label="2"]
		30 [label="3"]
		40 [shape=point width=0]

		{
			rank=same
			2 -> 40 [minlen=2 style=invis]
		}
		10:e -> 20 [dir=back]
		10:w -> 40 [style=invis]
		10 -> 30 [weight=100]
		20 -> 30:e [style=invis]

		11 [label="1"]
		21 [label="2"]
		31 [label="3"]
		41 [shape=point width=0]

		{
			rank=same
			20 -> 41 [minlen=2 style=invis]
		}
		11:e -> 21 [dir=back]
		11:w -> 41 [style=invis]
		11 -> 31 [weight=100]
		21 -> 31:e
	}
	```

- **Cammino**

	Per i grafi, un [cammino](../../ct0371-2/02/README.md#caratteristiche) è detto **semplice** quando ogni nodo è distinto, ed è un **ciclo** se $n_0 = n_k$.

- **Connessione**

	Un grafo è **connesso** se per ogni $u, v \in V$ esiste un cammino $(u, ..., v)$, altrimenti è **disconnesso**.

	Si dice poi **componente connessa** il sottoinsieme $V' \subseteq V$ per cui:
	1. $G[V']$ è _connesso_
	2. $V' \not\subset V''$ se $G[V'']$ è _connesso_, per cui $V'$ non fa parte di una _componente connessa_ più grande

	L'insieme delle _componenti connesse_ di $G$ formano una **partizione** di $V$.

	Per esempio nel seguente grafo ci sono tre _componenti connesse_:
	```dot process
	digraph {
		node [shape=circle]
		edge [dir=none]

		{
			rank=same
			1
			2
		}

		1 -> 9 [style=invis]
		9 [shape=point width=0]

		2 -> 3, 4
		{
			rank=same
			9 -> 3 [minlen=2 style=invis]
			3 -> 4
			4 -> 7 [minlen=2 style=invis]
			7 -> 8
		}

		{
			rank=same
			5 -> 6
		}
		5 -> 7
		6 -> 8
	}
	```

- **Complemento**

	Un grafo $G^C = (V, E^C)$ è **complemento** di $G = (V, E)$ se:
	$$
	\forall (u, v) \in E^C, (u, v) \not\in E
	$$
	infatti $K_n^C = E_n$, ovvero il grafo _completo_ è _complemento_ di quello _vuoto_.

	Per esempio, il primo grafo è complemento del secondo e viceversa:
	```dot process
	digraph {
		node [shape=circle]
		edge [dir=none]

		11 [label="1"]
		21 [label="2"]
		31 [label="3"]

		1 -> 2, 3
		11 -> 21, 31 [style=invis]
		{
			rank=same
			2 -> 3 [style=invis]
			3 -> 21 [minlen=2 style=invis]
			21 -> 31
		}
	}
	```

- **Grafo bipartito**

	Un grafo è **bipartito** se può essere diviso in **due parti** in cui non ci sono archi.

	Per esempio, in questo grafo i vertici di ogni suddivisione non sono _adiacenti_:
	```dot process
	digraph {
		rankdir=LR
		node [shape=circle]
		edge [dir=none]

		subgraph cluster_0 {
			1 -> _0 -> 2 [weight=100 style=invis]
		}

		subgraph cluster_1 {
			3 -> _1 -> 4 [weight=100 style=invis]
		}

		1 -> 3 [weight=100]
		3 -> 2
		2 -> 4

		_0 [shape=point width=0]
		_1 [shape=point width=0]
	}
	```
