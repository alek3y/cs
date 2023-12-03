# Distribuzioni discrete

## Uniforme

$$X \sim \mathrm{U}(\{x_1, ..., x_n\})$$

- **Probabilità**: $P_X(x) = \frac{1}{n}$
- **Media**: $E(X) = \frac{x_1 + x_n}{2}$
- **Varianza**: $\mathrm{Var}(X) = \frac{n^2 - 1}{12}$

Per esempio, se $X$ descrive il lancio di un dado ogni elemento avrà probabilità $P_X(x) = \frac{1}{6}$.

## Ipergeometrica

$$X \sim \mathrm{Ig}(N, m, n)$$

- **Probabilità**: $P_X(k) = \frac{{m \choose k} {N-m \choose n-k}}{N \choose n}$
- **Media**: $E(X) = n\frac{m}{N}$
- **Varianza**: $\mathrm{Var}(X) = n\frac{m}{N}\frac{N - m}{N}\frac{N - n}{N - 1}$
- **Funzione**,
	- per $P(X = k)$: `dhyper(x=k, m=m, n=N-m, k=n)`
	- per $P(X \leq k)$: `phyper(q=k, m=m, n=N-m, k=n)`

Modella l'[estrazione](../../01/README.md#estrazione) di $n$ palline **senza reinserimento** da un'urna di $N$ palline dei quali $m$ sono successi.

Per esempio, con $N = 12$ programmi di cui $m = 5$ vanno aggiornati, la probabilità che su $n = 4$ vanno aggiornati almeno $2$ è $P(X \geq 2) = 1 - P(X \leq 1) = 1 - P_X(0) - P_X(1)$.

## Di Bernoulli

$$X \sim \mathrm{Ber}(p)$$

- **Probabilità**: $P_X(x) = \begin{cases}1 - p & \text{se } x = 0 \\ p & \text{se } x = 1 \\ 0 & \text{altrimenti}\end{cases} = p^x (1 - p)^{1-x}\mathbf{1}_{\{0, 1\}}(x)$
- **Media**: $E(X) = p$
- **Varianza**: $\mathrm{Var}(X) = p(1 - p)$
- **Funzione**,
	- per $P(X = k)$: `dbinom(x=k, size=1, prob=p)`
	- per $P(X \leq k)$: `pbinom(q=k, size=1, prob=p)`

## Binomiale

$$X \sim \mathrm{Bin}(n, p)$$

- **Probabilità**: $P_X(k) = {n \choose k} p^k (1 - p)^{n - k}$
- **Media**: $E(X) = np$
- **Varianza**: $\mathrm{Var}(X) = np(1 - p)$
- **Funzione**,
	- per $P(X = k)$: `dbinom(x=k, size=n, prob=p)`
	- per $P(X \leq k)$: `pbinom(q=k, size=n, prob=p)`

Modella l'[estrazione](../../01/README.md#estrazione) **con reinserimento** di $n$ prove ognuna con probabilità $p$ di successo.

Se gli elementi $N \to \infty$, anche _senza reinserimento_, si usa la _binomiale_ perchè ci si aspetta che $p$ non cambi.
Quando $n = 1$ invece, rappresenta la distribuzione di _Bernoulli_, per cui $\mathrm{Ber}(p) = \mathrm{Bin}(1, p)$.

Per esempio, in una ricerca su $N \to \infty$ siti, ogni sito ha $p = 0.2$ di contenere la parola chiave, per cui dei primi $n = 10$ siti la probabilità che $k \leq 5$ la contengano è $P(X \leq 5) = P_X(0) + ... + P_X(5) \approx 0.9936$.

## Di Poisson

$$X \sim \mathrm{Po}(\lambda)$$

- **Probabilità**: $P_X(k) = \frac{\lambda^k}{k!}e^{-\lambda}$
- **Media**: $E(X) = \lambda$
- **Varianza**: $\mathrm{Var}(X) = \lambda$
- **Funzione**,
	- per $P(X = k)$: `dpois(x=k, lambda=𝜆)`
	- per $P(X \leq k)$: `ppois(q=k, lambda=𝜆)`
	- per $P(X \geq k)$: `1-ppois(q=k-1, lambda=𝜆)`

Modella $X$ sul **conteggio** di un certo evento favorevole secondo una media $\lambda$.

Per esempio, ogni mezz'ora arrivano in media $\lambda = 10$ messaggi, la probabilità che nella prossima mezz'ora ne arrivino al più $k = 3$ è $P(X \leq 3) = \frac{10^0}{0!}e^{-10} + \frac{10^1}{1!}e^{-10} + ... + \frac{10^3}{3!}e^{-10} \approx 0.0103$.

La distribuzione di _Poisson_ è **approssimabile** con la distribuzione _binomiale_, infatti:
$$
\lim_{\substack{n \to \infty \\ p \to 0 \\ n \cdot p \to \lambda}} {n \choose k} p^k (1 - p)^{n - k} = \frac{\lambda^k}{k!}e^{-\lambda}
$$
spesso utilizzata quando $n \geq 100$ e $p \leq 0.05$, per cui si sceglie $\lambda = np$.

Per esempio, se $p = 0.03$ dei chip arrivano da una fabbrica, la probabilità che su $n = 100$ ce ne siano al massimo $k = 3$ da loro è $P(X \leq 3) = \sum\limits_{k = 0}^3 {100 \choose k} 0.03^k 0.97^{100 - k} \approx \sum\limits_{k = 0}^3 \frac{3^k}{k!}e^{-3}$ dove $\lambda = 100 \cdot 0.03 = 3$.

## Geometrica

$$X \sim \mathrm{Geo}(p)$$

- **Probabilità**: $P_X(x) = p(1 - p)^{x - 1}$
- **Media**: $E(X) = \frac{1}{p}$
- **Varianza**: $\mathrm{Var}(X) = \frac{1 - p}{p^2}$
- **Funzione**,
	- per $P(X = x)$: `dgeom(x=x-1, prob=p)`
	- per $P(X \leq x)$: `pgeom(q=x-1, prob=p)`

Modella $X$ sul numero di **ripetizioni necessarie** per ottenere il primo successo con probabilità $p$.

Questa è l'unica distribuzione discreta con **mancanza di memoria**, per cui:
$$
P(X > n + m | X > m) = \frac{P(X > n + m)}{P(X > m)} = P(X > n)
$$
dato che $P(X > k) = (1 - p)^k$.

Nell'esempio dei [siti web](#binomiale), per trovare la parola servono $x = 15$ siti con probabilità $P_X(15) \approx 0.0088$, e ne servono più di $10$ avendone controllati $4$ con probabilità $P(X > 10 | X > 4) = P(X > 6) \approx 0.261$.
