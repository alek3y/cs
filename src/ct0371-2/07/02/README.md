# Longest common subsequence

In questo problema si vogliono cercare le **massime sottosequenze comuni** $X_k = x_{i_1}...x_{i_k}$ di massima lunghezza tra due sequenze $X = x_1...x_m$ e $Y = y_1...y_n$ sapendo che:
$$
i_1, ..., i_k \in \{1, ..., n\}\ \land\ i_1 < i_2 < ... < i_k
$$

Per esempio, $\mathrm{LCS}(ABCD, ACBD) = \{ABD, ACD\}$.

## Sottostruttura ottima

Di una sequenza $X$ lunga $m$, un carattere $x_i$ può essere **incluso o meno** e quindi si avranno $O(2^m)$ alternative.

Si dice **prefisso** di $X$ la sequenza $X^k = x_1...x_k$ lunga $k \leq m$ per cui $X^0 = \epsilon$, $X^1 = x_1$ e $X^m = X$.

Data quindi una sequenza $W = w_1...w_k \in \mathrm{LCS}(X, Y)$, la **sottostruttura ottima** diventa:
- Se $x_m = y_n$, allora $w_k = x_m = y_n$ e $W^{k-1} \in \mathrm{LCS}(X^{m-1}, Y^{n-1})$
- Se $x_m \neq y_n$, allora:
	- Se $w_k \neq x_m$, allora $W \in \mathrm{LCS}(X^{m-1}, Y)$
	- Se $w_k \neq y_n$, allora $W \in \mathrm{LCS}(X, Y^{n-1})$

che riduce il problema a $O(n \cdot m)$ alternative, perchè per ogni prefisso degli $n$ se ne possono scegliere $m$.

## Ricorsione

La **lunghezza** delle sottosequenze massime appartenenti a $\mathrm{LCS}(X^i, Y^j)$ si può trovare come:
$$
c_{i,j} = \begin{cases}
0 & \text{se } i = 0 \lor j = 0 \\
c_{i-1, j-1} + 1 & \text{se } i,j > 0 \land x_i = y_j \\
\max(c_{i-1, j}, c_{i, j-1}) & \text{se } i,j > 0 \land x_i \neq y_j
\end{cases}
$$

## Bottom-up

Sia $c$ una matrice $(m+1) \times (n+1)$ contenete le **lunghezze** $c_{i,j}$ delle sottosequenze in $\mathrm{LCS}(X^i, Y^j)$, e $b$ una matrice $m \times n$ contenente la miglior **direzione** presa dall'algoritmo partendo da $\mathrm{LCS}(X^i, Y^j)$:
- $b_{i,j} = {\nwarrow}$, se $x_i = y_j$ quindi si va a $\mathrm{LCS}(X^{i-1}, Y^{j-1})$
- $b_{i,j} = {\uparrow}$, se $x_i \neq y_j$ e si riduce a $\mathrm{LCS}(X^{i-1}, Y^j)$
- $b_{i,j} = {\leftarrow}$, se $x_i \neq y_j$ e si riduce a $\mathrm{LCS}(X^i, Y^{j-1})$

```c
LCS(X, Y)
	m = X.length
	n = Y.length
	for i = 0 to m
		c[i, 0] = 0
	for j = 1 to n
		c[0, j] = 0
	for i = 1 to m
		for j = 1 to n
			if X[i] == Y[j]
				c[i, j] = c[i-1, j-1] + 1
				b[i, j] = ↖
			else
				if c[i-1, j] >= c[i, j-1]
					c[i, j] = c[i-1, j]
					b[i, j] = ↑
				else
					c[i, j] = c[i, j-1]
					b[i, j] = ←
	return b, c
```

La complessità è data dalla soluzione di **tutti i sottoproblemi**, quindi $T(m, n) = \Theta(m \cdot n)$.

Si può quindi calcolare e **visitare la soluzione** con la stessa complessità di `LCS`:
```c
print_LCS(X, Y)
	b, c = LCS(X, Y)
	print_LCS_aux(X, b, X.length, Y.length)

print_LCS_aux(X, b, i, j)
	if i > 0 and j > 0
		if b[i, j] == ↖
			print_LCS_aux(X, b, i-1, j-1)
			print(X[i])
		else
			if b[i, j] == ↑
				print_LCS_aux(X, b, i-1, j)
			else
				print_LCS_aux(X, b, i, j-1)
```
dove `print_LCS_aux` ha complessità $O(i + j)$ perchè nel caso peggiore deve decrementare sia $i$ che $j$.

Per esempio, dati $X = AAC$ e $Y = ACB$ le tabelle $c$ e $b$ ricavate saranno:
$$
\begin{array}{r|rrrr}
& Y \\
X & \epsilon & \textcolor{mediumseagreen}{A} & \textcolor{mediumseagreen}{C} & B \\ \hline
\epsilon & 0 & 0 & 0 & 0 \\
A & \textcolor{indianred}{0} & \nwarrow 1 & \leftarrow 1 & \leftarrow 1 \\
\textcolor{mediumseagreen}{A} & 0 & \textcolor{mediumseagreen}{\nwarrow 1} & \uparrow 1 & \uparrow 1 \\
\textcolor{mediumseagreen}{C} & 0 & \uparrow 1 & \textcolor{mediumseagreen}{\nwarrow 2} & \textcolor{indianred}{\leftarrow 2} \\
\end{array}
$$

### Ottimizzazione

Un'ottimizzazione consiste nel ridurre lo spazio consumato **rimuovendo** $b$, perchè $c_{i,j}$ dipende dai vicini:
```c
print_LCS_aux(X, c, i, j)
	if i > 0 and j > 0
		if c[i, j] == c[i-1, j]
			print_LCS_aux(X, c, i-1, j)
		else if c[i, j] == c[i, j-1]
			print_LCS_aux(X, c, i, j-1)
		else
			print_LCS_aux(X, c, i-1, j-1)
			print(X[i])
```

Dall'esempio precedente si nota che è importante l'**ordine dei controlli**, infatti se il vicino verso $\nwarrow$ fosse controllato per primo, dalla cella $(x_3, y_3) = (C, B)$ si passerebbe a $(x_2, y_2) = (A, C)$.

Inoltre con un vettore di $\min(m, n)$ elementi è anche possibile rimuovere $c$ se interessa **solo la lunghezza**.

## Top-down
```c
LCS(X, Y)
	m = X.length
	n = Y.length
	c = []
	for i = 1 to m*n
		push(c, -1)
	return LCS_aux(X, Y, c, m, n)

LCS_aux(X, Y, c, i, j)
	if c[i, j] == -1
		if i == 0 or j == 0
			c[i, j] = 0
		else
			if X[i] == Y[j]
				c[i, j] = LCS_aux(X, Y, c, i-1, j-1) + 1
			else
				c[i, j] = max(LCS_aux(X, Y, c, i-1, j), LCS_aux(X, Y, c, i, j-1))
	return c[i, j]
```

La complessità di `LCS_aux` è $O(m \cdot n)$ perchè al massimo riempie $c$, mentre con `LCS` diventa $\Theta(m \cdot n)$.
