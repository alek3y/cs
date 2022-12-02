# Algebra degli integrali

Alcune delle operazioni integrabili tra due funzioni $f$ e $g$, comprendono:

- **Proprietà di linearità**:
	$$\int af(x) + bg(x) dx = a\int f(x) dx + b\int g(x) dx$$

- **Funzione composta**:
	$$f'(g(x)) \cdot g'(x) dx = f(g(x)) + c$$

- **Integrazione per parti**:
	$$\int f'(x) g(x) dx = f(x)g(x) - \int f(x) g'(x) dx$$
	che è ricavabile attraverso l'integrazione della _derivata del prodotto_.

	Per esempio:
$$
\begin{split}
\int \log(x) dx &=
\int 1 \cdot \log(x) dx =
\int (x)' \cdot \log(x) dx = \\
&= x \log(x) - \int x \cdot \frac{1}{x} dx =
x \log(x) - x + c
\end{split}
$$

- **Integrazione per sostituzione**:

	Lo scopo di questo metodo è quello di sostituire con $t$ quello che da più fastidio, per esempio:
	$$\int -\frac{1}{\sqrt{1 - x}} dx = \int \frac{1}{t} 2tdt = 2t + c = 2\sqrt{1 - x} + c$$
	se e solo se:
	$$t = \sqrt{1 - x} \Rightarrow t^2 = 1 - x \Rightarrow 2tdt = -1dx$$

- **Integrazione di funzioni razionali**:

	In base al gradi del numeratore $N(x)$ e del denominatore $D(x)$, l'integrazione avviene con:
	- La **divisione tra polinomi**, se $\deg(N(x)) \geq \deg(D(x))$
	- Il metodo **A e B**, altrimenti
