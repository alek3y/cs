# Funzioni elementari

- $$\log n = O(n)$$

	Infatti se $\log n = \ln n$, si sa che $\ln n \leq n - 1 \leq n$ perchè $n - 1$ è la _tangente_ di $\ln n$ su $n = 1$.

	Analoghe sono le altre basi di $\log$.

- $$n \log n = O(n^2)$$

	Sapendo che $\log n \leq n$ per la precedente dimostrazione, anche $n \cdot \log n \leq n \cdot n$.

- $$n! = O(n^n)$$

	Infatti $n! = \underbrace{\overset{\leq n}{1} \cdot \overset{\leq n}{2} \cdot ... \cdot \overset{\leq n}{n}}_n \leq \underbrace{n \cdot n \cdot ... \cdot n}_n$.

- $$n! = \Omega(2^n)$$

	Per cui $n! = 1 \cdot \overset{\geq 2}{2} \cdot ... \cdot \overset{\geq 2}{n} \geq 1 \cdot 2 \cdot ... \cdot 2 = 2^{n-1}$, di conseguenza $n! \geq c \cdot 2^n = 2^{n-1}$ se $c = \frac{1}{2}$.

- $$\log n! = O(n \log n)$$

	Sapendo che $n! \leq n^n$ per le precedenti dimostrazioni, allora $\log n! \leq \log n^n = n \log n$

- $$\sqrt{n} = O(n)$$

	Infatti se $n \leq n^2$, allora $\sqrt{n} \leq n$.
