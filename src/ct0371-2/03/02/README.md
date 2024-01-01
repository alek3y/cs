# Merge sort

Il **merge sort** divide a metà l'array e riordina ricorsivamente le due parti per poi riunirle.

```c
mergesort(Array A, int p, int r)
  if p < r
    med = (p + r)/2
    mergesort(A, p, med)
    mergesort(A, med+1, r)
    merge(A, p, med, r)

merge(Array A, int p, int med, int r)
  L = []
  left_len = med - p + 1
  for i = 1 to left_len
    push(L, A[p + i-1])

  R = []
  right_len = r - med
  for i = 1 to right_len
    push(R, A[med + i])

  i = 1, j = 1
  for k = p to r
    if i <= left_len and (j > right_len or L[i] <= R[j])
      A[k] = L[i]
      i++
    else
      A[k] = R[j]
      j++
```

L'algoritmo è **corretto** per l'[invariante](../../01/02/README.md#analisi-della-correttezza) dell'ultimo `for`:
> In `A[p..k-1]` sono **ordinati** elementi che, come `L[i]` e `R[j]`, sono minori del resto di `L` e `R`

e quando il `for` termina `k = r + 1` quindi l'_invariante_ vale per `A[p..r+1-1]` cioè l'intero array.

La complessità di `merge` si può ricavare, sapendo che $n = r - p + 1$, da:
$$
\begin{split}
T(n) &= \Theta(\texttt{left\_len}) + \Theta(\texttt{right\_len}) + \Theta(r - p + 1) = \\
&= \Theta(\texttt{left\_len} + \texttt{right\_len}) + \Theta(n) = \\
&= \Theta(\texttt{med} - p + 1 + r - \texttt{med}) + \Theta(n) = \\
&= \Theta(r - p + 1) + \Theta(n) = \Theta(n)
\end{split}
$$
mentre quella di `mergesort` è $T(n) = 2T(\frac{n}{2}) + \Theta(n) = \Theta(n \log n)$ per il [teorema master](../../../ct0371-1/01/03/README.md#teorema-master).

L'algoritmo è **stabile** ma non _in loco_, ed è migliorabile con l'[insertion sort](../01/README.md) su array piccoli.
