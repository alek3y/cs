# Divisibilità

In decimale $a \in \mathbb{N}$ si può rappresentare come la somma delle sue cifre:
$$a = c_0 + 10c_1 + 10^2c_2 + ... + 10^nc_n = c_0 + \sum_{i=1}^n c_i 10^i$$

Quindi per stabilire se $b|a$, cioè se $a$ è multiplo di $b$, basta verificare che:
$$a \equiv_b 0 \Leftrightarrow c_0 + \sum_{i=1}^n c_i (10^i \bmod b) \equiv_b 0$$

Di conseguenza se ne può ricavare che:
- $$2|a \Leftrightarrow c_0 \equiv_2 0$$
	cioè quando l'ultima cifra è pari, perchè $10^i \equiv_2 0, \forall i \geq 1$.

- $$3|a \Leftrightarrow \sum_{i=0}^n c_i \equiv_3 0$$
	cioè quando la somma delle cifre è multiplo di $3$, perchè $10^i \equiv_3 1, \forall i \geq 1$.

- $$4|a \Leftrightarrow c_0 + 2c_1 \equiv_4 0$$
	perchè $10^1 \equiv_4 2$ mentre $10^i \equiv_4 0, \forall i \geq 2$.

- $$5|a \Leftrightarrow c_0 \equiv_5 0$$
	cioè quando l'ultima cifra è $0$ o $5$, perchè $10^i \equiv_5 0$.

- $$6|a \Leftrightarrow 3|a \land 2|a$$
	e quindi la somma delle cifre è multiplo di $3$ e l'ultima è pari.

e per estensione:
- $$8|a \Leftrightarrow c_0 + 2c_1 + 4c_2 \equiv_8 0$$
	perchè $10^1 \equiv_8 2$, $10^2 \equiv_8 4$ mentre $10^i \equiv_8 0, \forall i \geq 3$.

- $$9|a \Leftrightarrow \sum_{i=0}^n c_i \equiv_9 0$$
	cioè quando la somma delle cifre è multiplo di $9$, perchè $10^i \equiv_9 1, \forall i \geq 1$.

- $$10|a \Leftrightarrow c_0 \equiv_{10} 0$$
	cioè quando l'ultima cifra è $0$, perchè $10^i \equiv_{10} 0$.

In questo modo è possibile semplificare il calcolo del resto, per esempio $12530 \cdot 114211 \equiv_3 2 \cdot 1 \equiv_3 2$, perchè $1 + 2 + 5 + 3 + 0 = 11 \equiv_3 2$ e $1 + 1 + 4 + 2 + 1 + 1 = 10 \equiv_3 1$.

## Minimo comune multiplo

Dati $a, b \in \mathbb{Z}$ il minimo fra i multipli comuni di $a$ e $b$ è detto **minimo comune multiplo**, ed è _il numero più piccolo che è multiplo di entrambi_:
$$\mathrm{mcm}(a, b)$$

## Massimo comun divisore

Dati $a, b \in \mathbb{Z}$ il massimo fra i divisori comuni di $a$ e $b$ è detto **massimo comun divisore**, ed è _il numero più grande che divide entrambi_:
$$\mathrm{MCD}(a, b)$$

Quando $MCD(a, b) = 1$, $a$ e $b$ si dicono **relativamente primi** o anche **coprimi**.

### Algoritmo di Euclide

Siano $a, b \in \mathbb{N}$, la procedura sarà:
```c
int gcd(int a, int b) {
	if (b > a) {
		swap(&a, &b);
	}

	while (b != 0) {
		int r = a % b;
		a = b;
		b = r;
	}

	return a;
}
```

che espressa ricorsivamente diventa:
$$
\mathrm{MCD}(a, b) =
\begin{cases}
a & \text{se } b = 0 \\
\mathrm{MCD}(b, a \bmod b) & \text{se } b > 0
\end{cases}
$$

Per esempio, se $a = 300$ e $b = 18$:
1. $a = 18$, $b = 300 \bmod 18 = 12$
2. $a = 12$, $b = 18 \bmod 12 = 6$
3. $a = 6$, $b = 12 \bmod 6 = 0$

e quindi $\mathrm{MCD}(300, 18) = 6$.

### Identità di Bezout

Siano $a, b \in \mathbb{N}^+$, allora:
$$\exists x, y \in \mathbb{Z} : \mathrm{MCD}(a, b) = ax + by$$

Per esempio, $\mathrm{MCD}(12, 8) = 4$ allora $4 = 12 \cdot 1 + 8 \cdot (-1)$.
