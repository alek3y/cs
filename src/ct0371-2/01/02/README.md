# Lista

Al posto dell'[array](../01/README.md) si può usare una collezione $L$ di $n$ record contenenti i campi `key`, `info`, `next` e `prev`.
Quando la collezione è **vuota** si avrà che `L.head = NIL`.

Anche in questo caso, la **complessità spaziale** sarà $S(n) = \Theta(n)$.

## Implementazione

- **Insert**

	```c
	insert(Dizionario L, Chiave K, Elem V)
	  p = {key = K, info = V}
	  p.next = L.head
	  p.prev = NIL

	  if L.head != NIL
	    L.head.prev = p
	  L.head = p
	```
	per cui potrebbe contenere chiavi **duplicate** ma $T(n) = \Theta(1)$.

- **Delete**

	```c
	delete(Dizionario L, Chiave K)
	  x = L.head
	  while x != NIL  // 𝛩(n)
	    if x.key == K
	      if x.next != NIL
	        x.next.prev = x.prev
	      if x.prev != NIL
	        x.prev.next = x.next
	      else
	        L.head = x.next

	      tmp = x
	      x = x.next
	      free(tmp)  // 𝛩(1)
	    else
	      x = x.next
	```
	per cui $T(n) = \Theta(n)$.

- **Search**

	```c
	search(Dizionario L, Key K)
	  x = L.head
	  while x != NIL and x.key != K
	    x = x.next

	  if x != NIL
	    return x.info
	  else
	    return NIL
	```
	per cui $T(n) = O(n)$.

### Analisi della correttezza

Si può dimostrare che il ciclo della `search` è **corretto**, dimostrando dell'**invariante** le proprietà:
- **Inizializzazione**: è vera precedentemente alla prima iterazione
- **Conservazione**: se è vera prima di un'iterazione lo è anche prima della successiva
- **Terminazione**: è vera alla fine del ciclo, e ci da quindi informazioni sullo stato dell'algoritmo

In questo caso, l'invariante del ciclo `while` $\text{INV}_i$ è espressa come:
> All'inizio dell'$i$-esima iterazione, gli elementi da `L.head` all'$i$-esimo `x` _escluso_ hanno chiavi diverse da `K`

La dimostrazione consisterà nel provare che $\text{INV}_i$ è vero per ogni $i = 0, ..., n$:

1. **Inizializzazione**:

	Per verificare $\text{INV}_0$ basta notare come `x = L.head` prima di entrare nel ciclo, e di conseguenza non ci sono elementi tra `L.head` e `x` con `x` **escluso** che hanno chiave uguale a `K`.

2. **Conservazione**:

	Per ipotesi sappiamo che $\text{INV}_i$ è vera e lo è anche la condizione `x != NIL and x.key != K`.

	Va dimostrato che $\text{INV}_{i+1}$ è vera, ovvero che da `L.head` a `x` **incluso** non sia presente la chiave `K`.
	Questo risulta evidente perchè `x.key != K` è dato vero nella condizione del ciclo.

3. **Terminazione**:

	In questa situazione sappiamo che la condizione `x != NIL and x.key != K` è falsa.
	Di conseguenza si ha che `x == NIL or x.key == K`, per cui in entrambi i casi $\text{INV}_n$ resta vera.

	Quindi se `x.key == K`, allora `x` è la **prima occorrenza** della chiave `K`.
