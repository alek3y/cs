# Radix sort

Il **radix sort** effettua l'ordinamento **sulle cifre** dalla meno significativa alla più significativa, sapendo che:
> Ogni numero è composto da $d$ **cifre** di $k$ **valori**, su cui la $d$-esima è la **più significativa**
```c
radix(Array A, int d)
  for i = 1 to d
    sort(A, i)  // Ordinamento stabile in A sulla cifra i
```

L'algoritmo è **corretto** perchè, per induzione su $i$:
- **Caso base** per $i = 1$, l'unica colonna presente viene ordinata
- **Passo induttivo** assumendo che le colonne $1, ..., i-1$ siano ordinate, finisce con $1, ..., i$ in ordine:
	- Se due cifre in $i$ sono **diverse**, l'algoritmo le mette in posizione corretta
	- Se sono **uguali**, l'algoritmo **stabile** garantisce che siano nello stesso ordine per le cifre successive

La complessità è $\Theta(d \cdot T_{\mathrm{sort}}(n)) = \Theta(d(n + k))$ se `sort` è il [counting sort](../01/README.md) per le cifre.

## Suddivisione di una _word_

Dati $n$ numeri da $b$ bit **raggruppati** in $r$ bit con valori in $\{0, ..., 2^r-1\}$, l'algoritmo esegue in tempo:
$$
\Theta(d(n + k)) = \Theta\left(\frac{b}{r}(n + 2^r)\right)
$$

Si vuole trovare un $r \leq b$ che **minimizza** $\frac{b}{r} (n + 2^r)$:
- Se $b < \lfloor\log_2 n\rfloor$, allora $2^r \leq 2^b < 2^{\log_2 n} = n$ per cui si diminuisce $\frac{b}{r}$ con il più grande $r = b$
- Se $b \geq \lfloor\log_2 n\rfloor$, allora si diminuisce $\frac{b}{r}$ con il più grande $r$ fino a $2^r \leq n$, cioè $r = \lfloor\log_2 n\rfloor$

assicurando sempre una complessità **lineare** di $\Theta(n)$, se però nell'ultimo caso $b = O(\log n)$.
