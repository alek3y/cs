# Stimatori di variabilità

In questo caso le _statistiche_ permettono di stimare la **variabilità** della _popolazione_.

Come esempio, si considera il campione [precedente](../01/README.md).

## Varianza campionaria

La [varianza](../../../ct0111/03/README.md#varianza) $\mathrm{Var}(X)$ si stima con la **varianza campionaria**, cioè:
$$
S^2 = \frac{1}{n - 1} \sum_{i = 1}^n (X_i - \bar{X})^2 = \frac{1}{n-1}\left(\left(\sum_{i = 1}^n X_i^2\right) - n\bar{X}^2\right)
$$
dove $X_i - \bar{X}$ è detto **scarto dalla media**, ovvero la distanza.

Come _stimatore_ è **non distorto**, **consistente** e **asintoticamente normale**.

Nell'esempio $s^2 \approx \frac{20391}{29} \approx 703.2 \text{ sec}^2$.

## Deviazione standard campionaria

La **deviazione standard** $\sigma = \sqrt{\mathrm{Var}(X)}$ si stima con la **deviazione standard campionaria**, cioè:
$$
S = \sqrt{S^2}
$$
attraverso la _varianza campionaria_, e come lei è **consistente**, **asintoticamente normale** ma **distorta**.

Nell'esempio $s \approx \sqrt{703.2} \approx 26.5 \text{ sec}$.

## Scarto interquartile campionario

Come la [media](../01/README.md#media-campionaria), la _varianza_ è **sensibile** a poche osservazioni estreme, per cui si usa lo **scarto interquartile**:
$$
IQR = Q_3 - Q_1 = q_{3/4} - q_{1/4}
$$
dove $Q_3$ e $Q_1$ sono il primo e il terzo **quartile**, ovvero [quantili](../01/README.md#quantile-campionario) di ordine $\frac{1}{4}$ e $\frac{3}{4}$.

La _stima_ si trova con lo **scarto interquartile campionario**, cioè:
$$
\widehat{IQR} = \hat{Q}_3 - \hat{Q}_1 = \hat{q}_{3/4} - \hat{q}_{1/4}
$$
e si usa anche per trovare i **valori anomali** (o _outliers_), cioè quei valori $X_i$ tali che:
$$
X_i \not\in \left[\hat{Q}_1 - 1.5\widehat{IQR},\ \hat{Q}_3 + 1.5\widehat{IQR}\right]
$$
e vanno **rimossi** se sono errori o osservazioni di un'altra _popolazione_.

Nell'esempio $\widehat{IQR} = 59 - 34 = 25$ e un _valore anomalo_ è $139 \not\in [34 - 37.5, 59 + 37.5] = [-3.5, 96.5]$.
