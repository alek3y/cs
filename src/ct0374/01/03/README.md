# Espressioni regolari

Una **regex** (o _regular expression_) è una notazione che descrive linguaggi regolari per cui, dato un alfabeto $\Sigma$:
- Se $a \in \Sigma$, allora $a$ è una _regex_
- $\epsilon$ è sempre una _regex_
- $\emptyset$ è anch'esso una _regex_
- Se $r_1$ e $r_2$ sono _regex_, allora $r_1 \cup r_2$ è una _regex_
- Se $r_1$ e $r_2$ sono _regex_, allora $r_1 \circ r_2$ è una _regex_
- Se $r$ è _regex_, allora $r^\ast$ è una _regex_

dove $^\ast$, $\circ$ e $\cup$ è l'ordine di rispettiva **precedenza**, e per convenzione $r_1r_2 = r_1 \circ r_2$.

Il **linguaggio** $L(R)$ di una _regex_ $R$ invece, è definito come:
1. Se $R = a$ allora $L(R) = \{a\}$
2. Se $R = \epsilon$ allora $L(R) = \{\epsilon\}$
3. Se $R = \emptyset$ allora $L(R) = \emptyset$
4. Se $R = R_1 \cup R_2$ allora $L(R) = L(R_1) \cup L(R_2)$
5. Se $R = R_1 \circ R_2$ allora $L(R) = L(R_1) \circ L(R_2)$
6. Se $R = {R_1}^\ast$ allora $L(R) = L(R_1)^\ast$

Per esempio, il linguaggio della _regex_ $(0 \cup 1)0^\ast$ è:
$$
\begin{split}
L((0 \cup 1)0^\ast) &\underset{5}{=} L(0 \cup 1) \circ L(0^\ast) = \\
&\underset{4, 6}{=} (L(0) \cup L(1)) \circ L(0)^\ast = \\
&\underset{1}{=} (\{0\} \cup \{1\}) \circ \{0\}^\ast = \{0, 1\} \circ \{0\}^\ast
\end{split}
$$
