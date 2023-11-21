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
	bool compreso(Node* u, int k) {
	  int somma;
	  return compreso_aux(u, k, somma);
	}

	bool compreso_aux(Node* u, int k, int& somma) {
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

- Verificare che il cammino da ogni nodo a qualsiasi foglia contenga lo stesso numero di nodi neri.

	```cpp
	int blackheight(Node* u) {
	  if (u == nullptr) {
	    return 0;
	  } else {
	    int b_sx = blackheight(u->left);
	    int b_dx = blackheight(u->right);

	    if (b_sx == -1 || b_dx == -1 ||
	      (b_sx != b_dx && u->left != nullptr && u->right != nullptr)) {
	      return -1;
	    }

	    int ris = u->left == nullptr ? b_dx : b_sx;
	    if (u->color == 'B') {
	      ris++;
	    }
	    return ris;
	  }
	}
	```

- Contare il numero di foglie di un albero generale.

	```cpp
	int n_foglie(Node* u) {
	  if (u == nullptr) {
	    return 0;
	  } else {
	    int dx = n_foglie(u->right_sibling);
	    if (u->left_child == nullptr) {
	      return dx + 1;
	    }
	    int sx = n_foglie(u->left_child);
	    return sx + dx;
	  }
	}
	```

- Verificare che un albero generale sia $k$-ario completo con $k \geq 2$.

	```cpp
	bool completo(Node* u, int k) {
	  int h;
	  return completo_aux(u, k, h);
	}

	bool completo_aux(Node* u, int k, int& h) {
	  if (u == nullptr) {
	    h = -1;
	    return true;
	  } else {
	    int hf = -1, grado = 0;
	    Node* f = u->left_child;
	    bool ris = true;
	    while (f != nullptr && ris) {
	      grado++;
	      int t;
	      ris = grado <= k && completo_aux(f, k, t);
	      if (hf == -1) {
	        hf = t;
	      } else {
	        ris = ris && t == hf;
	      }
	      f = f->right_sibling;
	    }
	    h = hf + 1;
	    return ris && (grado == k || grado == 0);
	  }
	}
	```
