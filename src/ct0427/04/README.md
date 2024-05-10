# Verifica delle ipotesi

Nella verifica delle ipotesi si considera il **sistema d'ipotesi** composto da:
- **Ipotesi nulla**, $H_0$: rappresenta lo _status quo_
- **Ipotesi alternativa**, $H_A$: rappresenta l'alternativa che si vuole dimostrare che invaliderebbe $H_0$

La **valutazione** del _sistema_ avviene tramite un **test**, che difende l'_ipotesi nulla_ fino a **prova contraria**.

L'_ipotesi nulla_ $H_0$ sarà una [relazione d'ordine](../../ct0434/03/01/README.md) tra il parametro $\theta$ e un $\theta_0 \in \mathbb{R}$.
L'opposto, ovvero $H_A$, sarà:
- **Bilaterale**, con test **a due code**: se $H_A: \theta \neq \theta_0$
- **Unilaterale sinistra**, con test **ad una coda**: se $H_A: \theta < \theta_0$
- **Unilaterale destra**, con test **ad una coda**: se $H_A: \theta > \theta_0$

Per esempio, se una connessione ha velocità media dichiarata $H_0: \mu = 54$ (_Mbps_) allora l'_ipotesi alternativa_ può essere $H_A: \mu \neq 54$ se la si vuole dimostrare o $H_A: \mu < 54$ se basta verificare che non scenda.

La **statistica test** si ricava da una trasformazione di $\hat\theta$, nella cui distribuzione si trovano una **regione di accettazione** $\mathcal{A}$, per cui si accetta $H_0$, e una **regione di rifiuto** $\mathcal{R}$, per cui si rifiuta $H_0$.

## Errori

La scelta del campione può portare ad un **errore campionario** che causa conclusioni sbagliate:

| | Rifiuto $H_0$ | Non rifiuto $H_0$ |
|:-:|:-|:-|
| **$H_0$ è vera** | Errore del **1° tipo** | Corretto |
| **$H_0$ è falsa** | Corretto | Errore del **2° tipo** |

Per stimare la correttezza della _valutazione_ si può trovare:
- $\alpha$, ovvero il **livello di significatività**, cioè la probabilità di **commettere** un errore del _1° tipo_
- $\beta(\theta)$, ovvero la **potenza del test**, cioè la probabilità di **non commettere** un errore del _2° tipo_

Di conseguenza, dato un _test_ $\mathcal{T}$, il _livello di significatività_ è **esprimibile** come $\alpha = P(\mathcal{T} \in \mathcal{R} | H_0)$.

## Intervalli di confidenza

Dato un test $\mathcal{T}$ e un'_ipotesi alternativa_ **bilaterale**, è possibile concludere che il test **non rifiuta** $H_0$ con _significatività_ $\alpha$ sse l'[intervallo di confidenza](../03/README.md) di $\theta$ con _livello di confidenza_ $1 - \alpha$ **contiene** $\theta_0$.

Per esempio, dato il campione $2.5, 7.4, 8.0, 4.5, 7.4, 9.2$ e $H_0: \mu = 6$ contro $H_A: \mu \neq 6$, l'_intervallo di confidenza_ di livello $95\%$ è $[4.74, 8.26]$ e quindi $H_0$ non è rifiutabile perchè $\mu_0 = 6$ ci rientra.
