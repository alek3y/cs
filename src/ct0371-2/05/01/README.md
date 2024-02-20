# Counting sort

Il **counting sort** consiste nel salvare in un il **numero di occorrenze** degli elementi, assumendo che:
> I **valori** degli elementi da ordinare vanno da $0$ a $k$

```c
counting(Array A, int k)
  B = []
  C = []  // Valori da 0 a k rappresentati da indici da 1 a k+1
  for i = 0 to k
    C[i+1] = 0
  for j = 1 to A.length
    C[A[j]+1]++
  for i = 1 to k
    C[i+1] += C[i]  // Trasforma i conteggi in indici per B
  for j = A.length down to 1
    B[C[A[j]+1]] = A[j]
    C[A[j]+1]--
  return B
```

Con le **somme prefisse** del _terzo ciclo_, su `C` in posizione $i$ c'è il numero di elementi $x$ in `A` con valori $x \leq i$.
Inoltre, `C[A[j]+1]` viene decrementato per poter posizionare **elementi uguali** in posizioni precedenti.

La complessità è $\Theta(2k) + \Theta(2n) = \Theta(n + k)$, e viene spesso usato quando $k = O(n)$.

Partendo dal fondo e decrementando i valori su `C`, ci si assicura che l'algoritmo sia **stabile**.
