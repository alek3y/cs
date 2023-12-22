# Heap sort

L'**heap sort** sfrutta la proprietà dei [max heap](../../02/06/README.md) per cui in prima posizione è presente l'**elemento più grande**.

```c
heapsort(Array A)
	build_max_heap(A)
	for i = A.length down to 2
		swap(A[1], A[i])
		A.heap_size--
		max_heapify(A, 1)
```

L'algoritmo è **corretto** per l'[invariante](../../01/02/README.md#analisi-della-correttezza) degli elementi:
> Su `A[1..i]` c'è un _max heap_ con i più piccoli elementi di `A`, e su `A[i+1..n]` sono i più grandi **ordinati**

e quando il `for` termina `i = 1` quindi l'_invariante_ vale con `A[2..n]` ordinati e `A[1]` il più piccolo.

La complessità di `heapsort` è $O(n \log n)$ perchè `max_heapify` da $O(\log n)$ viene chiamato $n - 1$ volte.

L'algoritmo è **in loco**, ma non è _stabile_.
