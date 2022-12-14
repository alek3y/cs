# Integrali impropri

Sia $f \in C^0 : [a, b] \to \mathbb{R}$, si dice **integrale improprio** l'integrale
$$
\int_a^{+\infty} f(x) dx = \lim_{t \to +\infty} \int_a^t f(x) dx = l \in \mathbb{R}
$$
per cui la funzione si dice **integrabile** a $+\infty$.

Quando invece, è posto su un punto di _discontinuità_ $c \in [a, b]$:
$$
\int_a^b f(x) dx = \lim_{t \to c^-} \int_a^t f(x) dx + \lim_{t \to c^+} \int_t^b f(x) dx
$$

Allo stesso modo, se l'intervallo di integrazione è $\mathbb{R}$:
$$
\int_{\mathbb{R}} f(x) dx =
\int_{-\infty}^{+\infty} f(x) dx =
\lim_{t \to -\infty} \int_t^0 f(x) dx + \lim_{t \to +\infty} \int_0^t f(x) dx
$$

## Esempio

Con il [teorema del confronto](../03/README.md#integrali-definiti) è possibile dire se un integrale improprio **diverge** o **converge**, per esempio:
$$
s_n = \sum_{n = 2}^{+\infty} \frac{1}{n} \leq
\int_1^{+\infty} \frac{1}{x} dx \leq
S_n = \sum_{n = 1}^{+\infty} \frac{1}{n}
$$
dove $s_n$ sono i _plurirettangoli inscritti_, mentre $S_n$ sono i _plurirettangoli circoscritti_.

Sapendo che $s_n$ diverge, allora possiamo dire che anche $\int_1^{+\infty} \frac{1}{x} dx$ diverge.

Un altro esempio è:
$$
\int_0^{+\infty} \sin(x) dx = \lim_{t \to +\infty} \int_0^t \sin(x) dx =
\lim_{t \to +\infty} -\cos(t) + \cos(0)
$$
che non ha soluzione perchè $\lim_{t \to +\infty} \cos(t)$ oscilla tra $[-1, 1]$ e quindi $\sin(x)$ **non è integrabile** a $+\infty$.

## Teorema delle serie armoniche

Visto l'esempio precedente, sfruttando le [serie armoniche](../../09/README.md#serie-armonica-generalizzata) è possibile determinare il **criterio di convergenza** dell'integrale improprio.

Per cui, nel caso di **asintoti orizzontali**,
$$
\int_a^{+\infty} \frac{1}{x^p} dx
$$
**converge** se $p > 1$ e **diverge** $p \leq 1$.

Mentre, nel caso di **asintoti verticali**,
$$
\int_0^a \frac{1}{x^p} dx
$$
**converge** se $p < 1$ e **diverge** $p \geq 1$.

Per esempio, $\int_1^{+\infty} \frac{1}{\sqrt[3]{x}} dx$ _diverge_ perchè $\frac{1}{3} \leq 1$.

## Teorema del confronto con funzioni simili

Attraverso il _teorema del confronto_ è possibile determinare il **carattere** dell'integrale tramite una funzione simile.

Per esempio, dell'integrale improprio
$$\int_2^{+\infty} \frac{\cos^2(x)}{x^2} dx$$
sappiamo che $\cos^2(x) \leq 1$ perchè **oscilla** in $[0, 1]$, infatti:
$$\frac{\cos^2(x)}{x^2} \leq \frac{1}{x^2},\; \forall x > 1$$
e sapendo che $\frac{1}{x^2}$ converge, anche $\frac{\cos^2(x)}{x^2}$ converge.

Un altro esempio è:
$$\int_2^{+\infty} \frac{1+\cos^2(x)}{\sqrt{x}(2-\sin^4(x))} dx$$
per cui, dato che
$$\frac{1}{2\sqrt{x}} \leq \frac{1+\cos^2(x)}{\sqrt{x}(2-\sin^4(x))}$$
allora l'integrale diverge, sapendo che anche $\frac{1}{2\sqrt{x}}$ diverge.
