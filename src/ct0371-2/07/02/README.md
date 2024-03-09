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
