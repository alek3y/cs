# Statistica Z

Basta che $\hat\theta$ sia **asintoticamente non distorto e normale**, per ricavare gli intervalli dalla **statistica Z**:
$$
Z = \frac{\hat\theta - \theta}{\mathrm{SE}(\hat\theta)}
$$
ovvero $\hat\theta$ [standardizzato](../../../ct0111/03/02/README.md#normale), trovando il **quantile** $z_{\alpha/2}$ di livello $\frac{\alpha}{2}$ con `qnorm(1 - 𝛼/2)`.

Con questo si trova che $\hat\theta$ ha **margine di errore** $z_{\alpha/2}\mathrm{SE}(\hat\theta)$, da cui si ricava che l'_intervallo di confidenza_ è:
$$
\begin{split}
A &= \hat\theta - z_{\alpha/2}\mathrm{SE}(\hat\theta) \\
B &= \hat\theta + z_{\alpha/2}\mathrm{SE}(\hat\theta)
\end{split}\; \Leftrightarrow\;
\hat\theta \pm z_{\alpha/2}\mathrm{SE}(\hat\theta)
$$

## Intervalli per la media

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

### Differenza di medie

Dati due campioni indipendenti $X_1, ..., X_n$ e $Y_1, ..., Y_m$ e il parametro **differenza di medie**
$$
\theta = \mu_X - \mu_Y\; \Rightarrow\; \hat\theta = \bar{X} - \bar{Y}
$$
si può ricavare l'_intervallo di confidenza_:
$$
(\bar{X} - \bar{Y}) \pm z_{\alpha/2}\sqrt{\frac{\sigma_X^2}{n} + \frac{\sigma_Y^2}{m}}
$$
dato che $\mathrm{Var}(\hat\theta) = \mathrm{Var}(\bar{X}) + \mathrm{Var}(\bar{Y})$.

### Margine di errore

La **dimensione** del campione permette di **limitare** il _margine di errore_ a $\Delta$, infatti:
$$
z_{\alpha/2}\frac{\sigma}{\sqrt{n}} \leq \Delta\; \Rightarrow\; n \geq \left(\frac{z_{\alpha/2}\sigma}{\Delta}\right)^2
$$
che sarà arrotondato per eccesso perchè $n \in \mathbb{N}$.

## Intervalli di proporzioni

Una situazione in cui $\sigma$ è ignota è una quella della stima di una **proporzione campionaria** di $X_1, ..., X_n$:
$$
\hat{p} = \frac{n_A}{n}
$$
dove $n_A$ è il numero di $X_i$ per cui la proprietà $A(X_i)$ è vera, allora si può dire che è una [Bernoulliana](../../../ct0111/03/01/README.md#di-bernoulli):
$$
X_i = \begin{cases}
1 & \text{se } A(X_i) \\
0 & \text{se } \neg A(X_i)
\end{cases}
$$
con $E(X_i) = p$ e $\mathrm{Var}(X_i) = p(1 - p)$, da cui si ha che $\hat{p} = \bar{X}$ è _non distorto_ e _asintoticamente normale_:
$$
\widehat{\mathrm{SE}}(\hat{p}) = \sqrt{\frac{\hat{p}(1 - \hat{p})}{n}} = \sqrt{\frac{\bar{X}(1 - \bar{X})}{n}}\; \Rightarrow\; \bar{X} \pm z_{\alpha/2}\sqrt{\frac{\bar{X}(1 - \bar{X})}{n}}
$$

### Differenza di proporzioni

Dati due campioni indipendenti di dimensione $n_1$ e $n_2$ e il parametro **differenza di proporzioni**
$$
\theta = p_1 - p_2\; \Rightarrow\; \hat\theta = \hat{p}_1 - \hat{p}_2
$$
si può ricavare l'_intervallo di confidenza_, come per la [media](../01/README.md#differenza-di-medie):
$$
(\hat{p}_1 - \hat{p}_2) \pm z_{\alpha/2}\sqrt{\frac{\hat{p}_1(1 - \hat{p}_1)}{n_1} + \frac{\hat{p}_2(1 - \hat{p}_2)}{n_2}}
$$

### Margine di errore

In questo caso **limitare** il _margine di errore_ a $\Delta$, porta la **dimensione** ad essere:
$$
z_{\alpha/2}\sqrt{\frac{\hat{p}(1 - \hat{p})}{n}} \leq \Delta\; \Rightarrow\; n \geq \underbrace{0.25}_{\hat{p}(1 - \hat{p})}\left(\frac{z_{\alpha/2}}{\Delta}\right)^2
$$
dove $0.25$ è il **valore massimo** assunto da $\hat{p}(1 - \hat{p})$ per $0 \leq \hat{p} \leq 1$.

## Intervalli di verosimiglianza

Se la stima non avviene con la media $\bar{X}$, si può usare lo stimatore $\hat\theta$ di [massima verosimiglianza](../../02/02/README.md), perchè è **asintoticamente non distorto e normale** verso $\mathrm{N}(\theta, I(\theta)^{-1})$, per trovare l'_intervallo di confidenza_:
$$
\hat\theta \pm z_{\alpha/2}I(\hat\theta)^{-\frac{1}{2}}
\;\Leftrightarrow\;
\hat\theta \pm z_{\alpha/2}J(\hat\theta)^{-\frac{1}{2}}
$$
