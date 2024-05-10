# Significatività osservata

Il **livello di significatività osservato**, o _$p$-value_, è la probabilità di ottenere un valore **più estremo** di quello trovato con la _statistica test_, ovvero il minimo _livello di significatività_ per poter rifiutare $H_0$.

In questo modo è possibile **non fissare** $\alpha$, perchè $p$ esprime **quanto estremo** è il valore trovato:

| Significatività osservata | Decisione su $H_0$ |
|:-:|:-:|
| $p < 0.001$ | **Rifiuto** netto |
| $0.001 \leq p < 0.01$ | **Rifiuto** |
| $0.01 \leq p < 0.05$ | **Rifiuto** debole |
| $0.05 \leq p < 0.1$ | Incerta |
| $p \geq 0.1$ | Non rifiutabile |

Il suo valore si può ricavare a seconda dell'_ipotesi alternativa_ $H_A$.
Sarà necessario trovare il valore della funzione di [ripartizione](../../../ct0111/03/README.md#funzione-di-ripartizione) $F$, che nel _test Z_ è $\Phi$ [della normale](../../../ct0111/03/02/README.md#normale) e nel _test T_ è $F_\nu$ dove $\nu$ sono i _gradi di libertà_:

| Test | Significatività osservata |
|-:|:-:|
| **Unilaterale destra** | $P(Z \geq z) = 1 - F(z)$ |
| **Unilaterale sinistra** | $P(Z \leq z) = F(z)$ |
| **Bilaterale** | $P(|Z| > |z|) = 2 - 2F(|z|)$ |

Nel caso l'_ipotesi nulla_ venga rifiutata, il test utilizzato si dice **significativo**.
