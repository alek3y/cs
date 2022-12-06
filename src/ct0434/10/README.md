# Calcolo combinatorio

## Permutazioni

Il numero di **permutazioni semplici** (o _senza ripetizioni_, per cui ogni elemento non si ripete) di $n$ oggetti è la quantità di modi in cui si possono disporre:
$$P_n = n!$$

Per esempio, avendo gli oggetti $\{A, B, C, D\}$, le permutazioni sono $P_4 = 4 \cdot 3 \cdot 2 \cdot 1$.
Questo perchè la prima volta si hanno $4$ oggetti da scegliere, la seconda $3$, la terza $2$ e l'ultima ne rimane solo uno.

## Disposizioni

Le disposizioni semplici di $n$ elementi **presi $k$ alla volta** (o _a $k$ a $k$_) sono tutte le possibili sequenze **ordinate** (i.e. due sequenze possono avere gli stessi elementi ma il loro ordine deve essere diverso) di $k$ elementi:
$$D_{n, k} = \frac{n!}{(n-k)!} = \binom{n}{k}k!$$

Per esempio, per determinare quante combinazioni dei primi $3$ vincitori ci sono in una gara da $9$ partecipanti, basta fare $9 \cdot 8 \cdot 7 = \frac{9!}{(9 - 3)!}$.

### Proprietà

- $D_{n, m} = 0$ se $m > n$
- $D_{n, 0} = 1$
- $D_{n, 1} = n$
- $D_{n, n} = n!$

## Combinazioni

Il numero di **combinazioni semplici** di $n$ elementi presi $k$ alla volta è il numero di tutti i gruppi che si possono costruire lunghi $k$, i cui elementi non si ripetono:
$$C_{n, k} = \binom{n}{k} = \frac{D_{n, k}}{P_k} = \frac{n!}{k!(n-k)!}$$
dove $\binom{n}{k}$ è chiamato il **coefficiente binomiale**.

### Proprietà

- $\binom{n}{k} = 0$ se $k > n$
- $\binom{n}{n} = \binom{n}{0} = 1$
- $\binom{n}{1} = n$
- $\binom{n}{k} = \binom{n}{n - k}$

## Multiinsieme

Un **multiinsieme** è un insieme i cui elementi possono ripetersi, per esempio:
$$\{3 \cdot A, 1 \cdot B, 2 \cdot C\} = \{A, A, A, B, C, C\}$$

Il numero di **permutazioni** di un multiinsieme $A = \{n_1 \cdot a_1, n_2 \cdot a_2, ..., n_k \cdot a_k\}$ di $n = |A|$ elementi avente $k$ oggetti unici è espresso come:
$$
P_n' = \frac{n!}{n_1! \cdot n_2! \cdot ... \cdot n_k!} =
\binom{n}{
\begin{matrix}
n_1 & n_2 & ... & n_k
\end{matrix}
}
$$
chiamato **coefficiente multinomiale**.

Per esempio, il numero di permutazioni della parola $\text{SASSO}$ sono $\frac{5!}{3!} = 20$, mentre della parola $\text{MAMMA}$ sono $\frac{5!}{3!2!}$.
