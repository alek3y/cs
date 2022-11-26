# Infiniti e infinitesimi

## Funzioni infinitesime

Una funzione $f$ si dice **infinitesima** su $x_0$ se:
$$\lim_{x \to x_0} f(x) = 0$$

L'**ordine di infinitesimo** di due funzioni $f$ e $g$ _infinitesime_, dette **infinitesimi simultanei** su $x_0$, è dato da:
$$\lim_{x \to x_0} \left|\frac{f(x)}{g(x)}\right| = l$$

Per cui, quando:
- $f$ **tende prima** a $0$ rispetto a $g$, $f$ ha ordine di **infinitesimo superiore** a $g$: $f = o(g) \Rightarrow l = 0$
- $g$ **tende prima** a $0$ rispetto a $f$, $f$ ha ordine di **infinitesimo inferiore** a $g$: $g = o(f) \Rightarrow l = +\infty$
- $f$ e $g$ **tendono insieme** a $0$, $f$ ha lo **stesso ordine di infinitesimo** di $g$: $f = O(f) \Rightarrow l > 0$

### Principio di sostituzione

Siano $f_1$, $f_2$, $g_1$, $g_2$ infinitesimi simultanei in $x_0$, allora
$$
\lim_{x \to x_0} \frac{f_1(x) + f_2(x)}{g_1(x) + g_2(x)} =
\lim_{x \to x_0} \frac
{f_1(x) \left(1 + \frac{f_2(x)}{f_1(x)}\right)}
{g_1(x) \left(1 + \frac{g_2(x)}{g_1(x)}\right)} =
\lim_{x \to x_0} \frac{f_1(x)}{g_1(x)}
$$
se e solo se $f_2$ e $g_2$ hanno ordine di **infinitesimo superiore** (cioè tendono prima a $0$) di $f_1$ e $g_1$, dato che nel raccoglimento $\frac{f_2(x)}{f_1(x)}$ e $\frac{g_2(x)}{g_1(x)}$ tendono a zero.

Per esempio:
$$
\lim_{x \to 0} \frac{2x + \sin^3(x) + 1 - \cos(x)}{\log(1 + x^2) + 3\sin(x)} =
\lim_{x \to 0} \frac
{2x \left(1 + \frac{\sin^3(x)}{2x} + \frac{1 - \cos(x)}{2x}\right)}
{3\sin(x) \left(\frac{\log(1 + x^2)}{3\sin(x)} + 1\right)} =
\lim_{x \to 0} \frac{2x \left(1 + \frac{1 - \cos(x)}{2x}\right)}{3\sin(x)}
$$
perchè $\frac{\sin^3(x)}{2x}, \frac{\log(1 + x^2)}{3\sin(x)} \to 0$, dato che $\sin^3(x)$ e $\log(1 + x^2)$ sono infinitesimi superiori a $2x$ e $3\sin(x)$,
$$
\lim_{x \to 0} \frac{2x \left(1 + \frac{1}{2} \frac{1 - \cos(x)}{x}\right)}{3\sin(x)}
\overset{\mathcal{H}}{=}
\lim_{x \to 0} \frac{2x \left(1 + \frac{1}{2} \frac{\sin(x)}{1}\right)}{3\sin(x)} =
\lim_{x \to 0} \frac{2x}{3\sin(x)} = \frac{2}{3}
$$

## Funzioni infinite

Una funzione $f$ si dice **infinita** su $x_0$ se:
$$\lim_{x \to x_0} f(x) = \infty$$

L'**ordine di infinito** di due funzioni $f$ e $g$ _infinite_, dette **infiniti simultanei** su $x_0$, è dato da:
$$\lim_{x \to x_0} \left|\frac{f(x)}{g(x)}\right| = l$$

Per cui, quando:
- $f$ **tende prima** a $\pm\infty$ rispetto a $g$, $f$ ha ordine di **infinito superiore** a $g$: $f = o(g) \Rightarrow l = +\infty$
- $g$ **tende prima** a $\pm\infty$ rispetto a $f$, $f$ ha ordine di **infinito inferiore** a $g$: $g = o(f) \Rightarrow l = 0$
- $f$ e $g$ **tendono insieme** a $\pm\infty$, $f$ ha lo **stesso ordine di infinito** di $g$: $f = O(f) \Rightarrow l > 0$

### Principio di sostituzione

Siano $f_1$, $f_2$, $g_1$, $g_2$ infiniti simultanei in $x_0$, allora
$$
\lim_{x \to x_0} \frac{f_1(x) + f_2(x)}{g_1(x) + g_2(x)} =
\lim_{x \to x_0} \frac
{f_1(x) \left(1 + \frac{f_2(x)}{f_1(x)}\right)}
{g_1(x) \left(1 + \frac{g_2(x)}{g_1(x)}\right)} =
\lim_{x \to x_0} \frac{f_1(x)}{g_1(x)}
$$
se e solo se $f_1$ e $g_1$ hanno ordine di **infinito inferiore** (cioè tendono prima a $\pm\infty$) di $f_2$ e $g_2$, dato che nel raccoglimento $\frac{f_2(x)}{f_1(x)}$ e $\frac{g_2(x)}{g_1(x)}$ tendono a zero.
