# Operazioni tra funzioni

$$f, g: A \to \mathbb{R} \text{ continue}$$

Le operazioni tra due funzioni continue $f$ e $g$ risulteranno sempre in una funzione continua, come $f + g$, $f \cdot g$, $\frac{f}{g}$, $|f|$.

Vale anche per le funzioni composte $h(x) = (f \circ g)(x) = f(g(x))$, che si può verificare con il **cambio di variabili**, cioè:
$$\lim_{x \to x_0} g(x) = l \not\in \mathrm{Dom}(f), \text{ ma è di accumulazione su } \mathrm{Dom}(f) \Rightarrow \lim_{x \to x_0} f(g(x)) = \lim_{t \to l} f(t)$$

Per esempio, $h(x) = \ln \frac{1}{x}$ è continua perchè $\frac{1}{x}$ e $\ln x$ sono continue, e un limite si risolverà:
$$\lim_{x \to 0^+} \ln \frac{1}{x} \underset{\frac{1}{x} \to +\infty}{=} \lim_{t \to +\infty} \ln t = +\infty$$

## Funzioni inverse

Se una funzione $f$ è **strettamente monotona** e è definita su un **intervallo** oppure un **insieme chiuso e limitato** (per esempio l'unione di più intervalli), allora $f^{-1}$ è **continua**.
