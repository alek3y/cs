# Concatenamento

Un modo per risolvere le _collisioni_ consiste nello sfruttare le **linked list** per metterle su ogni cella, per poi inserirci tutti gli elementi che hanno chiavi diverse ma **hash uguali**.

In questo caso, le **implementazioni** sono:
- `search(T, x)` in tempo proporzionale alla lunghezza della lista su `T[h(x.key)]`
- `insert(T, x)` in $\Theta(1)$ inserendo `x` in testa a `T[h(x.key)]`
- `delete(T, x)` in $\Theta(1)$ se `x.prev` è presente, altrimenti come `search`

## Tempo di ricerca

Nel **caso peggiore** tutti gli $n = |K|$ elementi sono nella **stessa cella**, e quindi il tempo di `search` è $O(n)$, mentre nel **caso medio** il tempo dipende dalla **distribuzione** delle chiavi fra le $m$ celle.

Una _distribuzione_ **ideale** di $h$ è data dall'**hashing uniforme** e indipendente, per cui:
$$
P(h = i) = \frac{1}{m},\ \forall i \in \{0, ..., m-1\}
$$
da cui si ricava il **fattore di carico** $\alpha = \frac{n}{m}$.

In questo caso, la **lunghezza media** di una lista $n_i$ su `T[i]` è:
$$
\frac{n_0 + n_1 + ... + n_{m-1}}{m} = \frac{n}{m} = \alpha
$$
Di conseguenza, la ricerca **senza successo** avviene in $\Theta(1 + \alpha)$, perchè $h$ è almeno $\Theta(1)$ ma $\exists n : \alpha < 1$. La ricerca **con successo** invece, avviene in $\Theta\left(1 + \frac{\alpha}{2}\right) = \Theta(1 + \alpha)$, perchè in media arriva fino a metà lista.

Per cui, finchè $n = O(m)$ si ha $\alpha = O(1)$ perchè $h$ è _uniforme_, altrimenti quando $n$ cresce `T` va **riallocato**.

## Funzione hash

Quando $U \nsubseteq \mathbb{N}$ le chiavi vanno **trasformate**, per esempio attraverso la **notazione posizionale**:
$$
\text{CLRS} = 128^3 \cdot \text{C} + 128^2 \cdot \text{L} + 128^1 \cdot \text{R} + 128^0 \cdot \text{S}
$$
dove $128 = 2^7$ è il numero di valori rappresentabili in $7$ bit per carattere nella codifica **ASCII**, i.e. base $128$.

In generale, per diminuire le collisioni si può usare la tecnica di **hashing universale** per cui la funzione $h$ viene scelta casualmente da una famiglia $H$ di funzioni _uniformi_ all'inizio del programma.

### Divisione

$$
h(k) = k \bmod m
$$
risulta essere **semplice**, ma richiede un'accurata scelta di $m$:
- $m \neq 2^p$ perchè altrimenti $h(k)$ considera solo i $p$ bit meno significativi, scartando gli altri
- $m \neq 2^p-1$ se $k$ è una stringa in base $2^p$, perchè altrimenti le permutazioni di $k$ danno lo stesso $h(k)$
- $m = q$ dove $q$ è un **numero primo** distante da potenze di $2$ e $10$

### Moltiplicazione

$$
h(k) = \lfloor (k \cdot A \bmod 1) \cdot m \rfloor
$$
con $U \subseteq \mathbb{N}$, fissando $A \in (0, 1)$ e ottenendo da $k \cdot A \bmod 1$ un valore in $[0, 1)$ da trasformare in $[0, m)$.

In questo modo $m$ non è più critico e la funzione è **ottimale** per la maggior parte degli $A$, tra cui $A = \frac{\sqrt{5} - 1}{2}$.

Se $k$ è una _word_ lunga $w$, la funzione si può **semplificare** scegliendo una $q$ tra le $2^w$ _word_ e ponendo:
$$
m = 2^p\ \land\ A = \frac{q}{2^w}
$$
così che $h(k)$ restituisca i $p$ bit più significativi di $k \cdot A \bmod 1$, ovvero `((k*q) & ((1 << w)-1)) >> (w-p)`.
