# Cardinalità

L'argomento è già stato trattato superficialmente nella parte di [Calcolo 1](../../../ct0432/02/04/README.md).

Due insiemi $A$ e $B$ si dicono **equipotenti** (o **equipollenti**) se hanno la **stessa cardinalità**, cioè:
$$|A| = |B| \Leftrightarrow \exists f\colon A \to B \text{ biettiva}$$

Ricavare la cardinalità significa **contare** gli elementi dell'insieme, che a sua volta consiste nel **costruire una biezione**.

## Biezione

Creare una biezione **di un insieme**, consiste nell'associare ogni elemento ad un numero $n \in \mathbb{N}$:
$$|A| = n \Leftrightarrow \exists f\colon A \to \hat n \text{ biettiva}$$
dove $\hat n = \{1, 2, ..., n\}$.

Per esempio, la cardinalità di $A$ è $n$ se ogni elemento può essere associato singolarmente ad un elemento di $\hat n$ e quindi esiste una **biezione**:
$$
\begin{matrix}
A = \{& a, & b, & c, & ... &\} \\
&\downarrow &\downarrow &\downarrow &\downarrow & \\
\hat n = \{& 1, & 2, & 3, & ... &\}
\end{matrix}
$$
e quindi $\forall x \in A, \exists! y \in \hat n : y = f(x)$, per cui l'insieme si dice **numerabile**.

## Insiemi infiniti

Due insiemi infiniti $A$ e $B$ hanno la **stessa cardinalità**, quando:
$$|A| = |B| \Leftrightarrow \exists f\colon A \to B \text{ iniettiva} \land \exists g\colon B \to A \text{ iniettiva}$$

Per esempio, sia $P$ l'insieme dei numeri pari, allora $|P| = |\mathbb{N}|$ anche se $P \subset \mathbb{N}$.

Quindi qualsiasi insieme sia **numerabile** e **equipollente** a $\mathbb{N}$, avrà la cardinalità di $\mathbb{N}$:
$$|\mathbb{N}| = \aleph_0$$

Esistono più tipi di cardinalità infinite $\aleph$, dato che:
$$|\mathbb{N}| \neq |\mathbb{R}| = |P(\mathbb{N})| = 2^{\aleph_0} = \aleph_1$$
e si dicono **numeri cardinali transfiniti** perchè $\exists n : n > \aleph_0$.
