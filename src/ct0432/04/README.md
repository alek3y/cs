# Limiti

Il limite di una funzione $f(x)$ su un **punto di accumulazione** $x_0$, esprime **verso quale valore tende la funzione** all'avvicinarsi a $x_0$.

$$\lim_{x \to x_0} f(x)$$

## Definizione epsilon-delta

Il limite $\lim_{x \to x_0} f(x) = l$ con $l \in \mathbb{R}$, **se e solo se**
$$
\forall \epsilon > 0,
\exists \delta > 0 : x \in B(x_0, \delta) \Rightarrow f(x) \in B(l, \epsilon),
\hspace{1em} \forall x \in \mathrm{Dom}(f) \setminus \{x_0\}
$$
e quindi, per ogni raggio $\epsilon$, si sta cercando un raggio $\delta$ per cui i tutti i valori immagine dell'intorno circolare $B(x_0, \delta)$ (cioè i valori delle $y$) appartengano a $B(l, \epsilon)$.

Di conseguenza la proprietà **vale e il limite esiste** se, qualunque sia $\epsilon$ (sulle _ordinate_), sarà sempre possibile trovare un suo $\delta$ corrispondente (sulle _ascisse_).

Si consideri $d(x_1, x_2) = |x_2 - x_1|$ cioè la distanza tra due punti $x_1$ e $x_2$, allora la definizione potrà essere espressa come
$$
\forall \epsilon > 0,
\exists \delta > 0 : d(x_0, x) < \delta \Rightarrow d(l, f(x)) < \epsilon
$$

Un esempio lo è [questa animazione](https://www.desmos.com/calculator/zzssfdbsln) per cui si può notare che quando $\delta \to 0$, anche $\epsilon$ fa tendere i valori di $f(x) \to l$.

### Intorno sinistro e destro

Quando su $x_0$ il limite **non è definito**, è possibile tendere verso il punto da **sinistra** o da **destra** per trovare il punto specifico verso cui tende la funzione.

Per esempio, nel caso della funzione
$$f(x) = \frac{x+1}{\mathrm{sgn}(x)} \Rightarrow \nexists \lim_{x \to 0} f(x)$$
perchè la [funzione tende](https://www.desmos.com/calculator/cp6ni9rnbv) verso $y = -1$ da _sinistra_, mentre verso $y = 1$ da _destra_, e di conseguenza
$$\forall \epsilon \in (0, 1), \nexists \delta > 0 : x \in B(0, \delta) \Rightarrow f(x) \in B(0, \epsilon)$$
cioè che quando $0 < \epsilon < 1$, non c'è alcun intervallo di cui le $x$ danno che $f(x) \in B(0, \epsilon)$, infatti quando $x = 0.04$, $f(x) = 1.04$ che supera l'intervallo di $\epsilon$, e quindi non vale _per ogni_ $\epsilon > 0$.

Per poter determinarlo, sarà quindi necessario tendere da **destra**, o da **sinistra** come:
$$\forall \epsilon > 0, \exists \delta > 0 : x \in (x_0 - \delta, x_0) \Rightarrow f(x) \in B(l, \epsilon)$$

### Caso infinito

Nel caso in cui $x \to \pm\infty$ o $f(x) \to \pm\infty$, invece di prendere un intorno circolare si prende un **intorno sinistro o destro** di $\pm\infty$.

Per esempio, con $\lim_{x \to +\infty} f(x) = l$:
$$\forall \epsilon > 0, \exists M > 0 : x > M \Rightarrow f(x) \in B(l, \epsilon)$$
e ne si può vedere l'effetto con [questa animazione](https://www.desmos.com/calculator/8286y6mubt).

## Teoremi

Tra i teoremi più importanti ci sono:

1. **Teorema di unicità**

	Che dice che un limite su un punto $x_0$ può assumere un solo valore $l \in \mathrm{Codom}(f)$.

	$$\lim_{x \to x_0} f(x) = l_1 \land \lim_{x \to x_0} f(x) = l_2 \Rightarrow l_1 = l_2$$
	perchè se fossero diversi, dovrebbe per forza esistere un intorno (delle $y$) di $l_1$ e un altro di $l_2$ che non condividono alcun punto, ma visto che per entrambi la _controimmagine_ (valori sulle $x$) è un intorno di $x_0$, allora se $x \in I_{x_0} \Rightarrow f(x) \in I_{l_1} \land f(x) \in I_{l_2}$, cosa assurda, visto che abbiamo dato per scontato che $I_{l_1} \cap I_{l_2} = \emptyset$.

2. **Teorema di permanenza del segno**

	Che dice che se il risultato di un limite è positivo, allora esiste un intorno di $x_0$ per cui $f(x)$ è positiva su tutti i punti dell'intorno.

	$$l > 0 \Rightarrow \exists I_{x_0} : f(x) > 0, \forall x \in I_{x_0}$$
