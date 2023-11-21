# Di ricerca

Un albero si dice **binario di ricerca** se $k = 2$ e soddisfa la **proprietà di ricerca**:
> Ogni `y` nel sottoalbero **sinistro** di `x` ha `y.info <= x.info` e ogni `y` nel **destro** `y.info >= x.info`

che genera una stampa in ordine crescente con la visita [in-order](../03/README.md#depth-first-search).

## Operazioni

- **Ricerca**

	```c
	search(Node x, Elem k) -> Node | NIL
	  if x == NIL or x.info == k
	    return x
	  else
	    if x.Key > k
	      return tree_search(x.left, k)
	    else
	      return tree_search(x.right, k)
	```
	oppure iterativamente:
	```c
	search(Node x, Elem k) -> Node | NIL
	  while x != NIL and x.info != k
	    if k < x.info
	      x = x.left
	    else
	      x = x.right
	  return x
	```
	con $T(n) = O(h)$ perchè nel caso peggiore fa il cammino più lungo dell'albero, cioè l'altezza $h$.

- **Massimo** e **minimo**

	```c
	maximum(Node x) -> Node
	  while x.right != NIL
	    x = x.right
	  return x

	minimum(Node x) -> Node
	  while x.left != NIL
	    x = x.left
	  return x
	```
	anche in questo caso entrambi con $T(n) = O(h)$.

- **Successore** o **predecessore**

	Il **successore** del nodo `x` in un _albero di ricerca_ sarà il **minimo del sottoalbero destro** se ne possiede uno, altrimenti sarà l'**antenato** di `x` che lo contiene nel suo sottoalbero sinistro:

	```c
	successor(Node x)
	  if x.right != NIL
	    return minimum(x.right)
	  else
	    y = x.parent
	    while y != NIL and y.right == x
	      x = y
	      y = y.parent
	    return y
	```
	con $T(n) = O(h)$ perchè sia `minimum` che il cammino per la radice sono al più $O(h)$.

	Il **predecessore** invece, è la versione specchiata del _successore_:
	```c
	predecessor(Node x)
	  if x.left != NIL
	    return maximum(x.left)
	  else
	    y = x.parent
	    while y != NIL and y.left == x
	      x = y
	      y = y.parent
	    return y
	```
	sempre con $T(n) = O(h)$.

- **Inserimento**

	```c
	insert(Tree t, Node z)
	  y = NIL
	  x = T.root
	  while x != NIL  // Al più h volte
	    y = x
	    if z.info < x.info
	      x = x.left
	    else
	      x = x.right
	  z.parent = y
	  if y == NIL   // Anche T.root è NIL
	    T.root = z
	  else
	    if z.info < y.info
	      y.left = z
	    else
	      y.right = z
	```
	con $T(n) = O(h)$.
