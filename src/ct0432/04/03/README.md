# Funzioni continue

Una funzione $f\colon A \subseteq \mathbb{R} \to \mathbb{R}$ si dice **continua su un punto** $x_0 \in A$, se:
$$\lim_{x \to x_0} f(x) = f(x_0)$$
che vale se $x_0$ è di **accumulazione** per $A$ (cioè che ogni intorno su $x_0$ ha punti appartenenti ad $A$), ma anche se è un punto **isolato** di $A$ (perchè non ha punti nell'intorno di $x_0$).

Di conseguenza, la funzione $f$ si dice **continua** se è continua su **ogni punto del suo dominio**.

## Determinare la continuità

Per determinare se la funzione è continua sul punto $x_0$ basterà tenderci da sinistra e da destra:
$$f \text{ è continua} \Leftrightarrow \lim_{x \to x_0^-} f(x) = \lim_{x \to x_0^+} f(x) = f(x_0) \in \mathbb{R}$$

Per esempio, con $\mathrm{sgn}(x)$ non é continua su $x = 0$ perchè
$$\lim_{x \to 0^-} \mathrm{sgn}(x) \neq \lim_{x \to 0^+} \mathrm{sgn}(x)$$
mentre $f(x) = \begin{cases}1 & \text{se } x > 0 \\ -1 & \text{se } x < 0\end{cases}$ è continua perchè $D = \mathbb{R} \setminus \{0\}$ e quindi $0 \not\in \mathrm{Dom}(f)$.

## Operazioni tra funzioni continue

$$f, g: A \to \mathbb{R} \text{ continue}$$

Le operazioni tra due funzioni continue $f$ e $g$ risulteranno sempre in una funzione continua, come $f + g$, $f \cdot g$, $\frac{f}{g}$, $|f|$.

Vale anche per le funzioni composte $h(x) = (f \circ g)(x) = f(g(x))$, che si può verificare con il **cambio di variabili**, cioè:
$$\lim_{x \to x_0} g(x) = l \not\in \mathrm{Dom}(f), \text{ ma è di accumulazione su } \mathrm{Dom}(f) \Rightarrow \lim_{x \to x_0} f(g(x)) = \lim_{t \to l} f(t)$$

Per esempio, $h(x) = \ln \frac{1}{x}$ è continua perchè $\frac{1}{x}$ e $\ln x$ sono continue, e un limite si risolverà:
$$\lim_{x \to 0^+} \ln \frac{1}{x} \underset{\frac{1}{x} \to +\infty}{=} \lim_{t \to +\infty} \ln t = +\infty$$

## Funzioni inverse

Se una funzione $f$ è **strettamente monotona** e è definita su un **intervallo** oppure un **insieme chiuso e limitato** (per esempio l'unione di più intervalli), allora $f^{-1}$ è **continua**.

## Funzioni elementari continue

- $x^a$, $a^x$, $\log_a x$
- $\sin(x)$, $\tan(x)$
- $\arcsin(x)$, $\arctan(x)$
- $|x|$

Per decidere se una **funzione definita a tratti** è continua, bisogna controllare il punti di giunzione tra i vari casi.

Per esempio, la funzione $\mathrm{sgn}(\sin(x))$ è continua nei punti $\mathbb{R} \setminus \{k\pi\}$, perchè quando $\mathrm{sgn}(x_0) = 0$, $\lim_{x \to x_0^-} \mathrm{sgn}(x) \neq \lim_{x \to x_0^+} \mathrm{sgn}(x)$.

Un altro caso, è quello di trovare un parametro $a$ che renda la funzione definita a tratti continua, per esempio con:
$$f(x) = \begin{cases}x + 1 & \text{ se } x \leq 1 \\ 3 - 2ax^2 & \text{ se } x > 1\end{cases}$$
la funzione è continua se $\lim_{x \to 1^+} f(x) = f(1)$, e quindi se
$$3 - 2a \cdot 1^2 = 2 \Leftrightarrow 3 - 2a = 2 \Leftrightarrow a = \frac{1}{2}$$

## Teorema di Bolzano

Avendo una funzione $f\colon [a, b] \to \mathbb{R}$ continua, il teorema dice che se:
$$f(a) \cdot f(b) < 0$$
e quindi $f(a)$ ha segno opposto di $f(b)$ (perchè se sono entrambi negativi o positivi sono maggiori di zero), allora:
$$\exists c \in [a, b] : f(c) = 0$$
cioè che se $f$ è continua allora passerà per forza su $y = 0$.
