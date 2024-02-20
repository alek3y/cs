# Di ricerca

Un albero si dice **binario di ricerca** se $k = 2$ e soddisfa la **proprietà di ricerca**:
> Ogni `y` nel sottoalbero **sinistro** di `x` ha `y.info <= x.info` e ogni `y` nel **destro** `y.info >= x.info`

che genera una stampa in ordine crescente con la visita [in-order](../03/README.md#depth-first-search).

## Operazioni

Dato che la maggior parte delle operazioni sono $O(h)$ dove $h$ è l'**altezza**, mantenere l'albero [bilanciato](../README.md#alberi-k-ari) permette di ottenere al meglio $T(n) = O(\log n)$ invece che $O(n)$ nel caso peggiore.

- **Ricerca**

	```c
	search(Node x, Elem k) -> Node | NIL
	  if x == NIL or x.info == k
	    return x
	  else
	    if x.info > k
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
	insert(Tree T, Node z)
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

- **Cancellazione**

	Si può dimostrare che il _successore_ di un nodo $x$ **non ha figlio sinistro**, dato che nella visita [in-order](../03/README.md#depth-first-search) precedono i nodi del sottoalbero sinistro di $x$, poi $x$ e poi i nodi del sottoalbero destro.

	Per _assurdo_ se $s$, i.e. il _successore_ di $x$, avesse un _figlio sinistro_ $y$, nella visita si avrebbe che $x \leadsto y \leadsto s$ che è una _contraddizione_ dato che allora sarebbe $y$ il _successore_ di $x$ e non $s$.

	Questo permetterà di formare un algoritmo **corretto** per la _cancellazione_, che richiederà però:
	```c
	transplant(Tree T, Node u, Node v)  // Rimpiazza u con v
	  if u.parent == NIL
	    T.root = v
	  else
	    if u.parent.left == u
	      u.parent.left = v
	    else
	      u.parent.right = v

	  if v != NIL
	    v.parent = u.parent
	```
	con $T(n) = \Theta(1)$, con cui si ottiene:
	```c
	delete(Tree T, Node z)
	  if z.left == NIL
	    transplant(T, z, z.right)  // Rimpiazza z con z.right dato che z.left è vuoto
	  else if z.right == NIL
	    transplant(T, z, z.left)
	  else

	    // Trova il successore in tempo O(h) per metterlo al posto di z
	    y = minimum(z.right)

	    // Se z non è il padre di y si rimpiazza y con y.right così che z.right
	    // sia di ricerca e si possa spostare y al posto di z
	    if y.parent != z
	      transplant(T, y, y.right)
	      y.right = z.right
	      z.right.parent = y

	    // Si sostituisce z con y anche se z è padre di y perchè essendo successore
	    // y non ha y.left e quindi come figli di y si mettono z.left e z.right
	    transplant(T, z, y)
	    y.left = z.left
	    z.left.parent = y
	```
	che avrà complessità $T(n) = O(h)$ per via di `minimum`.

- **Costruzione**

	```c
	build(Array A) -> Tree
	  Tree T = {root = NIL}
	  for i = 1 to A.length  // n volte
	    Node u = {key = A[i], left = NIL, right = NIL}
	    insert(T, u)  // O(h) con h = i nel caso peggiore in cui A è ordinato
	  return T
	```
	con $T(n) = \sum\limits_{i = 1}^{n} O(i) = O\left(\sum\limits_{i = 1}^{n} i\right) = O\left(\frac{n(n + 1)}{2}\right) = O(n^2)$.

	Se sappiamo però che `A` è dato **ordinato**, allora si può **ottimizzare**:
	```c
	build_ord(Array A) -> Tree
	  Tree T = {root = build_ord_aux(A, 1, A.length, NIL)}
	  return T

	build_ord_aux(Array A, int inf, int sup, Node parent) -> Node
	  if inf > sup
	    return NIL
	  else
	    med = floor((inf + sup)/2)
	    Node r = {key = A[med], parent = parent}
	    r.left = build_ord_aux(A, inf, med-1, r)
	    r.right = build_ord_aux(A, med+1, sup, r)
	    return r
	```
	con $T(n) = \Theta(n)$ perchè $T(n) = 2T\left(\frac{n}{2}\right) + d$ se $n > 0$ per il [teorema master](../../../ct0371-1/01/03/README.md#teorema-master).

## Esempi

- Restituire il numero massimo di ripetizioni in un albero binario di ricerca in $\Theta(n)$.

	```cpp
	int massimo_ripetizioni(Tree t) {
	  if (t.root == nullptr) {
	    return 0;
	  }

	  int max = 1, count = 1;
	  PNode iter = minimum(t.root);  // O(h)
	  int value = iter->key;
	  iter = successor(iter);
	  while (iter != nullptr) {
	    if (iter->key == value) {
	      count++;
	    } else {
	      if (count > max) {
	        max = count;
	      }
	      count = 1;
	      value = iter->key;
	    }
	    iter = successor(iter);  // O(h)
	  }

	  if (count > max) {
	    max = count;
	  }
	  return max;
	}
	```
	con $T(n) = \Theta(n)$ perchè chiamando `minimum` e poi $n$ volte `successor` si ha una visita in-order.

- Verificare che un albero sia di ricerca.

	```cpp
	bool is_bst(PNode r) {
	  if (r == nullptr) {
	    return true;
	  }
	  int min, max;
	  return is_bst_aux(r, min, max);
	}

	bool is_bst_aux(PNode u, int& min, int& max) {
	  int min_sx, min_dx, max_sx, max_dx;
	  bool is_sx, is_dx;
	  if (u->left == nullptr) {
	    is_sx = true;
	    min_sx = max_sx = u->key;
	  } else {
	    is_sx = is_bst_aux(u->left, min_sx, max_sx);
	  }
	  if (u->right == nullptr) {
	    is_dx = true;
	    min_dx = max_dx = u->key;
	  } else {
	    is_dx = is_bst_aux(u->right, min_dx, max_dx);
	  }

	  min = min_sx;
	  max = max_dx;
	  return is_sx && is_dx && max_sx <= u->key && min_dx >= u->key;
	}
	```
	con $T(n) = \Theta(n)$ per la [decomposizione](../04/README.md).

- Verificare che dato un albero $T$ _binario di ricerca_ $\forall k, k \in T \land k + 2 \in T \Rightarrow k+1 \in T$.

	```cpp
	bool check(Tree t) {
	  if (t.root == nullptr) {
	    return true;
	  }

	  PNode iter = minimum(t.root), succ;
	  bool valid = true;
	  while (iter != nullptr && valid) {
	    succ = successor(iter);
	    if (succ != nullptr && succ->key >= iter->key+2) {
	      valid = false;
	    } else {
	      iter = succ;
	    }
	  }
	  return valid;
	}
	```
	con $T(n) = O(n)$ dato che la visita in-order può terminare in anticipo se l'albero non è valido.
