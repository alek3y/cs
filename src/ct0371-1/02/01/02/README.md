# Grado

Dato un grafo **non orientato** e la sua _matrice di adiacenza_ $A = (a_{ij}) = (a_{ji})$, il **grado** di un suo nodo $i$ è:
$$
\deg(i) = |N(i)| = \sum_{j \in V} a_{ij}
$$

Per i grafi **orientati** la _matrice di adiacenza_ non è _simmetrica_, quindi il **grado** si divide in **entrante** e **uscente**:
$$
\begin{split}
\text{in-deg}(i) &= \sum_{j \in V} a_{ji} \\
\text{out-deg}(i) &= \sum_{j \in V} a_{ij}
\end{split}
$$
da cui si ricava che $\sum\limits_{i \in V} \text{in-deg}(i) = \sum\limits_{i \in V} \text{out-deg}(i) = |E|$.

## Proprietà

- **Numero di cammini**

	Dalla [matrice di adiacenza](../../../ct0435/06/README.md#matrice-di-adiacenza) si può ricavare il **numero di cammini** lunghi $k$ da ogni nodo $i$ a $j$:
	$$
	A^k = (a_{ij}^{(k)})
	$$

	In particolare nei grafi _non orientati_ $A = A^T$, quindi se $k = 2$ si ha che:
	$$
	a_{ii}^{(2)} = A_i \cdot A^i \underset{A_i = A^i}{=} A_i \cdot A_i^T = \sum_{j = 1}^n {a_{ij}}^2 = \sum_{j = 1}^n a_{ij} = \deg(i)
	$$

	Si può dimostrare, per _induzione_ su $k$, che $A^k = A^{k-1} \times A$ contiene il numero di cammini lunghi $k$:
	- **Caso base**, per $k = 1$: $a_{ij}^{(1)}$ è il numero di archi da $i$ a $j$ e quindi cammini lunghi $1$
	- **Caso base**, per $k = 2$:
		$$
		a_{ij}^{(2)} = A_i \cdot A^j = \sum_{l = 1}^n a_{il} \cdot a_{lj}
		$$
		conta il cammino da $i$ a $j$ via $l$ sse $a_{il} \cdot a_{lj} = 1$, cioè se $a_{il}, a_{lj} \neq 0$ e quindi $(i, l), (l, j) \in E$.
	- **Passo induttivo**, assumendo che valga per $k-1$:
		$$
		a_{ij}^{(k)} = \sum_{l \in V} a_{il}^{(k-1)} \cdot a_{lj}
		$$
		dove $a_{il}^{(k-1)}$ è il numero di cammini lunghi $k-1$ da $i$ a $j$ per l'_ipotesi induttiva_ e, come per $k = 2$, il cammino è contato solo se $a_{lj} \neq 0$ e quindi se $(l, j) \in E$.

- **Almeno due nodi hanno lo stesso grado**

	Un grafo _n.o._ avrebbe tutti i **gradi distinti** se fossero $0, ..., n-1$, ovvero se il nodo con grado $0$ non avesse nodi _adiacenti_, ma è una **contraddizione** perchè quello con grado $n-1$ lo sarebbe a tutti.

- **Lemma della stretta di mano**

	Per un grafo _non orientato_ $G = (V, E)$ la somma dei gradi corrisponde a:
	$$
	\sum_{v \in V} \deg(v) = 2|E| = 2m
	$$
	perchè ogni arco sta tra due nodi e quindi è **contato due volte**.

- **Il numero di vertici con grado dispari è pari**

	Dal _lemma della stretta di mano_ si ricava che, dato $P$ l'insieme dei vertici con grado pari e $D = V \setminus P$:
	$$
	\begin{split}
	2m &= \sum_{u \in V} \deg(u) = \sum_{u \in P} \deg(u) + \sum_{u \in D} \deg(u) = \\
	&= \sum_{u \in P} 2\frac{1}{2}\deg(u) + \sum_{u \in D} \left(2\frac{1}{2}(\deg(u)-1) + 1\right) = \\
	&= 2\left(\sum_{u \in P} \frac{1}{2}\deg(u) + \sum_{u \in D}\frac{1}{2}(\deg(u) - 1)\right) + \sum_{u \in D} 1 = \\
	&= 2\left(\sum_{u \in V} \frac{1}{2}(\deg(u) - (\deg(u) \bmod 2)) \right) + |D| = \\
	&= 2h(V) + |D|
	\end{split}
	$$
	da cui si può concludere che $|D| = 2m - 2h(V)$ che è **pari**, perchè $h(V) \in \mathbb{N}$.

- **Grafi $k$-regolari**

	Dal _lemma della stretta di mano_ si ricavano anche i **grafi $k$-regolari**, ovvero i grafi $G = (V, E)$ tali che:
	$$
	\forall u \in V,\ \deg(u) = k
	$$

	In particolare, un grafo $G = (V, E)$ ha proprietà:
	- $|V| = |E|$, se è $2$-regolare:
		$$
		2m = \sum_{u \in V} \deg(u) = \sum_{u \in V} 2 = 2n
		$$
	- $|V|$ è pari, se è $3$-regolare:
		$$
		2m = \sum_{u \in V} \deg(u) = \sum_{u \in V} 3 = 3n
		$$
		quindi $n = 2\frac{m}{3}$ che è pari, dato che $\frac{m}{3} \in \mathbb{N}$.
	- $|E|$ è pari, se è $4$-regolare:
		$$
		2m = \sum_{u \in V} \deg(u) = \sum_{u \in V} 4 = 4n
		$$
		quindi $m = 2n$.
	- $|V|$ ed $|E|$ sono entrambi pari o dispari, se è $6$-regolare:
		$$
		2m = \sum_{u \in V} \deg(u) = \sum_{u \in V} 6 = 6n
		$$
		quindi $m = 3n$.
