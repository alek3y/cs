# Array

La _struttura_ può essere un array `A` di $n$ elementi ordinati dal campo `key` e contenenti anche il campo `info`.

La **complessità spaziale** sarà quindi $S(n) = \Theta(n)$.

## Implementazione

- **Search**

	Prima di `search` sarà necessario implementare `search_index` che fa la **ricerca binaria**:
	```c
	search_index(Dizionario A, Chiave K, int p, int r) -> int
	  if p > r
	    return -1             // 𝛩(1)
	  else
	    med = floor((p+r)/2)  // 𝛩(1)
	    if A[med].key == k
	      return med          // 𝛩(1)
	    else
	      if A[med].key > k
	        return search_index(A, K, p, med-1)  // T(n/2)
	      else
	        return search_index(A, K, med+1, r)  // T(n/2)
	```
	che per l'`else` risulterà essere una [ricorrenza](../../../ct0371-1/01/03/README.md):
	$$
	T(n) = \begin{cases}
	\Theta(1) & \text{se } n = 0 \\
	T\left(\frac{n}{2}\right) + \Theta(1) & \text{se } n > 0
	\end{cases}
	$$
	da cui $T(n) = \Theta(\log n)$ per il [teorema master](../../../ct0371-1/01/03/README.md#teorema-master).

	Di conseguenza per `search` basterà:
	```c
	search(Dizionario A, Chiave K) -> Elem | NIL
	  i = search_index(A, K, 1, A.length)  // 𝛩(log n)
	  if i == -1
	    return NIL        // 𝛩(1)
	  else
	    return A[i].info  // 𝛩(1)
	```
	da cui $T(n) = \Theta(\log n) + \Theta(1)$.

- **Insert**

	```c
	insert(Dizionario A, Chiave K, Elem V)
	  i = 1
	  while i <= A.length and A[i].key < K  // d*i volte, con massimo i = A.length
	   i++

	  if i <= A.length and A[i].key == K
	    A[i].info = V
	  else  // Più costoso del corpo dell'if
	    reallocate(A, A.length + 1)   // O(n)
	    A.length++                    // 𝛩(1)

	    for j = A.length down to i+1  // (n+1)-(i+1)+1 = n-i+1 volte, dove n+1 per A.length++
	      A[j] = A[j-1]               // d*1

	    A[i].key = K                  // 𝛩(1)
	    A[i].info = V                 // 𝛩(1)
	```
	dove $O(n)$ per la `reallocate` perchè al massimo copierà $n$ elementi in un'altra area di memoria.

	Di conseguenza, $T(n) = \Theta(1) + \cancel{d \cdot i} + O(n) + (n \cancel{-i} + 1) \cdot d = O(n)$.

- **Delete**

	```c
	delete(Dizionario A, Chiave K)
	  i = search_index(A, K, 1, A.length)  // 𝛩(log n)
	  for j = i to A.length-1              // (n-1)-i+1 = n-1 volte
	    A[j] = A[j+1]                      // d*1

	  reallocate(A, A.length-1)            // O(n)
	  A.length--                           // 𝛩(1)
	```
	per cui $T(n) = \Theta(\log n) + (\underbrace{n - i}_{\leq n}) \cdot d + O(n) = O(n)$.

- **Reallocate**

	La `reallocate` può **raddoppiare** e **dimezzare** la capacità $h$ di un array con $n$ elementi, secondo:
	$$
	n \leq h < 4n,\ \forall n > 0 \\
	\Updownarrow \\
	n \leq h\ \land\ n > \frac{h}{4} \\
	\Updownarrow \\
	\frac{h}{4} < n \leq h
	$$
	iniziando con $n = 0$ e $h = 1$, se $n > h$ allora $h$ viene raddoppiato e se $n \leq \frac{h}{4}$ viene dimezzato.

	Di conseguenza $S(n) = \Theta(h) = \Theta(n)$ perchè $h < 4n$.

### Analisi ammortizzata

Tramite l'**analisi ammortizzata** è possibile analizzare il **costo medio** di $n$ operazioni.

Per esempio, si vogliono fare $n$ inserimenti su un array con $0$ elementi di _capacità_ $1$, allora:
$$
C_i = \begin{cases}
i & \text{se } \exists k : i = 2^k + 1 \\
1 & \text{altrimenti}
\end{cases}
$$
con $C_i$ che rappresenta il **costo** dell'$i$-esimo insert, che vale $i$ quando la `reallocate` raddoppia la _capacità_.

Il **costo** per $n$ inserimenti risulterà quindi essere:
$$
\begin{split}
C(n) = \sum_{i = 1}^n C_i &\leq n + \sum_{i = 0}^{\lfloor\log_2 n\rfloor} 2^i \\
&\leq n + \frac{2^{\lfloor\log_2 n\rfloor + 1} - 1}{2 - 1} \\
&\leq 3n
\end{split}
$$
dove $\sum\limits_{i = 0}^{\lfloor\log_2 n\rfloor} 2^i$ rappresenta la somma di tutti gli $i$-esimi inserimenti che hanno costo $i$.

Di conseguenza il **costo ammortizzato** sarà $\frac{C(n)}{n} \leq \frac{3n}{n} = 3 = \Theta(1)$.
