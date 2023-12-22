# Code di priorità

In una **coda di priorità** ogni elemento è associato ad un **peso**, che determina l'ordine di estrazione.

Se l'elemento estratto ha la **priorità più alta** allora è a **massima priorità**, altrimenti è a **minima priorità**.

## Operazioni

Una realizzazione delle _code a massima priorità_ sfrutta gli [heap](../02/06/README.md), portando ad operazioni al più da $O(\log n)$.

- **Massimo**

	```c
	maximum(Heap A) -> Elem
	  return A[i]
	```
	quindi $T(n) = \Theta(1)$.

- **Estrazione del massimo**

	```c
	extract_max(Heap A) -> Elem
	  max = A[i]
	  A[1] = A[A.heap_size]  // Mantengo le foglie a sinistra scegliendo l'ultima
	  A.heap_size--
	  max_heapify(A, 1)
	  return max
	```
	per cui $T(n) = O(\log n)$ per la `max_heapify`.

- **Incremento di priorità**

	```c
	increase(Heap A, Node i, Elem k)
	  A[i] = k
	  while i > 1 and A[parent(i)] < A[i]  // Al massimo h volte se i è foglia
	    swap(A[i], A[parent(i)])
	    i = parent(i)
	```
	con $T(n) = O(h) = O(\log n)$, che è **corretto** per l'[invariante](../01/02/README.md#analisi-della-correttezza) del `while`:
	> Su `A` c'è un _max heap_ con la possibile eccezione che `A[i]` è più grande di `A[parent(i)]`

	per cui al termine si presenta una situazione in cui:
	- `i = 1`, allora l'eccezione sarebbe su `A[1]` che non ha padre
	- `A[parent(i)] >= A[i]`, allora non c'è alcuna eccezione

	quindi in entrambi i casi si può affermare che `A` è _max heap_.

- **Inserimento**

	```c
	insert(Heap A, Elem k)
	  A.heap_size++
	  A[A.heap_size] = -Infinity  // Mantiene l'heap corretto per la funzione increase
	  increase(A, A.heap_size, k)
	```
	con $T(n) = O(\log n)$ per `increase`.
