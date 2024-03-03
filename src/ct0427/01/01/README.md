# Stimatori di posizione

Le _statistiche_ che misurano la **posizione** servono a stimare un punto della distribuzione della _popolazione_.

Come esempio, un campione $X_1, ..., X_n$ di $n = 30$ processi dei loro tempi di esecuzione può essere:
$$
\begin{matrix}
9 & 15 & 19 & 22 & 24 & 25 & 30 & 34 & 35 & 35 \\
36 & 36 & 37 & 38 & 42 & 43 & 46 & 48 & 54 & 55 \\
56 & 56 & 59 & 62 & 69 & 70 & 82 & 82 & 89 & 139
\end{matrix}
$$
per cui $X$ è la variabile casuale del tempo di un processo casuale di cui si vuole studiare la distribuzione.

## Media campionaria

La [media](../../../ct0111/03/README.md#valore-atteso) $E(X)$ si stima con la [media campionaria](../../../ct0111/04/README.md#media-campionaria) su variabili i.i.d. $X_i$ con $E(X_i) = \mu$ e $\mathrm{Var}(X_i) = \sigma^2$:
$$
\bar{X} = \frac{1}{n} \sum_{i = 1}^n X_i
$$
che è uno _stimatore_ **non distorto** di $\mu$, perchè $E(\bar{X}) = \mu$, **consistente**, per la _legge dei grandi numeri_, ed è quindi anche **asintoticamente normale** per il [teorema del limite centrale](../../../ct0111/04/README.md#lln-e-clt).

Nell'esempio, $\bar{x} = \hat\mu = \hat\theta = 48.23$ è una _stima_ di $\mu = \theta$ mentre $\bar{X} = \hat\mu = \hat\theta$ è lo _stimatore_.

Il suo **errore standard** sarebbe $\mathrm{SE}(\bar{X}) = \frac{\sigma}{\sqrt{n}}$, e può essere quindi stimato con:
$$
\widehat{\mathrm{SE}}(\bar{X}) = \frac{S}{\sqrt{n}}
$$
dove $S$ è la [deviazione standard campionaria](../02/README.md#deviazione-standard-campionaria), e come lei è **consistente** e **asintoticamente normale**.

## Mediana campionaria

La [mediana](../../../ct0111/03/README.md#mediana) $M$ ha **sensibilità ridotta** rispetto alla _media_, dato che non dipende da poche osservazioni.

Lo _stimatore_ su un campione $X_1, ..., X_n$ **ordinato** è la **mediana campionaria**, cioè:
$$
\hat{M} = \begin{cases}
X_{(n+1)/2} & \text{se } n \text{ dispari} \\
(X_{n/2} + X_{n/2 + 1}) / 2 & \text{se } n \text{ pari}
\end{cases}
$$

Come la _media_ è **non distorta**, **consistente** e **asintoticamente normale**.

Nell'esempio, $\hat{M} = \frac{42 + 43}{2} = 42.5$.

## Quantile campionario

Il [quantile](../../../ct0111/03/README.md#quantili) $q_p$ di ordine $p$ si stima con il **quantile campionario** $\hat{q}_p$, cioè una $X_i$ del campione **ordinato** $X_1, ..., X_n$ che supera fino a $np$ elementi e ne precede fino a $n(1 - p)$.

Il _quantile_ di ordine $0.01\gamma$ viene detto **percentile** di ordine $\gamma$.

Nell'esempio $q_{1/4} = 34$ perchè segue $\lfloor 30 \cdot 0.25 \rfloor = 7$ elementi e ne precede $\lfloor 30 \cdot (1 - 0.25) \rfloor = 22$.
