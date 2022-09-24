# Matematica discreta

## Terminologia

- **Enunciato**: indica un espressione linguistica che rappresenta proprietà di **oggetti matematici** per cui è possibile stabilire un **valore di verità**. Può essere di tipo:

	- **Semplice**

	- **Composto**: formato da più enunciati connessi da dei connettivi

	- **Esistenziali**: cominciano con "Esiste" (e.g. "Esiste $n \in \mathbb{N} : n^2 = 441$"), e viene dimostrato con un **esempio** (e.g. "$21^2 = 441$"). Per dimostrare che è falso invece, va dimostrato **per ogni valore di** $n$ sul suo insieme di appartenenza.

	- **Universali**: cominciano con "Per ogni" (e.g. "Per ogni $n \in \mathbb{N}, n^2+n$ è pari"), e viene dimostrato per **tutti i valori di** $n$. Per dimostrare che è falso invece, basta un **contro-esempio**.

- **Oggetti matematici**: sono elementi come le funzioni, variabili, matrici, sequenze, etc.

- **Valori di verità**: il valore che può essere associato ad un enunciato, cioè **vero** o **falso**.

- **Teorema**: viene espresso attraverso un enunciato, ed è formato da una **dimostrazione**, cioè il processo che permette di ricavare il valore di verità.

	- **Lemma**: è un teorema che serve per la dimostrazione di un teorema successivo più importante

	- **Proposizione**: è un teorema di meno rilevanza utile per la dimostrazione di un teorema più importante

	- **Corollario**: è un teorema a seguito di un altro, la cui dimostrazione è talmente immediata da poter essere solo accennata o omessa

## Esempi di problemi

1. _Enunciato_: **Per ogni** (enunciato universale) $n \in \mathbb{N}, n^2 + n$ è pari.

	_Dimostrazione_:

	Si osserva che un numero $n$ è **pari** se $n = 2k$, mentre è **dispari** se $n = 2k + 1$ (entrambi con $k \in \mathbb{N}$).

	Si deve quindi dimostrare che $n^2 + n = 2k$ avendo $n, k \in \mathbb{N}$.

	Casi:
	1. $n$ **è pari**, quindi $n = 2r$, per cui $n^2 + n = 4r^2 + 2r = 2(2r^2 + r)$.

		Poniamo quindi $k = 2r^2 + r$, che rende $n^2 + n = 2k$.

	2. $n$ **è dispari**, quindi $n = 2r + 1$, per cui $n^2 + n = (2r + 1)^2 + (2r + 1) = 2(2r^2 +3r + 1)$.

		Poniamo quindi $k = 2r^2 + 3r + 1$, che rende $n^2 + n = 2k$.

	In entrambi i casi, $n^2 + n = 2k$, quindi è provato che sarà sempre pari.

2. _Enunciato_: **Per ogni** $n \in \mathbb{N}$, "$4n - 1$ è un numero primo" è falso.

	_Dimostrazione_:

	Si ha che:
	$$n = 1; \hspace{1em} 4n - 1 = 3$$
	$$n = 2; \hspace{1em} 4n - 1 = 7$$
	$$n = 3; \hspace{1em} 4n - 1 = 11$$
	$$n = 4; \hspace{1em} 4n - 1 = 15$$

	Per cui, con il **contro-esempio** $n = 4$, il teorema è vero.

3. _Enunciato_: **Non esiste** un numero primo massimo.

	_Dimostrazione_:

	Per assurdo, si assume che esista un numero primo massimo $x$. Si consideri quindi $y$ come il numero successivo al prodotto tra tutti questi numeri fino ad $x$.

	$$y = (2 \cdot 3 \cdot 5 \cdot 7 \cdot ... \cdot x) + 1$$

	Il valore $y$ sarà sempre maggiore di $x$, quindi $y$ **non è primo**, di conseguenza $y$ **deve essere** divisibile per almeno uno dei numeri primi da $2$ a $x$.

	$y$ però **non è divisibile per nessuno di questi numeri** (_contraddizione_), perchè avrà sempre resto $1$ (nota il $+1$ alla fine del prodotto).

	Pertanto, il teorema è vero.
