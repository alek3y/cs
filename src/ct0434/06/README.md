# Sommatorie

- $$
\sum_{k = 0}^n a_k = \sum_{0 \leq k \leq n} a_k
$$

- $$
\sum_{k \in \emptyset} a_k =
\sum_{k = 1}^0 a_k =
\sum_{1 \leq k \leq 0} a_k = 0
$$

- \
$$
\sum_{k = 0}^n k(k-1)(n-k) = \sum_{k = 2}^{n - 1} k(k-1)(n-k)
$$
	perchè dalla somma si possono escludere i casi in cui $a_k = 0$:
	- $k = 0 \Rightarrow a_k = 0$
	- $k = 1 \Rightarrow a_k = 0$
	- $k = n \Rightarrow a_k = 0$

	e tenere gli altri casi, come:
	- $k = 2 \Rightarrow a_k \neq 0$
	- $k = n-1 \Rightarrow a_k \neq 0$

## Proprietà

- Proprietà associativa:
$$\sum_{k \in K} (a_k + b_k) = \sum_{k \in K}a_k + \sum_{k \in K}b_k$$

- Proprietà distributiva:
$$\sum_{k \in K} ca_k = c\sum_{k \in K}a_k,\, c \in \mathbb{R}$$

## Fondere e spezzare le sommatorie

Per fondere una sommatoria:
$$\sum_{k = 1}^m a_k + \sum_{k = m}^n a_k = \sum_{k = 1}^n a_k + a_m$$
dove $a_m$ viene aggiunto perchè a sinistra dell'uguale viene sommato due volte.

Mentre, per spezzarla basta:
$$\sum_{k = 1}^n a_k = \sum_{k = 1}^m a_k + \sum_{k = m+1}^n a_k$$

## Somme multiple

Se gli indici di somme multiple sono **indici indipendenti**, allora:
$$
\sum_{i = 1}^5 \left(\sum_{j = 0}^4 \left(\sum_{k = 0}^3 \left(2^i + \frac{3i}{k}\right)\right)\right) =
\sum_{i = 1}^5 \sum_{k = 0}^3 \sum_{j = 0}^4 \left(2^i + \frac{3i}{k}\right)
$$

## Somme notevoli

- $$\sum_{i = 1}^n i = \frac{n(n+1)}{2}$$
- $$\sum_{i = 1}^n i^2 = \frac{n(n+1)(2n+1)}{6}$$
- $$\sum_{i = 0}^n a^i = \frac{a^{n+1}-1}{a-1},\, a > 1$$
