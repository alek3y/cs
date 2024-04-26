# Media

Su $\bar{X}$ la _statistica Z_ si può usare se è **normale** o se $n$ è **grande**, per il [teorema del limite centrale](../../../ct0111/04/README.md#lln-e-clt), per cui:
$$
\bar{X} \pm z_{\alpha/2}\frac{\sigma}{\sqrt{n}}
$$
dato che $\mathrm{SE}(\hat\mu) = \sqrt{\frac{\sigma^2}{n}}$.

Inoltre, quando $\sigma$ è ignoto, per $n$ grande è possibile calcolare il **livello approssimato** $1 - \alpha$ con:
$$
\hat\theta \pm z_{\alpha/2} \widehat{\mathrm{SE}}(\hat\theta)
$$
che per la _media campionaria_ sarà $\bar{X} \pm z_{\alpha/2}\frac{S}{\sqrt{n}}$ e per la _differenza di medie_ $\bar{X} - \bar{Y} \pm z_{\alpha/2}\sqrt{\frac{S_X^2}{n} + \frac{S_Y^2}{m}}$.

## Differenza di medie

Dati due campioni indipendenti $X_1, ..., X_n$ e $Y_1, ..., Y_m$ e il parametro **differenza di medie**
$$
\theta = \mu_X - \mu_Y\; \Rightarrow\; \hat\theta = \bar{X} - \bar{Y}
$$
si può ricavare l'_intervallo di confidenza_:
$$
(\bar{X} - \bar{Y}) \pm z_{\alpha/2}\sqrt{\frac{\sigma_X^2}{n} + \frac{\sigma_Y^2}{m}}
$$
dato che $\mathrm{Var}(\hat\theta) = \mathrm{Var}(\bar{X}) + \mathrm{Var}(\bar{Y})$.

## Margine di errore

La **dimensione** del campione permette di **limitare** il _margine di errore_ a $\Delta$, infatti:
$$
z_{\alpha/2}\frac{\sigma}{\sqrt{n}} \leq \Delta\; \Rightarrow\; n \geq \left(\frac{z_{\alpha/2}\sigma}{\Delta}\right)^2
$$
che sarà arrotondato per eccesso perchè $n \in \mathbb{N}$.
