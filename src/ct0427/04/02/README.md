# Test T

Se $X \sim \mathrm{N}(\mu, \sigma^2)$ con $\mu$ e $\sigma^2$ **ignote**, il test che potrà valutare le ipotesi con $H_0: \mu = \mu_0$ è la [statistica T](../../03/02/README.md):
$$
T = \frac{\bar{X} - \mu_0}{\frac{S}{\sqrt{n}}}
$$

Le **regioni di  rifiuto** e di **accettazione** per $H_0$ saranno quindi:

| Test | $\mathcal{R}$ | $\mathcal{A}$ | Significatività $\alpha$ |
|-:|:-:|:-:|:-:|
| **Unilaterale destra** | $[t_\alpha, \infty)$ | $(-\infty, t_\alpha)$ | $P(T > t_\alpha)$ |
| **Unilaterale sinistra** | $(-\infty, -t_\alpha]$ | $(-t_\alpha, \infty)$ | $P(T < -t_\alpha)$ |
| **Bilaterale** | $(-\infty, -t_{\frac{\alpha}{2}}] \cup [t_{\frac{\alpha}{2}}, \infty)$ | $(-t_{\frac{\alpha}{2}}, t_{\frac{\alpha}{2}})$ | $P(T < -t_{\frac{\alpha}{2}}) + P(T > t_{\frac{\alpha}{2}})$ |

## Confronto tra campioni

Dati i campioni $X_1, ..., X_n$, $Y_1, ..., Y_m$ e l'ipotesi $H_0: \mu_X - \mu_Y = D$ come [differenza di medie](../../03/01/README.md#differenza-di-medie), se:
- $\sigma_X^2 = \sigma_Y^2$, allora con la [varianza pooled](../../03/02/README.md#intervalli-per-la-media) si ha
$$
T = \frac{\bar{X} - \bar{Y} - D}{S_p\sqrt{\frac{1}{n} + \frac{1}{m}}}
$$
- $\sigma_X^2 \neq \sigma_Y^2$, allora si ha
$$
T = \frac{\bar{X} - \bar{Y} - D}{\sqrt{\frac{S_X^2}{n} + \frac{S_Y^2}{m}}}
$$
