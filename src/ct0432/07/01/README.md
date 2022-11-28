# Polinomio di Taylor

Il **polinomio di Taylor** serve ad **approssimare** una funzione, ed aumentando il **grado** del polinomio [aumenta la precisione](https://www.desmos.com/calculator/ff3abvyzip) della funzione intorno ad un punto $x_0$.

Quindi, se $f \in C^k(I)$ e $x_0 \in I$:
$$T_{k, x_0}(x) = \sum_{n = 0}^k \frac{f^{(n)}(x_0)}{n!}(x - x_0)^n$$
dove $k$ è l'**ordine** del polinomio di Taylor.

Il punto $x_0$ è l'unico punto per cui $T_{n, x_0}(x_0) = f(x_0)$, cioè è l'unico che tocca veramente la funzione, infatti è usato come punto di partenza per sviluppare il polinomio di Taylor.

Quando $x_0 = 0$, $T_{n,0}(x)$ viene detto **polinomio di Maclaurin**.

Un esempio è $f(x) = e^x$, per cui $T_{n, 0}(x) = 1 + x + \frac{x^2}{2} + \frac{x^3}{6} + ... + \frac{x^n}{n!}$.

## Formula di Taylor-Peano

Più ci si sposta dal punto $x_0$ più la precisione del polinomio diminuisce, per cui la **formula di Taylor-Peano** serve a trovare l'errore (cioè la differenza) tra la vera funzione e quella approssimata.

Sia $f$ derivabile $n-1$ volte in $\mathrm{Dom}(f)$ e derivabile $n$ volte in $x_0$ con $x_0 \in \mathrm{Dom}(f)$, allora il resto sarà:
$$w_n(x) = f(x) - T_{n, x_0}(x) = o((x - x_0)^n)$$
dove $o((x - x_0)^n)$ è chiamato **resto di Peano**, infatti $w_n(x)$ è _infinitesima superiore_ rispetto a $(x - x_0)^n$, cioè tende più velocemente a $0$.

## Formula di Taylor-Lagrange

La **formula di Taylor-Lagrange**, come per quella di _Taylor-Peano_, trova il resto tra una funzione $f$ e la sua approssimata.

Sia $f$ derivabile $n+1$ volte in $I$ e $f^{(n)}$ continua, allora:
$$
\forall x \in I, \exists c \in I :
f(x) = T_{n, x_0}(x) + \frac{f^{(n + 1)}(c)}{(n + 1)!}(x - x_0)^{n + 1}
$$
dove il secondo addendo è il **resto in forma di Lagrange**.
