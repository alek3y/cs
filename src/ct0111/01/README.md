# Probabilità elementare

Nella _probabilità_, un insieme $\Omega$ è detto **spazio campionario** se contiene tutti i risultati di un esperimento.
L'insieme $A$ è detto **evento** se $A \subseteq \Omega$, infatti è **evento certo** se $A = \Omega$ e **evento elementare** se $|A| = 1$.

Vengono chiamati **eventi incompatibili** invece, se
$$
A_i \cap A_j = \emptyset,\ \forall i, j = 1, ..., n : i \neq j
$$
ovvero se nessuno ha intersezione.

## Definizione

La **probabilità** è definita come una _funzione_ che assegna ad ogni _evento_ un valore in $\mathbb{R}^\geq$, per cui:
1. $0 \leq P(A) \leq 1$

2. $P(\Omega) = 1$

3. $P\left(\bigcup\limits_{i = 1}^n A_i\right) = \sum\limits_{i = 1}^n P(A_i)$ se $A_i$ sono _incompatibili_

Per cui si può **spezzettare** $A$ con delle [partizioni](../../ct0434/02/README.md#partizioni) $C_i$ di $\Omega$ per semplificare il calcolo della _probabilità_:
$$
P(A) = \sum_{i = 1}^{n} P(A \cap C_i)
$$
