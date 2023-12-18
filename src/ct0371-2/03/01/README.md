# Insertion sort

L'**insertion sort** consiste nell'estendere una parte di $k$ elementi ordinati al $(k+1)$-esimo elemento.

```c
insertion(Array A)
  for j = 2 to A.length
    key = A[j]
    i = j - 1
    while i >= 1 and key < A[i]
      A[i + 1] = A[i]
      i = i - 1
    A[i + 1] = key
```

L'algoritmo è **corretto** per la sua [invariante](../../01/02/README.md#analisi-della-correttezza):
> L'array `A[1..j-1]` contiene gli elementi **ordinati** che originariamente erano in `A[1..j-1]`

e quando il `for` termina `j = A.length+1` quindi l'_invariante_ vale per `A[1..n+1-1]`, cioè l'intero array.

Nel caso **migliore** è $\Theta(n)$ e nel **peggiore** $\Theta(n^2)$ perchè il `for` itera $n - 1$ volte ed il numero di confronti è:
$$
\left(\sum_{j = 2}^n j - 1\right) = \sum_{k = 1}^{n - 1} k = \frac{n(n - 1)}{2} = \Theta(n^2)
$$

L'algoritmo è **stabile** ed **in loco**.
