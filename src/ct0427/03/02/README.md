# Proporzioni

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

## Differenza di proporzioni

Dati due campioni indipendenti di dimensione $n_1$ e $n_2$ e il parametro **differenza di proporzioni**
$$
\theta = p_1 - p_2\; \Rightarrow\; \hat\theta = \hat{p}_1 - \hat{p}_2
$$
si può ricavare l'_intervallo di confidenza_, come per la [media](../01/README.md#differenza-di-medie):
$$
(\hat{p}_1 - \hat{p}_2) \pm z_{\alpha/2}\sqrt{\frac{\hat{p}_1(1 - \hat{p}_1)}{n_1} + \frac{\hat{p}_2(1 - \hat{p}_2)}{n_2}}
$$

## Margine di errore

In questo caso **limitare** il _margine di errore_ a $\Delta$, porta la **dimensione** ad essere:
$$
z_{\alpha/2}\sqrt{\frac{\hat{p}(1 - \hat{p})}{n}} \leq \Delta\; \Rightarrow\; n \geq \underbrace{0.25}_{\hat{p}(1 - \hat{p})}\left(\frac{z_{\alpha/2}}{\Delta}\right)^2
$$
dove $0.25$ è il **valore massimo** assunto da $\hat{p}(1 - \hat{p})$ per $0 \leq \hat{p} \leq 1$.
