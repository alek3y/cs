# Test Z

Quando l'_ipotesi nulla_ è $H_0: \theta = \theta_0$, una **statistica test** che valuta le ipotesi potrà essere la [statistica Z](../../03/01/README.md):
$$
Z = \frac{\hat\theta - \theta_0}{\mathrm{SE}(\hat\theta)}
$$

A seconda dell'_ipotesi alternativa_ $H_A$ si ottengono diverse **regioni di rifiuto** e **accettazione** per $H_0$:

| Test | $\mathcal{R}$ | $\mathcal{A}$ | Significatività $\alpha$ |
|-:|:-:|:-:|:-:|
| **Unilaterale destra** | $[z_\alpha, \infty)$ | $(-\infty, z_\alpha)$ | $P(Z > z_\alpha)$ |
| **Unilaterale sinistra** | $(-\infty, -z_\alpha]$ | $(-z_\alpha, \infty)$ | $P(Z < -z_\alpha)$ |
| **Bilaterale** | $(-\infty, -z_{\frac{\alpha}{2}}] \cup [z_{\frac{\alpha}{2}}, \infty)$ | $(-z_{\frac{\alpha}{2}}, z_{\frac{\alpha}{2}})$ | $P(Z < -z_{\frac{\alpha}{2}}) + P(Z > z_{\frac{\alpha}{2}})$ |

Per esempio, dato il campione $X_1, ..., X_{100}$ con $\bar{X} \sim \mathrm{N}(5200, \frac{800^2}{100})$ si vuole verificare che l'_ipotesi nulla_ $H_0: E(X) = \mu = 5000$ sia valida, contro $H_A: \mu > 5000$, con significatività $\alpha = 0.05$:
$$
z = \frac{5200 - 5000}{\frac{800}{\sqrt{100}}} = 2.5
$$
che rientra nella _regione di rifiuto_ $\mathcal{R}$, perchè $z_\alpha = 1.645$ ricavato da `qnorm(1-0.05)`.

## Stima di verosimiglianza

La statistica Z si può costruire anche con lo stimatore $\hat\theta$ di [massima verosimiglianza](../../02/02/README.md), infatti:
$$
Z = \frac{\hat\theta - \theta_0}{J(\theta_0)^{-1/2}} \approx \frac{\hat\theta - \theta_0}{J(\hat\theta)^{-1/2}}
$$

Per esempio, data $f(x) = \theta(1 - x)^{\theta - 1}$ e $\sum\limits_{i = 1}^{300} \ln(1 - x_i) = -291.4$ si vuole valutare l'ipotesi $H_0: \theta = 1.2$ contro $H_A: \theta < 1.2$.
Dalla massima verosimiglianza si trovano $\hat\theta = 1.03$ e $J(\theta_0) = \frac{n}{\theta_0^2} = \frac{300}{1.2^2}$, quindi:
$$
z = \frac{1.03 - 1.2}{\frac{1.2}{\sqrt{300}}} = -2.45
$$
che rientra nella _regione di rifiuto_ $\mathcal{R} = (-\infty, -1.645]$ con significatività $\alpha = 0.05$.
