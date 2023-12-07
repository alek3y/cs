# Altri tipi

Tra gli [alberi binari di ricerca](../04/README.md) ci sono anche altri tipi come:
- **Alberi AVL**

	Sono alberi [**bilanciati**](../README.md#alberi-k-ari) i cui nodi contengono un **fattore di bilanciamento** che è minore o uguale ad $1$ per ogni nodo, e rappresenta la differenza dell'altezza del sottoalbero sinistro con quella del destro.

	L'_inserimento_ e _cancellazione_ sono più complesse dato che bisogna mantenere l'albero bilanciato.

- **B-Alberi**

	Sono alberi **bilanciati** che hanno **almeno** due figli, dove:
	- Tutte le foglie hanno la stessa profondità
	- Ogni nodo $v$ (_radice_ esclusa) contiene $\mathrm{grado}(v)-1 \leq K(v) \leq 2\mathrm{grado}(v)-1$ chiavi **ordinate**
	- La radice $r$ contiene $1 \leq K(r) \leq 2\mathrm{grado}(r)-1$ chiavi **ordinate**
	- Ogni nodo interno $v$ ha $K(v)+1$ figli
	- Le chiavi separano gli intervalli delle chiavi nei sottoalberi

	Per esempio,
	```dot process
	graph {
		node [shape=record]
		0 [label="46"]
		1 [label="27 | 37"]
		2 [label="66 | 79"]
		3 [label="10 | 15 | 25"]
		4 [label="30 | 35"]
		5 [label="40 | 45"]
		6 [label="50 | 55 | 65"]
		7 [label="68 | 74"]
		8 [label="80 | 99"]

		0 -- 1, 2
		1 -- 3, 4, 5
		2 -- 6, 7, 8
	}
	```

- **Alberi rossi e neri**

	Contengono oltre alla chiave il **colore** del nodo, che può essere **rosso** o **nero**.

	L'albero viene vincolato in base al _colore_ in un modo che garantisce che l'albero sia **bilanciato**.
