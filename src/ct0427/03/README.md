# Intervalli di confidenza

<p align="center">
<img src="assets/01.png" alt="Normale con mergine di errore"
</p>

Un $[A, B]$ è detto **intervallo di confidenza** per $\theta$ di livello $(1-\alpha) 100\%$ se ha **livello di confidenza**:
$$
P(A \leq \theta \leq B) = 1 - \alpha
$$
dove $\theta$ è un **numero** mentre $A$ e $B$ sono **variabili casuali** perchè dipendono dal campione.

Basta che $\hat\theta$ sia **asintoticamente non distorto e normale**, per ricavare gli intervalli dalla **statistica Z**:
$$
Z = \frac{\hat\theta - \theta}{\mathrm{SE}(\hat\theta)}
$$
ovvero $\hat\theta$ [standardizzato](../../ct0111/03/02/README.md#normale), trovando il **quantile** $z_{\alpha/2}$ di livello $\frac{\alpha}{2}$ con `qnorm(1 - 𝛼/2)`.

Con questo si trova che $\hat\theta$ ha **margine di errore** $z_{\alpha/2}\mathrm{SE}(\hat\theta)$, da cui si ricava che l'_intervallo di confidenza_ è:
$$
\begin{split}
A &= \hat\theta - z_{\alpha/2}\mathrm{SE}(\hat\theta) \\
B &= \hat\theta + z_{\alpha/2}\mathrm{SE}(\hat\theta)
\end{split}\; \Leftrightarrow\;
\hat\theta \pm z_{\alpha/2}\mathrm{SE}(\hat\theta)
$$

## Intervalli di verosimiglianza

Se la stima non avviene con la media $\bar{X}$, si può usare lo stimatore $\hat\theta$ di [massima verosimiglianza](../../02/02/README.md), perchè è **asintoticamente non distorto e normale** verso $\mathrm{N}(\theta, I(\theta)^{-1})$, per trovare l'_intervallo di confidenza_:
$$
\hat\theta \pm z_{\alpha/2}I(\hat\theta)^{-\frac{1}{2}}
\;\Leftrightarrow\;
\hat\theta \pm z_{\alpha/2}J(\hat\theta)^{-\frac{1}{2}}
$$

## Intervalli di trasformazioni

Dato uno stimatore $\theta$ con  _intervallo di confidenza_ $[A, B]$, una sua **trasformazione** $\psi = g(\theta)$ ha intervallo:
- $[g(A), g(B)]$ se $g$ è **crescente**, perchè $P(g(A) \leq g(\theta) \leq g(B)) = P(A \leq \theta \leq B)$
- $[g(B), g(A)]$ se $g$ è **decrescente**

Le **trasformazioni** possono anche migliorare l'approssimazione del _livello di confidenza_.
Per esempio, se $\theta$ è vicino a $0$, si può trovare l'intervallo $[A, B]$ di $\psi = \log\theta$ e poi si ritorna a $\theta = g(\psi) = e^\psi$ con $[e^A, e^B]$.
