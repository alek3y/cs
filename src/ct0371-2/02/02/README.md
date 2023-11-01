# Puntatori

La _struttura dati_ è implementata salvando su ogni nodo un puntatore al successivo.

Un albero di esempio può essere:
```dot process
graph {
	node [shape=circle]

	0 [shape=point width=0]
	A -- L, B
	L -- E, R
	B -- 0 [style=invis]
	B -- O
}
```

## Singoli figli

Nel caso di $k = 2$, ogni nodo della rappresentazione contiene `(info, parent, left, right)`.

Ogni nodo avrà allocato spazio per $k$ figli, di conseguenza la **complessità spaziale** è $S(n) = \Theta(kn)$.

Nell'esempio, l'albero avrà puntatori rappresentati come:
```dot process
digraph {
	node [shape=box]
	edge [arrowsize=0.5]

	0 [shape=point width=0]
	A -> L, B
	L -> E, R
	B -> 0 [style=invis]
	B -> O

	{
		edge [constraint=false]
		L, B -> A
		E, R -> L
		O -> B
	}
}
```

### Implementazione

- **Padre**

	```c
	padre(Tree P, Node v) -> Node | NIL
	  return v.parent
	```
	per cui $T(n) = \Theta(1)$.

- **Figli**

	```c
	figli(Tree P, Node v) -> [Node]
	  l = []
	  if v.left != NIL
	    push(l, v.left)  // Assunto 𝛩(1)
	  if v.right != NIL
	    push(l, v.right)
	  return l
	```
	per cui $T(n) = \Theta(\mathrm{grado}(v)) = O(k)$.

## Figlio sinistro e fratello destro

In questa rappresentazione ogni nodo contiene `(info, parent, left_child, right_sibling)`.

Nel caso dell'esempio i puntatori saranno rappresentati come:
```dot process
digraph {
	node [shape=box]
	edge [arrowsize=0.5]

	{
		rank=same
		L -> B
	}
	{
		rank=same
		E -> R
	}

	0 [shape=point width=0]
	A -> L
	L -> E
	B -> 0 [style=invis]
	B -> O

	A -> B [style=invis]
	L -> R [style=invis]

	{
		edge [constraint=false]
		L, B -> A
		E, R -> L
		O -> B
	}
}
```

### Implementazione

- **Padre**

	```c
	padre(Tree P, Node v) -> Node | NIL
	  return v.parent
	```
	per cui $T(n) = \Theta(1)$.

- **Figli**

	```c
	figli(Tree P, Node v) -> [Node]
	  l = []
	  iter = v.left_child
	  while iter != NIL
	    push(l, iter)  // Assunto 𝛩(1)
	    iter = iter.right_sibling
	  return l
	```
	per cui $T(n) = \Theta(\mathrm{grado}(v)) = O(k)$.
