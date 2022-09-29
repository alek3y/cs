# Logica

La logica è detta **algebra booleana** quando riguarda lo studio di proposizioni che possono risultare nei valori di verità **vero** e **falso**.

Un predicato si può riassumere come $P(x)$, e viene espresso come "$x$ soddisfa $P$".

I tipi di predicati sono:
- **Atomico** (o semplice): \
	Esprime una relazione tra **oggetti matematici** (e.g. $3 < 5$, $5 \text{ è pari}$).

	Quando la relazione è tra variabili il predicato si dice **atomico con variabili** (e.g. $x + y = y + x$, $x \text{ è pari}$).

- **Composto**: È un predicato composto da più predicati semplici connessi tramite dei **connettivi logici** e **quantificatori**

## Connettivi

| $P$ | $Q$ | $P \lor Q$ | $P \land Q$ | $\neg P$ |
|:-:|:-:|:-:|:-:|:-:|
| V | V | V | V | F |
| V | F | V | F | F |
| F | V | V | F | V |
| F | F | F | F | V |

### Legge di De Morgan

$$\neg (P \lor Q) = \neg P \land \neg Q$$
$$\neg (P \land Q) = \neg P \lor \neg Q$$

### Implicazione

L'implicazione $P \Rightarrow Q$ può anche essere letta come "se $P$ allora $Q$".

Nel caso in cui $P$ sia valido, allora si può dedurre che anche $Q$ è valido.
Se invece, $P$ (l'**ipotesi**) non è valido, allora non si è assicurati che $Q$ sia valido e di conseguenza si considera l'implicazione come valida.

| $P$ | $Q$ | $P \Rightarrow Q$ | $\neg P \lor Q$ | $\neg Q \Rightarrow \neg P$ |
|:-:|:-:|:-:|:-:|:-:|
| V | V | V | V | V |
| V | F | F | F | F |
| F | V | V | V | V |
| F | F | V | V | V |

#### Esempio

Per esempio, avendo un insieme $S$, si vuole inserire solamente quegli elementi che soddisfano $P \Rightarrow Q$, dove $P = "\text{è rosso}"$ e $Q = "\text{è quadrato}"$.

L'elemento "🔴" soddisfa $P$, per cui deve rispettare anche $Q$, ma non essendo quadrato non potrà essere inserito.

Invece l'elemento "🟢", non soddisfa $P$, e di conseguenza **non è limitato** da $Q$, per cui può essere inserito, indipendentemente che soddisfi $Q$ o no.

Un altro esempio è, con $P = "\text{avere la patente}"$, e $Q = "\text{essere maggiorenni}"$, allora $P \Rightarrow Q$, e cioè che aver la patente implica l'aver raggiunto la maggiore età.

Essere maggiorenni però, non implica aver la patente, quindi $Q \not\Rightarrow P$.

#### Modus Ponens

Dalle implicazioni è possibile ricavare una deduzione detta **modus ponens**, per cui se $P$ è vero e $P \Rightarrow Q$ è vero, allora $Q$ è per forza vero.

Un teorema $P$ viene dimostrato utilizzando una catena di queste deduzioni, $P_0 \Rightarrow P_1 \Rightarrow \cdots \Rightarrow P_n = P$, dove $P_0$ è detto **assioma** (un enunciato considerato vero).

### Doppia implicazione

La doppia implicazione $P \Leftrightarrow Q$ può essere letta come "$P$ se e solo se $Q$", dove:
- $P \Rightarrow Q$ è la **condizione sufficente**
- $Q \Rightarrow P$ è la **condizione necessaria**

| $P$ | $Q$ | $P \Leftrightarrow Q$ | $P \Rightarrow Q \land Q \Rightarrow P$ |
|:-:|:-:|:-:|:-:|
| V | V | V | V |
| V | F | F | F |
| F | V | F | F |
| F | F | V | V |

#### Esempio

Dimostrare che $n \in \mathbb{N}$ è multiplo di $12$ **se e solo se** è multiplo di $3$ e di $4$.

1. Condizione sufficente ($\Rightarrow$)

	Si suppone che $n = 12t$, $t \in \mathbb{N}$.

	Dato che $12 = 3 \cdot 4 = 4 \cdot 3$, $n = 3 \cdot (4 \cdot t) = 4 \cdot (3 \cdot t)$, cioè $n = 3r$ o $n = 4s$, cioè che $n$ è multiplo di $3$ e $4$.

2. Condizione necessaria ($\Leftarrow$)

	Si suppone che $n = 3r$ e $n = 4s$.

	Quindi $3r = 4s$, allora $4s$ è multiplo di $3$ ma dato che $4$ non lo è $s = 3t$, cioè $n = 4s = 4 \cdot 3 \cdot t = 12t$.

## Priorità

1. $\neg$: _Negazione_
2. $\land$: _Congiunzione_
3. $\lor$: _Disgiunzione_
4. $\Rightarrow$: _Implicazione_
5. $\Leftrightarrow$: _Doppia implicazione_

Per esempio, $\neg P \land Q \Rightarrow R \lor \neg S = ((\neg P) \land Q) \Rightarrow (R \lor (\neg S))$.

## Quantificatori

1. **Universale**

	$$\forall x \in X, P(x)$$

	È vero se $P(x)$ è vero per ogni $x$.

	È falso invece, se $P(x)$ è falso per almeno un $x$, chiamato **contro-esempio**, quindi:

	$$\neg (\forall x, P(x)) \Leftrightarrow \exists x, \neg P(x)$$

2. **Esistenziale**

	$$\exists x \in X, P(x)$$

	È vero se $P(x)$ è vero per almeno un $x$, chiamato **esempio**.

	È falso invece, se $P(x)$ è falso per ogni $x$, quindi:

	$$\neg (\exists x, P(x)) \Leftrightarrow \forall x, \neg P(x)$$
