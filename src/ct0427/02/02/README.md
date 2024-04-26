# Verosimiglianza

La funzione di **verosimiglianza** indica la probabilità dell'aver osservato un certo _campione_ $X_1, ..., X_n$:
$$
L(\theta) \propto P(X_1 = x_1 \land ... \land X_n = x_n)
$$
se si ha un **problema di stima regolare**, ovvero se il dominio delle $X_i$ non dipende da $\theta$ e $L(\theta)$ è derivabile.

Dopo aver trovato $L(\theta)$ attraverso un **modello** sul _campione_ si può cercare il migliore valore di $\theta$ che la massimizza, e quindi adatta meglio il modello ai dati, con la derivata [prima](../../ct0432/05/04/README.md#studio-del-segno-della-derivata) e [seconda](../../ct0432/05/07/README.md#convessità-e-concavità-locale-e-punti-di-flesso):
$$
\frac{d}{d\theta}L(\theta) = 0\ \land\ \frac{d^2}{d^2\theta}L(\theta) < 0
$$

Quando il _campione_ è **indipendente** la _verosimiglianza_ diventa:
$$
L(\theta) \propto \prod_{i = 1}^n P(X_i = x_i)
$$
e quindi si preferisce usare la **log-verosimiglianza**, che semplifica il calcolo delle derivate:
$$
\ell(\theta) = \ln L(\theta) \propto \sum_{i = 1}^n \ln P(X_i = x_i)
$$

Nel caso **continuo**, $L(\theta) \propto f(x_1, ..., x_n) = \prod\limits_{i = 1}^n f(x_i)$ perchè $f(x_i)$ è l'area del [plurirettangolo](../../ct0432/10/03/README.md#integrali-definiti) con base $1$.

Per esempio, se $f(x) = \lambda e^{-\lambda x}$ si può stimare $\lambda$ con un _campione_ $X_1, ..., X_n$:
$$
\begin{split}
L(\lambda) &= \prod_{i = 1}^n \lambda e^{-\lambda x_i} = \lambda^n e^{-\lambda \sum\limits_{i = 1}^n x_i} \\
\ell(\lambda) &= n\ln\lambda - \lambda\sum_{i = 1}^n x_i \\
\ell'(\lambda) &= \frac{n}{\lambda} - \sum_{i = 1}^n x_i
\end{split}
\Rightarrow
\begin{cases}
\ell''(\lambda) = -\frac{n}{\lambda^2} < 0, \forall \lambda \\
\ell'(\lambda) = 0 \Leftrightarrow \hat{\lambda} = \frac{n}{\sum_{i = 1}^n x_i}
\end{cases}
$$

## Informazione attesa

Lo _stimatore di verosimiglianza_ è asintoticamente **non distorto**, **consistente** e asintoticamente **normale** su:
$$
\hat{\theta} \approx N(\theta, I(\theta)^{-1})
$$
dove $I(\theta)$ è detta l'**informazione attesa** di Fisher:
$$
I(\theta) = E(-\ell''(\theta)) = \sum_{i = 1}^n E\left(-\frac{d^2}{d\theta^2}\ln f(X_i)\right)
$$
che nei **campioni casuali semplici** diventa $I(\theta) = n E(-\frac{d^2}{d\theta^2}\ln f(X_1))$.

## Informazione osservata

L'_informazione attesa_ può essere **approssimata** con l'**informazione osservata**, cioè:
$$
J(\theta) = -\ell''(\theta)
$$
perchè per la [legge dei grandi numeri](../../ct0111/04/README.md#lln-e-clt) con l'aumentare di $n$ si ha che $J(\theta) \approx I(\theta)$.

La differenza è che $I(\theta)$ è una media di **più campioni** immaginari mentre $J(\theta)$ riflette **un campione** solo.

## Errore e efficienza

Dalla proprietà di _normalità_ si ricava che $\mathrm{Var}(\theta) = I(\theta)^{-1}$, quindi l'**errore standard** è:
$$
\mathrm{SE}(\hat\theta) \approx I(\theta)^{-1/2}
$$
per $n$ sufficientemente grande e, dato che $\theta$ non si conosce, può essere stimato da:
$$
\widehat{\mathrm{SE}}(\hat\theta) = I(\hat\theta)^{-1/2} = J(\hat\theta)^{-1/2}
$$

La **disuguaglianza di Cramer-Rao** identifica il limite a cui la varianza può arrivare:
$$
\mathrm{Var}(\hat\theta) \geq I(\theta)^{-1}
$$
infatti lo _stimatore di verosimiglianza_ è detto **asintoticamente efficiente** perchè $\mathrm{Var}(\hat\theta)$ è il meglio possibile.

Per esempio, considerato un campione $X_1, ..., X_n$ da una $X \sim \mathrm{Ber}(p)$:
$$
\begin{split}
L(p) &= \prod_{i = 1}^n p^{X_i}(1 - p)^{1 - X_i} \\
\ell(p) &= \ln p \sum_{i = 1}^n X_i + \ln (1 - p) \sum_{i = 1}^n (1 - X_i) \\
\ell'(p) &= \frac{1}{p}\sum_{i = 1}^n X_i + \frac{1}{1 - p}\sum_{i = 1}^n (1 - X_i) = \\
&= \frac{(1 - p)\sum\limits_{i = 1}^n X_i + p\sum\limits_{i = 1}^n (1 - X_i)}{p(1 - p)}
\end{split}
\Rightarrow
\begin{cases}
\ell''(p) = -\frac{1}{p^2}\sum\limits_{i = 1}^n X_i - \frac{1}{(1 - p)^2}\sum\limits_{i = 1}^n (1 - X_i) < 0, \forall p \\
\begin{split}
\ell'(p) = 0
&\Leftrightarrow \sum_{i = 1}^n X_i \cancel{- p\sum_{i = 1}^n X_i} - np + \cancel{p\sum_{i = 1}^n X_i} = 0 \\
&\Leftrightarrow \hat{p} = \frac{1}{n} \sum_{i = 1}^n X_i = \bar{X}
\end{split}
\end{cases} \\
\Downarrow \\
\begin{split}
J(\hat{p}) &= \frac{1}{\hat{p}^2}\overbrace{\sum_{i = 1}^n X_i}^{n\hat{p}} + \frac{1}{1 - \hat{p}^2}\left(n - \sum_{i = 1}^n X_i\right) = \\
&= \frac{n\hat{p}}{\hat{p}^2} + \frac{n - n\hat{p}}{(1 - \hat{p})^2} = \frac{n}{\hat{p}(1 - \hat{p})} \\
I(p) &= E(J(p)) = E(-\ell''(p)) = \\
&= \frac{1}{p^2}\sum_{i = 1}^n \underbrace{E(X_i)}_{p} + \frac{1}{(1 - p)^2}\sum_{i = 1}^n (1 - E(X_i)) = \\
&= \frac{np}{p^2} + \frac{n - np}{(1 - p)^2} = \frac{n}{p} + \frac{n}{1 - p} = \frac{n}{p(1 - p)}
\end{split} \\
\Downarrow \\
\widehat{\mathrm{SE}}(\hat{p}) = I(\hat{p})^{-1/2} = J(\hat{p})^{-1/2} = \sqrt{\frac{\hat{p}(1 - \hat{p})}{n}}
$$
dato che $J(\hat{p})$ è riducibile perchè si conosce $\hat{p}$, è stato necessario ricalcolare $E(J(p))$ per $I(p)$.

## Invarianza

Un'altra proprietà dello _stimatore di verosimiglianza_ è l'**invarianza**:
> Se $\hat{\theta}$ è lo _stimatore di massima verosimiglianza_ di $\theta$ allora anche $g(\hat{\theta})$ lo è di $\psi = g(\theta)$

Per esempio, data $X \sim \mathrm{Po}(\lambda)$ e $\hat\lambda = \bar{x} = 62.7$, allora:
$$
\psi = P(X > 75) = 1 - F(75)\ \approx\ \hat\psi = 1 - \hat{F}(75) = 1 - \sum_{x = 0}^{75} \frac{62.7^x}{x!}e^{-62.7}
$$
