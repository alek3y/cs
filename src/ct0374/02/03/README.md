# Automi a pila

Un **automa a pila** (o _PDA_) è una **sestupla** $(Q, \Sigma, \Gamma, \delta, q_0, F)$ equivalente alle _CFG_ che sfrutta uno **stack**:
```dot process
digraph {
	rankdir=LR
	ranksep=0.4
	node [shape=rect]
	edge [arrowsize=0.8]

	1 [label="Stati"]
	2 [shape=record label="{<p0> a | a | b | b}"]
	3 [shape=record label="<p1> x | y | z"]

	1 -> 2:p0 [label="Input"]
	1 -> 3:p1 [label="Stack"]
}
```
e i suoi componenti sono definiti come:
1. $Q$ è l'insieme degli **stati**
2. $\Sigma$ è l'**alfabeto dell'input**
3. $\Gamma$ è l'**alfabeto dello stack**
4. $\delta\colon Q \times \Sigma_\epsilon \times \Gamma_\epsilon \to P(Q \times \Gamma_\epsilon)$ è la **funzione di transizione**, con $\Sigma_\epsilon = \Sigma \cup \{\epsilon\}$ e $\Gamma_\epsilon = \Gamma \cup \{\epsilon\}$
5. $q_0 \in Q$ è lo **stato iniziale**
6. $F \subseteq Q$ è l'insieme degli **stati finali**
