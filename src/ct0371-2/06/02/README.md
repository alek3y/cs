# Indirizzamento aperto

Un modo alternativo per risolvere le _collisioni_ consiste nel memorizzare gli elementi con lo stesso _hash_ nella tabella **insieme agli altri**, per poi cercarli attraverso l'**ispezione** di `T[h(k)]`:
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

## Metodi di ispezione

Come per il [concatenamento](../01/README.md#tempo-di-ricerca), una funzione $h$ ideale rispetterebbe le proprietà dell'**hashing uniforme** così che, dato un $k$, ogni $h(k, i)$ è distribuito uniformemente sulle $m$ celle.

Dato che è difficile rispettarle, vengono adottate delle approssimazioni.

### Ispezione lineare

$$
h(k, i) = (h'(k) + i) \bmod m
$$
dove $h'\colon U \to \{0, 1, ..., m-1\}$ è detta funzione **hash ausiliaria**.

Questo metodo è **semplice**, ma genera la **stessa sequenza** di ispezioni per le $k$ diverse con lo stesso $h'(k)$.

### Ispezione quadratica

$$
h(k, i) = (h'(k) + c_1 \cdot i + c_2 \cdot i^2) \bmod m
$$
dove $h'$ è l'_hash ausiliaria_ e $c_1, c_2 \in \{1, ..., m-1\}$ sono costanti, con buoni valori $c_1 = c_2 = \frac{1}{2}$ e $m = 2^p$.

Anche in questo caso genera la **stessa sequenza** di ispezioni per $k$ diverse con $h'(k)$ uguali.

### Doppio hashing

$$
h(k, i) = (h_1(k) + i \cdot h_2(k)) \bmod m
$$
dove $h_{1,2}$ sono _hash ausiliarie_, di cui $h_1$ marca la cella di partenza mentre $h_2$ definisce lo step delle ispezioni.

Questo metodo genera **sequenze diverse** di ispezione, dato che dipendono da $h_2 \neq h_1$.

Per generare sequenze su tutti i valori della tabella, $h_2(k)$ dev'essere [relativamente primo](../../../ct0434/08/README.md#massimo-comun-divisore) con $m$:
- Si usa $m = 2^p$ **pari** e si definisce $h_2(k)$ come sempre **dispari**, e.g. $h_2(k) = 2 \cdot h'(k) + 1$
- Si usa $m$ **primo** e $h_2(k)$ **minore** di $m$, e.g. $h_1(k) = k \bmod m$, $h_2(k) = 1 + (k \bmod m')$ per $m' < m$

## Tempo di ricerca

Al contrario del [concatenamento](../01/README.md#tempo-di-ricerca), l'indice di prestazione $\alpha = \frac{n}{m} \in [0, 1]$ perchè $n = |K| \leq m$.

Se $\alpha < 1$ esiste almeno una cella vuota su cui la **ricerca senza successo** si può fermare, quindi:
- $P(i = 0) = 1$ perchè la prima ispezione è sempre effettuata
- $P(i = 1) = \frac{n}{m}$ ovvero la probabilità che la cella su $i = 0$ sia occupata
- $P(i = 2) = \frac{n}{m} \cdot \frac{n - 1}{m - 1}$ perchè anche la cella su $i = 1$ sia occupata
- $P(i = 3) = \frac{n}{m} \cdot \frac{n - 1}{m - 1} \cdot \frac{n - 2}{m - 2}$ perchè anche la cella su $i = 2$ sia occupata

Di conseguenza il [valore atteso](../../../ct0111/03/README.md#valore-atteso) di $i$, ovvero il **numero medio di ispezioni**, è al massimo[^1]:
$$
\begin{split}
E(i) &= 0 \cdot P(i = 0) + 1 \cdot P(i = 1) + ... \\
&\leq \sum_{k = 0}^\infty \alpha^k = \frac{1}{1 - \alpha}
\end{split}
$$
e lo stesso vale per l'**inserimento**, dato che cerca una cella vuota.

Nella **ricerca con successo** invece, il _numero medio di ispezioni_ è $\frac{1}{\alpha}\log\frac{1}{1 - \alpha}$.

[^1]: CLRS, Introduction to Algorithms (4th ed.), pp. 298-299
