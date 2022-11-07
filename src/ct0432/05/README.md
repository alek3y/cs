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
