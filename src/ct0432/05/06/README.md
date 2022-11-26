# Teoremi fondamentali su intervalli

Sia $f\colon [a, b] \to \mathbb{R}$ _continua_ e _derivabile_ in $]a, c[$, allora:

1. **Teorema di Lagrange**:

	Sia $f$ derivabile in $]a, b[$, allora $\exists c \in ]a, b[$:
	$$f'(c) = \frac{f(b) - f(a)}{b - a}$$
	e cioè che esiste un punto su cui la **tangente è parallela alla secante** passante per gli estremi.

2. **Teorema di Rolle**:

	Sia $f$ derivabile in $]a, b[$ tale che $f(a) = f(b)$, allora $\exists c \in ]a, b[$:
	$$f'(c) = 0$$

	Se per $\forall x \in ]a, b[$,
	- $f'(x) = g'(x)$, allora $f(x) = g(x) + d$ con $d \in \mathbb{R}$
	- $f'(x) = 0$, allora $f(x) = d$ e quindi $f$ è **costante**
	- $f'(x) > 0$, allora $f$ è **strettamente crescente** in $]a, b[$
	- $f'(x) < 0$, allora $f$ è **strettamente decrescente** in $]a, b[$

3. **Teorema di Cauchy**:

	Siano $f$ e $g$ derivabili in $]a, b[$ con $g'(x) \neq 0, \forall x \in ]a, b[$, allora $\exists c \in ]a, b[$:
	$$\frac{f(b) - f(a)}{g(b) - g(a)} = \frac{f'(c)}{g'(c)}$$
	che è una generalizzazione del _teorema di Lagrange_.

4. **Proprietà di Darboux**

	Se $x, y \in ]a, b[$ con $f'(x) \neq f'(y)$, allora:
	$$\forall m \in [f'(x), f'(y)], \exists c \in [x, y] : f'(c) = m$$
	e cioè che tutti i coefficienti angolari tra $f'(x)$ e $f'(y)$, corrispondono al coefficiente angolare di almeno un punto tra $x$ e $y$.

5. **Teorema di de l'Hopital**

	Siano $f$ e $g$ derivabili in un intorno $I_{x_0} \setminus \{x_0\}$, allora se:
	$$\lim_{x \to x_0} f(x) = \lim_{x \to x_0} g(x) \in \{0, \infty\} \land \lim_{x \to x_0} \frac{f'(x)}{g'(x)} = l$$
	si ha che:
	$$\lim_{x \to x_0} \frac{f(x)}{g(x)} = l$$

	Per esempio:
	$$\lim_{x \to 0} \frac{\sin(x)}{6x} \underset{[\frac{0}{0}]}{=} \lim_{x \to 0} \frac{\cos(x)}{6} = \frac{1}{6}$$
