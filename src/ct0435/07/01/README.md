# Combinazioni lineari

Una **combinazione lineare** dei _vettori_ $\vec{v}_1, ..., \vec{v}_n \in V$ con i _coefficienti_ $a_1, ..., a_n \in K$, è il **vettore**:
$$a_1\vec{v}_1 + a_2\vec{v}_2 + ... + a_n\vec{v}_n$$

## Dipendenza lineare

I vettori $\vec{v}_1, ..., \vec{v}_n \in V$ si dicono **linearmente indipendenti** se:
$$a_1\vec{v}_1 + ... + a_n\vec{v}_n = \vec{0} \Leftrightarrow a_1, ..., a_n = 0$$
cioè se la loro _combinazione lineare_ è $\vec{0}$ solamente quando i coefficienti sono $0$.

Si dicono **linearmente dipendenti** invece, se la _combinazione_ è $\vec{0}$ anche se $\exists i \in [1, n] : a_i \neq 0$. \
Infatti, se $a_1 \neq 0$:
$$
a_1\vec{v}_1 + a_2\vec{v}_2 + ... + a_n\vec{v}_n = 0 \\
\Downarrow \\
a_1\vec{v}_1 = -(a_2\vec{v}_2 + ... + a_n\vec{v}_n) \\
\Downarrow \\
\vec{v}_1 = -\frac{1}{a_1}(a_2\vec{v}_2 + ... + a_n\vec{v}_n)
$$
per cui sono _dipendenti_.

## Sistema di generatori

Un insieme $\{v_1, v_2, ..., v_n\} \subseteq V$ si dice **sistema di generatori** se:
$$\forall w \in V, \exists a_1, a_2, ..., a_n \in K : a_1v_1 + a_2v_2 + ... + a_nv_n = w$$
cioè se permette di trovare tutti i vettori dello spazio $V$ tramite opportune _combinazioni lineari_.
