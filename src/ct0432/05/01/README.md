# Teoremi

1. **Teorema di derivabilità**

	Una funzione $f$ si dice **derivabile** se:
$$
f_-'(x_0) = f_+'(x_0) \Leftrightarrow
\lim_{h \to 0^-} \frac{f(x_0 + h) - f(x_0)}{h} =
\lim_{h \to 0^+} \frac{f(x_0 + h) - f(x_0)}{h}
$$
	e quindi su $x_0$ si ottiene la stessa derivata sia da destra che da sinistra.

2. **Teorema di continuità delle funzioni derivabili**

	$$f \text{ derivabile in } x_0 \Rightarrow f \text{ continua su } x_0$$
	ma non vale il contrario, infatti se è continua non è necessariamente derivabile.

	Al contrario però,
	$$f \text{ non continua su } x_0 \Rightarrow f \text{ non derivabile in } x_0$$

## Esempio

Sia $f$ la funzione di valore assoluto:
$$
f(x) = |x| = \begin{cases}
x & \text{se } x \geq 0 \\
-x & \text{se } x < 0
\end{cases}
$$

La sua derivata sarà divisa in base ai due casi:
- Se $x_0 > 0$:
$$
\lim_{x \to x_0} \frac{f(x) - f(x_0)}{x - x_0} =
\lim_{x \to x_0} \frac{|x| - |x_0|}{x - x_0} =
\lim_{x \to x_0} \frac{x - x_0}{x - x_0} = 1
$$
- Se $x_0 < 0$:
$$
\lim_{x \to x_0} \frac{f(x) - f(x_0)}{x - x_0} =
\lim_{x \to x_0} \frac{|x| - |x_0|}{x - x_0} =
\lim_{x \to x_0} \frac{-x + x_0}{x - x_0} = -1
$$
- Se $x_0 = 0$, va controllata la derivabilità:
	$$f_-'(x_0) = -1 \neq f_+'(x_0) = 1$$
	di conseguenza $f$ non è derivabile su $x_0 = 0$ e quindi i punti di derivabilità sono $\mathbb{R} \setminus \{0\}$.
