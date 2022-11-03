# Derivate

La derivata di una funzione $f$ è l'equazione che restituisce il **coefficiente angolare** della **tangente** su un dato punto $x$.

## Rette

L'equazione di una retta è:
$$y = mx + q, \text{ con } m = \frac{\Delta y}{\Delta x} = \frac{y_2 - y_1}{x_2 - x_1}$$
e passante per un punto $P = (x_0, y_0)$:
$$y - y_0 = m(x - x_0)$$

## Rapporto incrementale

Si dice **retta secante** al grafico di una funzione $f$, la retta che interseca la funzione in due o più punti.

Il **rapporto incrementale** su $x_0$ con incremento $x - x_0$, corrisponde al **coefficiente angolare** della _retta secante_, che ha una delle due intersezioni sul punto $x_0$ interno al $\mathrm{Dom}(f)$, ed è:
$$r(x) = \frac{f(x) - f(x_0)}{x - x_0}$$

## Definizione

Avendo una funzione $f\colon A \subseteq \mathbb{R} \to \mathbb{R}$, dove $A$ è un intervallo o unione di intervalli e $x_o$ è interno ad $A$, se
$$f'(x_0) = \lim_{x \to x_0} r(x) = \lim_{x \to x_0} \frac{f(x) - f(x_0)}{x - x_0}$$
**esiste** ed è **finito**, allora $f$ si dice **derivabile** in $x_0$.

Un altro modo per scriverlo, ponendo $h = x - x_0$, è:
$$f'(x_0) = \lim_{h \to 0} \frac{f(x_0 + h) - f(x_0)}{h}$$

## Teoremi

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

## Funzioni elementari

- $f(x) = c \Rightarrow f'(x) = 0$
- $f(x) = x^n \Rightarrow f'(x) = nx^{n-1}$

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
