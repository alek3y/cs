# Algebra dei limiti

Le operazioni principali tra due limiti come
$$\lim_{x \to x_0} f(x) = l \hspace{2em} \lim_{x \to x_0} g(x) = m$$
comprendono:

- **Limite della somma**

	$$\lim_{x \to x_0} (f(x) + g(x)) = l + m$$

	| | $m \in \mathbb{R}$ | $m = +\infty$ | $m = -\infty$ |
	|:-:|:-:|:-:|:-:|
	| $l \in \mathbb{R}$ | $l+m$ | $+\infty$ | $-\infty$ |
	| $l = +\infty$ | $+\infty$ | $+\infty$ | _F. I._ |
	| $l = -\infty$ | $-\infty$ | _F. I._ | $-\infty$ |

	Per esempio,
	$$\lim_{x \to +\infty} (x^2 - x + 1) \underset{[+\infty - \infty]}{=} \lim_{x \to +\infty} \left(x^2 \left(1 + \frac{1}{x^2} - \frac{1}{x}\right)\right) = +\infty$$

- **Limite del prodotto**
	$$\lim_{x \to x_0} (f(x) \cdot g(x)) = l \cdot m$$

	| | $m < 0$ | $m = 0$ | $m > 0$ | $m = +\infty$ | $m = -\infty$ |
	|:-:|:-:|:-:|:-:|:-:|:-:|
	| $l < 0$ | $l \cdot m$ | $0$ | $l \cdot m$| $-\infty$ | $+\infty$ |
	| $l = 0$ | $0$ | $0$ | $0$ | _F. I._ | _F. I._ |
	| $l > 0$ | $l \cdot m$| $0$ | $l \cdot m$| $+\infty$ | $-\infty$ |
	| $l = +\infty$ | $-\infty$ | _F. I._ | $+\infty$ | $+\infty$ | $-\infty$ |
	| $l = -\infty$ | $+\infty$ | _F. I._ | $-\infty$ | $-\infty$ | $+\infty$ |

	Per esempio,
	$$\lim_{x \to +\infty} \frac{1}{x} \cdot x \underset{[0 \cdot +\infty]}{=} 1$$

- **Limite del reciproco**
	$$\lim_{x \to x_0} \frac{f(x)}{g(x)} = \frac{l}{m}$$

	| | $m < 0$ | $m = 0^-$ | $m = 0^+$ | $m > 0$ | $m = +\infty$ | $m = -\infty$ |
	|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
	| $l < 0$ | $l/m$ | $+\infty$ | $-\infty$ | $l/m$| $0^-$ | $0^+$ |
	| $l = 0$ | $0$ | _F. I._ | _F. I._ | $0$ | $0$ | $0$ |
	| $l > 0$ | $l/m$| $-\infty$ | $+\infty$ | $l/m$| $0^+$ | $0^-$ |
	| $l = +\infty$ | $-\infty$ | $-\infty$ | $+\infty$ | $+\infty$ | _F. I._ | _F. I._ |
	| $l = -\infty$ | $+\infty$ | $+\infty$ | $-\infty$ | $-\infty$ | _F. I._ | _F. I._ |

- **Limite di funzioni monotone**

	$$f\colon A \subseteq \mathbb{R} \to \mathbb{R} \text{ monotona}$$

	Avrà il limite,
	$$\lim_{x \to \alpha} f(x) = l$$
	che corrisponderà a:

	| | $f$ crescente | $f$ decrescente |
	|:-:|:-:|:-:|
	| $\alpha = \sup(A)$ | $l = \sup(\mathrm{Im}(f))$ | $l = \inf(\mathrm{Im}(f))$ |
	| $\alpha = \inf(A)$ | $l = \inf(\mathrm{Im}(f))$ | $l = \sup(\mathrm{Im}(f))$ |

	con $\alpha \not\in A$.
