# Ambiguità

Una _CFG_ è **ambigua** sse esiste $w \in L(G)$ tale che $w$ ha almeno **due parse tree diversi**.

Per esempio, se $G$ è
$$
E \rightarrow E + E \mid E \times E \mid a
$$
allora $a + a \times a \in L(G)$ produce i seguenti parse tree:
```dot process
digraph {
	pad=0
	ranksep=0.25
	color="transparent"
	node [shape=rect fixedsize=true width=0.25 height=0.25 color="transparent"]
	edge [arrowhead=none]

	subgraph cluster_0 {
		label="(a)"
		labeljust="l"
		labelloc="t"

		a1 [label="E"]
		a2 [label="E"]
		a3 [label="+"]
		a4 [label="E"]
		a5 [label="a"]
		a6 [label="E"]
		a7 [label="×"]
		a8 [label="E"]
		a9 [label="a"]
		a10 [label="a"]

		a1 -> a2, a3, a4
		a2 -> a5
		a4 -> a6, a7, a8
		a6 -> a9
		a8 -> a10
	}

	subgraph cluster_1 {
		label="(b)"
		labeljust="l"
		labelloc="t"

		b1 [label="E"]
		b2 [label="E"]
		b3 [label="×"]
		b4 [label="E"]
		b5 [label="a"]
		b6 [label="E"]
		b7 [label="+"]
		b8 [label="E"]
		b9 [label="a"]
		b10 [label="a"]

		b1 -> b2, b3, b4
		b2 -> b6, b7, b8
		b4 -> b5
		b6 -> b9
		b8 -> b10
	}
}
```

Questo **non vale con derivazioni diverse**, perchè per esempio con $R = \{S \rightarrow AB, A \rightarrow 0, B \rightarrow 1\}$ si possono avere le derivazioni $AB \Rightarrow 0B \Rightarrow 01$ e $AB \Rightarrow A1 \Rightarrow 01$ ma la grammatica non è ambigua.
