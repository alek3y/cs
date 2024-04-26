# Statistica T

Si dice **distribuzione T** con $n-1$ **gradi di libertà**, la [standardizzazione](../../../ct0111/03/02/README.md#normale) di $\hat\mu = \bar{X}$ stimata con $\widehat{\mathrm{SE}}(\hat\mu) = \frac{S}{\sqrt{n}}$:
$$
T = \frac{\bar{X} - \mu}{\frac{S}{\sqrt{n}}} = \frac{\sqrt{n}(\bar{X} - \mu)}{S}
$$

La distribuzione è vincolata da $n-1$ _gradi di libertà_, che originano dalle informazioni usate dalla [stima della varianza](../../01/02/README.md#varianza-campionaria) $S^2$ e che, al loro aumento, portano la distribuzione a convergere a $\mathrm{N}(\mu, \sigma^2)$.

## Intervalli per la media

L'**intervallo di confidenza** per la media $\mu$ di una popolazione _normale_ con la **statistica T** è:
$$
\bar{X} \pm t_{\alpha/2}\frac{S}{\sqrt{n}}
$$
dove $t_{\alpha/2}$ è il **quantile** della _distribuzione T_ con $n-1$ _gradi di libertà_ ricavabile da `qt(1 - 𝛼/2, df=n-1)`.

Nel caso della [differenza di medie](../01/README.md#differenza-di-medie), il metodo per trovare l'_intervallo di confidenza_ **dipende dalle varianze**:
- $\sigma_X^2 = \sigma_Y^2 = \sigma^2$, allora:

	$$
	(\bar{X} - \bar{Y}) \pm t_{\alpha/2}S_p\sqrt{\frac{1}{n} + \frac{1}{m}}
	$$
	dove $t_{\alpha/2}$ deriva dalla _distribuzione T_ con $n+m-2$ _gradi di libertà_, quindi `qt(1 - 𝛼/2, df=n+m-2)`.

	La stima di $\sigma^2$ si ottiene con la **varianza campionaria pooled** $S_p^2$, cioè combinata, di $X$ e $Y$:
	$$
	\hat{\sigma}^2 = S_p^2 = \frac{1}{n + m - 2}\left(\sum_{i = 1}^n (X_i - \bar{X})^2 + \sum_{i = 1}^m (Y_i - \bar{Y})^2\right)
	$$

- $\sigma_X^2 \neq \sigma_Y^2$, allora la _statistica T_

	$$
	T = \frac{(\bar{X} - \bar{Y}) - (\mu_X - \mu_Y)}{\sqrt{\frac{S_X^2}{n} + \frac{S_Y^2}{m}}}
	$$
	non ha più _distribuzione T_, ma la si può approssimare con i **gradi di libertà di Satterthwaite**:
	$$
	\nu = \frac{\left(\frac{S_X^2}{n} + \frac{S_Y^2}{m}\right)^2}{\frac{S_X^4}{n^2(n-1)} + \frac{S_Y^4}{m^2(m-1)}}
	$$
	con cui si ottiene l'_intervallo di confidenza_:
	$$
	(\bar{X} - \bar{Y}) \pm t_{\alpha/2}\sqrt{\frac{S_X^2}{n} + \frac{S_Y^2}{m}}
	$$
	dove $t_{\alpha/2}$ deriva dalla _distribuzione T_ con $\nu$ _gradi di libertà_, quindi `qt(1 - 𝛼/2, df=ν)`.
