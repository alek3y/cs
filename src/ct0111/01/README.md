# Probabilità elementare

Nella _probabilità_, un insieme $\Omega$ è detto **spazio campionario** se contiene tutti i risultati di un esperimento.
L'insieme $A$ è detto **evento** se $A \subseteq \Omega$, infatti è **evento certo** se $A = \Omega$ e **evento elementare** se $|A| = 1$.

Vengono chiamati **eventi incompatibili** invece, se
$$
A_i \cap A_j = \emptyset,\ \forall i, j = 1, ..., n : i \neq j
$$
ovvero se nessuno ha intersezione.

## Definizione

La **probabilità** è definita come una _funzione_ che assegna ad ogni _evento_ un valore in $\mathbb{R}^\geq$, per cui si ha:
1. **Positività**: $0 \leq P(A) \leq 1$

2. **Normalizzazione**: $P(\Omega) = 1$

3. **Additività**: $P\left(\bigcup\limits_{i = 1}^n A_i\right) = \sum\limits_{i = 1}^n P(A_i)$ se $A_i$ sono _incompatibili_

Per cui per la **legge della probabilità totale**, si può **spezzettare** $A$ in eventi _incompatibili_ attraverso delle [partizioni](../../ct0434/02/README.md#partizioni) $C_i$ di $\Omega$ semplificando quindi il calcolo della _probabilità_:
$$
P(A) = \sum_{i = 1}^{n} P(A \cap C_i)
$$

### Proprietà

Come conseguenze alla definizione di _probabilità_ si ottengono:
- Probabilità **complementare**:

	$$P(\overline{A}) = 1 - P(A)$$
	per _additività_ e _normalizzazione_, perchè $P(\Omega) = P(A \cup \overline{A}) = P(A) + P(\overline{A})$, rappresenta la probabilità totale $P(\Omega)$ meno quella del singolo evento $P(A)$ che non ci interessa.

- Probabilità dell'**evento impossibile**:

	$$P(\emptyset) = 0$$
	perchè $P(\emptyset) = P(\overline\Omega) = 1 - P(\Omega) = 0$.

- Probabilità di una **partizione** $C_1, C_2, ..., C_n$:

	$$P\left(\bigcup_{i = 1}^{n} C_i\right) = P(\Omega) = 1$$

- Probabilità dell'**unione**:

	$$P(A \cup B) = P(A) + P(B) - P(A \cap B)$$
	che è esteso _alternando segno_ sulle intersezioni degli elementi nei sottoinsiemi dell'[insieme delle parti](../../ct0434/02/README.md#insiemi-delle-parti).

- Probabilità degli **eventi equiprobabili**:

	$$P(A) = \frac{|A|}{|\Omega|} \Leftrightarrow P(\{w\}) = \frac{1}{|\Omega|}, \forall w \in \Omega$$
	in cui $|A|$ è il numero di _casi favorevoli_, mentre $|\Omega|$ è il numero di _casi possibili_.

## Estrazione

In un'urna con $N$ palline ci sono $m$ **successi** (i.e. palline che ci interessano) e $N - m$ **insuccessi**.

La probabilità di _estrarre_ $n$ su $N$ palline **reinserendole**[^1], di cui $k$ _successi_ e $n - k$ _insuccessi_, corrisponderà a:
$$
P(A_k) = {n \choose k} \left(\frac{m}{N}\right)^k \left(1 - \frac{m}{N}\right)^{n - k}
$$
dove $\frac{m}{N}$ sarà la probabilità di $1$ _successo_ e $\left(\frac{m}{N}\right)^k \left(1 - \frac{m}{N}\right)^{n - k}$ di **esattamente** $k$ _successi_ e $n - k$ _insuccessi_.
Questa probabilità è poi ripetuta ${n \choose k}$ volte, ovvero per tutti i modi in cui si possono estrarre $k$ successi da $n$.

Se invece si vuole **senza il reinserimento**[^2], diventerà:
$$
P(A_k) = \frac{{m \choose k} {N-m \choose n-k}}{N \choose n}
$$
dove ${m \choose k}$ è il numero di modi di scegliere i _successi_, ${N-m \choose n-k}$ gli _insuccessi_ e ${N \choose n}$ i modi di tutti gli $n$ da $N$.

[^1]: [Distribuzione binomiale](https://it.wikipedia.org/wiki/Distribuzione_binomiale)

[^2]: [Distribuzione ipergeometrica](https://it.wikipedia.org/wiki/Distribuzione_ipergeometrica)
