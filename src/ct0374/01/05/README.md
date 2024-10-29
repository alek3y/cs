# Pumping lemma

La dimostrazione di **non regolarità** è più difficile del contrario, ma è possibile con il **pumping lemma**.

Il lemma esprime che se $A$ è regolare **allora** esiste $p \geq 1$ tale che se $s \in A$ e $|s| \geq p$ allora $s = xyz$ e:
1. $xy^iz \in A ,\ \forall i \geq 0$
2. $|y| > 0$
3. $|xy| \leq p$

## Dimostrazione

Essendo regolare esiste un _DFA_ $D = (Q, \Sigma, \delta, q_0, F)$ tale che $L(D) = A$, allora si può scegliere come **pumping length** $p = |Q|$.
Sia poi $s = w_1w_2 \cdots w_n \in A$, allora:
- Se $n < p$, allora il teorema è vero perchè le condizioni non si applicano per queste stringhe
- Se $n \geq p$, allora $s$ è riconosciuta da una sequenza $q_0, q_1, ..., q_n$, con $q_n \in F$:
	```dot process
	digraph {
		rankdir=LR
		ranksep=0.25
		node [shape=circle]
		edge [arrowsize=0.8]

		q0 [label="q₀"]
		qi [label="qᵢ"]
		qn [label="qₙ" shape=doublecircle]

		_0 [shape=none label=<<table border="0" cellspacing="0" cellpadding="4"><tr><td>x</td></tr><tr><td port="p0"> ... </td></tr></table>>]
		_1 [shape=none label=<<table border="0" cellspacing="0" cellpadding="4"><tr><td>z</td></tr><tr><td port="p1"> ... </td></tr></table>>]
		_2 [shape=none label=<<table border="0" cellspacing="0" cellpadding="4"><tr><td>y</td></tr><tr><td port="p2"> ... </td></tr></table>>]

		q0 -> _0:p0 [arrowhead=none]
		_0:p0 -> qi
		qi -> _1:p1 [arrowhead=none]
		_1:p1 -> qn

		{
			rank=same
			_2:p2:w -> qi [arrowhead=none]
			_2:p2:e -> qi [style=invis]
			_2:p2:e -> qi
		}
	}
	```

	Dato che nella sequenza ci sono $n+1 \geq p+1$ stati (perchè le $w_i$ sono gli archi), c'è almeno uno stato $q_i$ che si ripete (come in figura) e di conseguenza la prima e la seconda condizione sono verificate.

	Infine alla visita del $(p+1)$-esimo stato, $q_i$ si sarà sicuramente già ripetuto. Di conseguenza, il _DFA_ avrà letto i primi $p$ caratteri che includono $xy$ (dato che $q_i$ si ripete su $y$), dimostrando la terza condizione.
