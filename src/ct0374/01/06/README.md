# Esercizi

- _Regex_ e _DFA_ dell'insieme $D$ di stringhe con numero pari di $a$, dispari di $b$ e che non contengono $ab$

	La _regex_ sarà $R = b(bb)^\ast(aa)^\ast$, mentre il _DFA_:
	```dot process
	digraph {
		rankdir=LR
		node [shape=circle label="" fixedsize=true width=0.4 height=0.4]
		edge [arrowsize=0.8]

		1
		2 [shape=doublecircle]
		3
		4
		5 [shape=doublecircle]
		p
		_0 [shape=point width=0 height=inf style=invis]

		_0 -> 1
		{
			rank=same
			1 -> 2 [label="b"]
		}
		1 -> p [label="a"]
		2 -> 3 [label="b"]
		2 -> 4 [label="a" weight=100]
		3 -> 2 [label="b"]
		3 -> p [label="a"]
		4 -> 5 [label="a"]
		4 -> p [label="b"]
		5 -> 4 [label="a"]
		5 -> p [label="b"]
	}
	```

- L'insieme $D$ di stringhe con lo stesso numero di $01$ e $10$ è regolare

	Questo è dimostrato perchè esiste un _DFA_ che lo riconosce:
	```dot process
	digraph {
		rankdir=LR
		node [shape=circle label="" fixedsize=true width=0.4 height=0.4]
		edge [arrowsize=0.8]

		1 [shape=doublecircle]
		2 [shape=doublecircle]
		3
		4 [shape=doublecircle]
		5
		_0 [shape=point width=0 height=inf style=invis]

		_0 -> 1
		1 -> 2 -> 2 [label="0"]
		2 -> 3 -> 3 [label="1"]
		2 -> 3 [weight=100 style=invis]
		3 -> 2 [label="0"]
		1 -> 4 -> 4 [label="1"]
		4 -> 5 -> 5 [label="0"]
		4 -> 5 [weight=100 style=invis]
		5 -> 4 [label="1"]
	}
	```

- Se $A$ è regolare allora lo è anche $\bar{A} = \Set{w | w \not\in A}$

	Essendo $A$ regolare esiste un _DFA_ $D = (Q, \Sigma, \delta, q_0, F)$ per cui $L(B) = A$.

	Sia allora $D' = (Q, \Sigma, \delta, q_0, Q \setminus F)$, da cui si conclude che $\forall w \in \Sigma^\ast, w \in L(D) \Leftrightarrow w \not\in L(D')$ perchè gli invertire gli stati finali porta ad accettare tutte le stringhe che non sono in $A$.

- $F = \Set{ww | w \in \{0, 1\}^\ast}$ non è regolare

	Per assurdo $F$ è regolare, allora si usa $s = 0^p10^p1 \in A$ con $|s| = 2p+2 \geq p$.

	Dato che $s = xyz$, $|y| > 0$ e $|xy| \leq p$, si ha che la $y$ si trova nei primi $p$ zeri perchè $|xy| \leq |0^p| = p$, quindi $xy^2z = 0^{p+k}10^p1$ (con $k > 0$ perchè $|y| > 0$) non fa parte di $F$ e allora non è regolare.

- $A = \Set{0^n1^n | n \geq 0}$ non è regolare

	Per assurdo $A$ è regolare, allora si sceglie $s = 0^p1^p \in A$ perchè $|s| = 2p \geq p$.

	Senza considerare che $|xy| \leq p$, si ottiene che:
	- $y$ si trova negli $0$: $xy^2z = 0^{p+k}1^p \not\in A$
	- $y$ si trova negli $1$: $xy^2z = 0^p1^{p+k} \not\in A$
	- $y$ si trova tra entrambi gli $0$ e $1$: $xy^2z$ avrà degli $1$ prima degli $0$, e.g.
		$$
		\underbrace{00}_{x}\underbrace{01}_{y}\underbrace{11}_{z} \in A
		\Rightarrow
		\underbrace{00}_{x}\underbrace{0101}_{y^2}\underbrace{11}_{z} \not\in A
		$$

- $A = \Set{a^{2^n} | n \geq 0}$ non è regolare

	Per assurdo $A$ è regolare, allora si sceglie $s = a^{2^p}$ perchè $|s| = 2^p \geq p$.

	Dato che $s = xyz$ con $|y| > 0$ e $|xy| \leq p$:
	$$
	2^p = |xyz|
	\hspace{0.5em}\underbrace{<}_{|y| > 0}\hspace{0.5em}
	|xy^2z|
	\hspace{0.5em}\underbrace{\leq}_{|xy| \leq p}\hspace{0.5em}
	2^p + p
	\hspace{0.5em}<\hspace{0.5em}
	2^p + 2^p = 2^{p+1}
	$$
	allora $2^p < |xy^2z| < 2^{p+1}$ di conseguenza $xy^2z \not\in A$ e $A$ non è regolare.

- $G = \Set{ab^nc^n | n \geq 0}$ non è regolare

	Per assurdo $G$ è regolare, allora si usa $s = ab^pc^p$ perchè $|s| = 2p+1 \geq p$.

	Siccome $s = xyz$ con $|y| > 0$ e $|xy| \leq p$, si ha che:
	- $a$ è in $y$: $xy^2z \not\in G$ perchè contiene due $a$
	- $a$ non è in $y$: $y$ contiene solo $b$ perchè $|xy| = |ay| \leq p$ e quindi $xy^2z = ab^{p+k}c^p \not\in G$

- $F = \Set{a^ib^jc^k | i, j, k \geq 0 \land (i = 1 \Rightarrow j = k)}$ non è regolare

	Per assurdo $F$ è regolare.

	Usando l'esercizio precedente si può notare che $G = F \cap a b^\ast c^\ast$, allora siccome sia $F$ (per ipotesi) che $a b^\ast c^\ast$ sono regolari anche $G$ deve esserlo per chiusura dell'intersezione, che è assurdo.
