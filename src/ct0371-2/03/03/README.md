# Quick sort

Il **quick sort** sceglie un elemento **pivot** spostando gli elementi minori a sinistra per **partizionare** l'array in `A[p..q-1]` e `A[q+1..r]` e poi riordinarli ricorsivamente, dove `A[q]` è il valore del _pivot_.

```c
quicksort(Array A, int p, int r)
  if p < r
    q = partition(A, p, r)
    quicksort(A, p, q-1)
    quicksort(A, q+1, r)

partition(Array A, int p, int r) -> int
  x = A[r]  // Pivot
  i = p - 1
  for j = p to r - 1
    if A[j] <= x
      i = i + 1
      swap(A[i], A[j])
  swap(A[i+1], A[r])
  return i+1
```

L'algoritmo è **corretto** per l'[invariante](../../01/02/README.md#analisi-della-correttezza) del `for`:
> Ogni elemento tra `p` ed `i` è **minore o uguale** al _pivot_ e tra `i+1` e `j-1` è **maggiore** del _pivot_

e quando il `for` termina `j = r` quindi l'_invariante_ vale con gli elementi tra `i+1` e `r-1` maggiori del _pivot_.

La complessità di `partition` è $\Theta(n)$ dato che impiega `r - p + 1 = n` iterazioni, mentre di `quicksort`:
$$
T(n) = \begin{cases}
O(1) & \text{se } n = 1 \\
T(k) + T(n - k - 1) + \Theta(n) & \text{se } n > 1
\end{cases}
$$
che si può risolvere separando nei diversi casi:
- **Peggiore**

	In questo caso una _partizione_ è grande $n - 1$ e l'altra è vuota, quindi:
	$$
	\begin{split}
	T(n) &= T(n - 1) + T(0) + \Theta(n) = T(n - 1) + \Theta(n) = \\
	&= T(n - 1) + cn = \\
	&= T(n - 2) + c(n - 1) + cn = \\
	&= \sum_{i = 1}^n ci = c\frac{n(n + 1)}{2} = \Theta(n^2)
	\end{split}
	$$

- **Migliore**

	In questo caso le _partizioni_ sono grandi $\lfloor\frac{n}{2}\rfloor$ e $\lfloor\frac{n}{2}\rfloor - 1$, quindi:
	$$
	T(n) = 2T\left(\frac{n}{2}\right) + \Theta(n) = \Theta(n \log n)
	$$
	per il [teorema master](../../../ct0371-1/01/03/README.md#teorema-master).

- **Medio**

	In questo caso una _partizione_ è grande $\frac{9}{10}n$ e l'altra $\frac{1}{10}n$, quindi:
	$$
	T(n) = T\left(\frac{9n}{10}\right) + T\left(\frac{n}{10}\right) + cn
	$$
	risolvibile con l'_albero delle ricorsioni_:
	```dot process
	graph {
		node [shape=box]

		0 [label="cn"]
		1 [label="¹⁄₁₀cn"]
		2 [label="⁹⁄₁₀cn"]
		3 [label="¹⁄₁₀₀cn"]
		4 [label="⁹⁄₁₀₀cn"]
		5 [label="⁹⁄₁₀₀cn"]
		6 [label="⁸¹⁄₁₀₀cn"]
		0 -- 1, 2
		1 -- 3, 4
		2 -- 5, 6

		{
			node [shape=point width=0]
			7, 8, 9, 10, 11, 12, 13, 14
		}
		{
			edge [style=dashed]
			3 -- 7, 8
			4 -- 9, 10
			5 -- 11, 12
			6 -- 13, 14
		}
	}
	```
	in cui ogni livello somma al più $cn$ dato che alcuni cammini **termineranno prima**, fino a:
	$$
	\frac{n}{10^i} = 1\ \lor\ \left(\frac{9}{10}\right)^i n = 1
	$$
	da cui si ricava che il cammino **più corto** è alto $\log_{10} n$ e quello **più lungo** $\log_{\frac{10}{9}} n$, quindi:
	$$
	T(n) \leq cn \cdot \log_{\frac{10}{9}} n = O(n \log n)
	$$

	Questo processo si può **generalizzare** per ogni $0 < \alpha < 1$ e $c > 0$:
	$$
	T(n) = T(\alpha n) + T((1 - \alpha)n) + cn = O(n \log n)
	$$

	Se però il caso medio provenisse dall'**alternarsi** del caso migliore $L(n)$ e peggiore $U(n)$ definiti come:
	$$
	\begin{split}
	L(n) &= 2U\left(\frac{n}{2}\right) + \Theta(n) \\
	U(n) &= L(n - 1) + \Theta(n)
	\end{split}
	$$
	si avrebbe che:
	$$
	\begin{split}
	L(n) &= 2\left(L\left(\frac{n}{2} - 1\right) + \Theta\left(\frac{n}{2}\right)\right) + \Theta(n) = \\
	&= 2L\left(\frac{n}{2} - 1\right) + 2\Theta\left(\frac{n}{2}\right) + \Theta(n) = \\
	&= 2L\left(\frac{n}{2} - 1\right) + \Theta(n) = \Theta(n \log n)
	\end{split}
	$$
	che è comunque $O(n \log n)$.

L'algoritmo è **in loco**, ma non è _stabile_.

## Pivot casuale

Scegliere un _pivot_ **casuale** al posto di `A[r]` diminuisce le probabilità che capiti il caso peggiore.

```c
random_quicksort(Array A, int p, int r)
  if p < r
    q = random_partition(A, p, r)
    random_quicksort(A, p, q-1)
    random_quicksort(A, q+1, r)

random_partition(Array A, int p, int r) -> int
  i = random(p, r)
  swap(A[i], A[r])
  return partition(A, p, r)
```

Come per il [merge sort](../02/README.md), è possibile migliorare le prestazioni sfruttando l'[insertion sort](../01/README.md) su array piccoli, oppure scegliendo il _pivot_ come **mediana** di tre elementi equidistanti.

## Chiavi duplicate

Nel caso fossero presenti chiavi **duplicate** si rischierebbe di avere array **sbilanciati** (e.g. nel caso di chiavi tutte uguali), che si possono evitare creando una _partizione_ per gli elementi uguali al _pivot_ `A[r]`:
```c
quicksort(Array A, int p, int r)
  if p < r
    <q, t> = partition(A, p, r)
    quicksort(A, p, q-1)
    quicksort(A, t+1, r)

partition(Array A, int p, int r) -> <int, int>
  x = A[r]
  min = eq = p
  mag = r
  while eq < mag
    if A[eq] < x
      swap(A[eq], A[min])
      min++
      eq++
    else if A[eq] == x
      eq++
    else
      mag--
      swap(A[eq], A[mag])
  swap(A[r], A[mag])
  return <min, mag>
```
che è **corretto** perchè il `while` ha _invariante_:
> Tra `p` e `min` sono **minori**, tra `min` ed `eq` sono **uguali** e tra `mag` e `r` sono **maggiori** del _pivot_

assicurando che quando il `while` termina `eq = mag` e quindi ci sono solo tre _partizioni_.

Questa versione di `partition` è $\Theta(r - p)$ cioè $\Theta(n)$, per cui la complessità di `quicksort` sarà la stessa.
