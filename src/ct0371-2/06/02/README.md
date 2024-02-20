# Indirizzamento aperto

Un modo alternativo per risolvere le _collisioni_ consiste nel memorizzare gli elementi con lo stesso _hash_ nella tabella insieme agli altri, per poi cercarli attraverso l'**ispezione** di `T[h(k)]`:
- Se la cella equivale a `k` la ricerca ha **successo**
- Se equivale a `NIL` la ricerca termina con **insuccesso**
- Se è diversa da `k` si trova il prossimo indice dall'**ordine di ispezione**, cioè il numero di _ispezioni_ fatte

Si vuole quindi che la _funzione hash_ rappresenti la posizione di $k$ in `T` dopo $i$ ispezioni fallite:
$$
h(k, i)\colon U \times \{0, ..., m-1\} \to \{0, ..., m-1\}
$$
con la condizione che per ogni $k$ la funzione assuma **tutti i valori**, così che **ogni cella** possa essere usata.

## Implementazione

In queste versioni delle operazioni, per semplicità, gli elementi di `T` sono solamente le chiavi stesse.

- **Inserimento**

	```c
	insert(T, k)
	  i = j = 0
	  found = false
	  while not found and i < m
	    j = h(k, i)
	    if T[j] == NIL or T[j] == DELETED
	      T[j] = k
	      found = true
	    else
	      i++
	  if found
	    return j
	  return NIL
	```

- **Ricerca**

	```c
	search(T, k)
	  i = j = 0
	  found = false
	  do
	    j = h(k, i)
	    if T[j] == k
	      found = true
	    else
	      i++
	  while not found and i < m and T[j] != NIL
	  if found
	    return j
	  return NIL
	```

- **Cancellazione**

	```c
	delete(T, k)
	  T[search(T, k)] = DELETED
	```

	Il motivo per cui si usa `DELETED` invece che `NIL` è che quest'ultimo serve ad indicare la **fine della catena** di ricerca, e quindi porterebbe alla perdita dei valori sulle $i$ successive.

	Lo svantaggio di questo metodo è che il tempo di ricerca **non dipende più** da $\alpha = \frac{n}{m}$.
