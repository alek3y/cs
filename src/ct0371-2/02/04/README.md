# Decomposizione

In generale, le operazioni sugli alberi si possono ridurre a
```c
decomponi(Node u)
  if u == NIL
    ...        // Effettua l'operazione sull'albero vuoto
  else if ...
    ...        // Opera sugli altri casi base se presenti
  else
    risultato_sx = decomponi(u.left)
    risultato_dx = decomponi(u.right)
    return ricombina(risultato_sx, risultato_dx)
```
con $T(n) = T(k) + T(n - k - 1) + d = \Theta(n)$, se _casi base_ e `ricombina` sono $\Theta(1)$, come [dimostrato](../03/README.md#depth-first-search).

## Esempi

- Un albero è _k-compreso_ per un $k \in \mathbb{N}$ se la somma dei nodi in ogni sottoalbero è compresa tra $-k$ e $k$.

	```cpp
	bool compreso(Node *u, int k) {
	  int somma;
	  return compreso_aux(u, k, somma);
	}

	bool compreso_aux(Node *u, int k, int &somma) {
	  if (u == nullptr) {
	    somma = 0;
	    return true;
	  } else {
	    int somma_sx, somma_dx;
	    bool compreso_sx = compreso_aux(u->left, k, somma_sx);
	    bool compreso_dx = compreso_aux(u->right, k, somma_dx);
	    somma = somma_sx + somma_dx + u->info;
	    return compreso_sx && compreso_dx && somma >= -k && somma <= k;
	  }
	}
	```
