# Taglio delle aste

Nel problema del **taglio delle aste** si vuole cercare il **massimo guadagno** $r_n$ ottenibile dalla vendita di pezzi ricavati dal taglio di un'asta lunga $n$, i cui prezzi $p_i$ dipendono dalla lunghezza $i$ venduta.

Per esempio se $n = 7$ e
$$
\Set{(i, p_i) | i \leq n} = \{(1, 1), (2, 5), (3, 8), (4, 9), (5, 10), (6, 17), (7, 17)\}
$$
conviene tagliare l'asta in pezzi da $2, 2, 3$ o $6, 1$ invece che $5, 2$ perchè si guadagna $18$ invece che $15$.

## Sottostruttura ottima

Un'asta lunga $n$ può essere **tagliata o meno** in ogni posizione $1 \leq i \leq n-1$, totalizzando
$$
\underbrace{2 \cdot 2 \cdots 2}_{n-1} = 2^{n-1}
$$
tagli e quindi portando la **complessità** a $\Theta(2^n)$.

Il **ricavo** per l'asta $r_n$ sarà quindi definito ricorsivamente come:
$$
r_n = \max(p_1 + r_{n-1}, p_2 + r_{n-2}, ..., p_n + r_0)
$$
dove $p_i$ è il ricavo della prima parte **non tagliata ulteriormente** e $r_{n-i}$ è il maggior ricavo sul resto dell'asta.

Per il problema si dice che valga la proprietà di **sottostruttura ottima** perchè la **soluzione ottima** cercata, in questo caso $r_n$, è esprimibile da combinazioni di _soluzione ottime_ di sottoproblemi.

## Ricorsione

L'algoritmo che se ne ricava
```c
cut_rod(p, n)
  if n == 0
    return 0
  else
    q = -1
    for i = 1 to n
      q = max(q, p[i] + cut_rod(p, n-i))
    return q
```
avrà complessità:
$$
T(n) = \begin{cases}
1 & \text{se } n = 0 \\
1 + \sum\limits_{i = 1}^n T(n - i) = 1 + \sum\limits_{j = 0}^{n-1} T(j) & \text{se } n > 0
\end{cases}
$$
che diventa $T(n) = 2^n$ per induzione su $n$:
- **Caso base**, per $n = 0$: $T(0) = 2^0 = 1$
- **Passo induttivo** assumendo che $T(n-1) = 2^{n-1}$:
$$
T(n) = 1 + \sum_{j = 0}^{n-1} T(j) = 1 + \sum_{j = 0}^{n-1-1} T(j) + T(n-1) = T(n-1) + T(n-1) = 2^n
$$

## Top-down

```c
cut_rod(p, n)
  r = {}
  for i = 0 to n
    r[i] = -1
  return cut_rod_aux(p, n, r)

cut_rod_aux(p, j, r)
  if r[j] < 0
    if j == 0
      r[j] = 0
    else
      q = -1
      for i = 1 to j
        q = max(q, p[i] + cut_rod_aux(p, j - i, r))
      r[j] = q
  return r[j]
```

La complessità si può ricavare come:
$$
T(n) = \underbrace{1}_{j = 0} + \sum_{j = 1}^n \sum_{i = 1}^j 1 = 1 + \sum_{j = 1}^n j = 1 + \frac{n(n + 1)}{2} = \Theta(n^2)
$$
perchè al diminuire di `j` i valori `r[j-i]` sono già stati calcolati.

## Bottom-up

```c
cut_rod(p, n)
  r = {}
  r[0] = 0
  for j = 1 to n
    q = -1
    for i = 1 to j
      q = max(q, p[i] + r[j - i])
    r[j] = q
  return r[n]
```

La complessità sarà anche in questo caso $T(n) = \Theta(n^2)$.

Volendo si può modificare per **salvare la posizione** del primo taglio da fare su un'asta lunga $j$:
```c
cut_rod(p, n)
  r = {}
  r[0] = 0
  for j = 1 to n
    q = -1
    for i = 1 to j
      if p[i] + r[j - i] > q
        q = p[i] + r[j - i]
        s[j] = i
    r[j] = q
  return r, s

print_cut_rod(p, n)
  r, s = cut_rod(p, n)
  while n > 0
    print(s[n])
    n = n - s[n]
```
