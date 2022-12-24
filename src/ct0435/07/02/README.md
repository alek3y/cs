# Basi

Si dice **base**, l'insieme **ordinato** $\{v_1, ..., v_n\} \subseteq V$ (ordinamenti diversi sono basi diverse) per cui:
- $\{v_1, ..., v_n\}$ è un **sistema di generatori**
- $v_1, ..., v_n$ sono vettori **linearmente indipendenti**

per cui ogni vettore $w \in V$ è espresso unicamente da una sola combinazione lineare.

In pratica, una base contiene il numero minimo di vettori richiesti per generare l'intero spazio vettoriale.

## Base canonica

Una base $B$ è detta **canonica** se segue la forma:
$$
\begin{pmatrix}
(1, 0, 0, ..., 0) \\
(0, 1, 0, ..., 0) \\
(0, 0, 1, ..., 0) \\
\vdots \\
(0, 0, 0, ..., 1)
\end{pmatrix}
$$

## Base ortogonale e ortonormale

Una base $B = \{v_1, ..., v_n\}$ si dice **ortogonale**, se:
$$v_i \cdot v_j = 0, \enspace \forall i, j : i \neq j$$

Oltre ad essere _ortogonale_ è anche detta **ortonormale** se:
$$||v_i|| = 1,\; \forall i$$
