# Serie

Una serie è la somma di tutti i termini di una successione:
$$\sum_{n = 0}^{+\infty} a_n = \lim_{n \to +\infty} s_n$$
dove $(s_n)_{n \in \mathbb{N}}$ è una successione di **somme parziali**.

Una **somma parziale** $s_n$ di $a_n$ corrisponde a:
$$s_n = a_0 + a_1 + a_2 + ... + a_n$$
quindi la somma dei termini fino ad $n$ (e.g. $s_2 = a_0 + a_1 + a_2$).

## Carattere di una serie

Avendo la serie:
$$\sum_{n = 0}^{+\infty} a_n = l$$
il **carattere** è stabilito da:
- $l \in \mathbb{R}$, allora è **convergente**
- $l = \pm\infty$, allora è **divergente**
- $\nexists \lim_{x \to +\infty} s_n$, allora è **indeterminata**

## Serie telescopica

Una serie **telescopica** si dice tale se, di tutti i termini, rimane parte del primo e dell'ultimo, come nel caso della **serie di Mengoli**, che corrisponde a:
$$
\begin{split}
& \sum_{n = 1}^{+\infty} \frac{1}{n(n+1)} =
\left(1 - \cancel{\frac{1}{2}}\right) +
\left(\cancel{\frac{1}{2}} - \cancel{\frac{1}{3}}\right) + ... +
\left(\cancel{\frac{1}{n}} - \frac{1}{n+1}\right) + ... = \\
=& \sum_{n = 1}^{+\infty} \left(\frac{1}{n} - \frac{1}{n+1}\right) =
\lim_{n \to +\infty} \left(1 - \frac{1}{n+1}\right) = 1
\end{split}
$$

## Serie geometrica

Avendo la _somma parziale_,
$$s_n = \sum_{i = 0}^n q^i = \frac{1 - q^{n+1}}{1 - q}$$
allora la **serie geometrica** sarà:
$$
\sum_{n = 0}^{+\infty} q^n = \lim_{n \to +\infty} s_n =
\begin{cases}
\frac{1}{1 - q} & \text{se } |a| < 1 \\
+\infty & \text{se } a \geq 1 \\
\nexists & \text{se } a \leq -1
\end{cases}
$$

## Serie a termini di segno costanti

Sia $(a_n)_{n \in \mathbb{N}}$ una successione con tutti i termini $a_n > 0$ oppure tutti $a_n < 0$, allora:
$$(s_n) \text{ è monotona} \Rightarrow \sum_n a_n \text{ non è indeterminata}$$

## Criteri di convergenza a segno costante

### Condizione necessaria per la convergenza

Se una serie $\sum_n a_n$ **converge**, allora si può dire che $a_n$ è **infinitesimo**, cioè $\lim_{n \to +\infty} a_n = 0$.
Sapendo che $a_n$ è infinitesimo però, non si può dire se la serie converge o no.

### Criterio del confronto

Siano $(a_n)$ e $(b_n)$ due successioni tali che $0 < a_n \leq b_n$, allora in base all'andamendo di una si può ricavare l'andamento dell'altra:
- $\sum_n b_n$ **converge**, allora anche $\sum_n a_n$ converge
- $\sum_n a_n$ **diverge**, allora anche $\sum_n b_n$ diverge

### Algebra delle serie

Sapendo che,
$$\sum_n a_n = l \land \sum_n b_n = m$$
sono due serie **convergenti**, allora:
$$\sum_n (\lambda a_n + \beta b_n) = \lambda l + \beta m$$
con $\lambda, \beta \in \mathbb{R}$.

### Serie armonica generalizzata

La serie
$$\sum_{n = 1}^{+\infty} \frac{1}{n^\alpha},\; \alpha \in \mathbb{R}$$
**converge** se $\alpha > 1$ e **diverge** se $0 < \alpha \leq 1$, di conseguenza si può pensare a
$$\sum_{n = 1}^{+\infty} \frac{1}{n}$$
come ad un separatore tra serie armoniche che _convergono_ e quelle che _divergono_.

### Criterio del rapporto e della radice

Avendo una serie $\sum_n a_n$ a **termini positivi** e
$$\lim_{n \to +\infty} \frac{a_{n+1}}{a_n} = l \lor \lim_{n \to +\infty} \sqrt[n]{a_n} = l$$
allora è possibile determinare il **carattere** della serie:
- $l < 1$, la serie **converge**
- $l > 1$, la serie **diverge**
- $l = 1$, nulla è certo

## Criteri di convergenza a segno qualsiasi

### Serie assolutamente convergente

Quando la serie $\sum_n a_n$ è **assolutamente convergente**, e quindi
$$\sum_n |a_n| \text{ converge} \Rightarrow \sum_n a_n \text{ converge}$$

### Criterio di Leibniz

Se $\sum_n a_n$ è una serie a termini di **segno alterno**, allora:
$$\lim_{n \to +\infty} a_n = 0 \land |a_{n+1}| < |a_n| \Rightarrow \sum_n a_n \text{ converge}$$
cioè se $a_n$ è infinitesimo.
