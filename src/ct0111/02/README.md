# Probabilità condizionata

La probabilità di un evento $A$ dato l'evento **avvenuto** $B$ è:
$$
P(A|B) = \frac{P(A \cap B)}{P(B)}
$$
che considera l'evento $A$ all'interno dell'**ipotetico universo** $B$, _rimpiazzandolo_ a $\Omega$.

Per esempio data un'urna $\{N_1, N_2, B_1, B_2, B_3\}$, la probabilità di scegliere $N_1$ è $P(A) = \frac{1}{5}$ ma sapendo di aver scelto una pallina nera, la probabilità diventa $P(A|B) = \frac{1}{2}$.

Le probabilità **condizionate** allo stesso evento $B$ obbediscono agli [assiomi](../01/README.md#definizione), e.g. $P(\overline{A}|B) = 1 - P(A|B)$.

Da questo si può ricavare la **probabilità composta** di più eventi:
$$
P(A_1 \cap A_2 \cap A_3 \cap ... \cap A_n) = P(A_1) \cdot P(A_2 | A_1) \cdot P(A_3 | A_1 \cap A_2) \cdot ... \cdot P(A_n | A_1 \cap ... \cap A_{n-1})
$$

Per esempio data l'urna $\{2 \cdot N, 3 \cdot B\}$, la probabilità di scegliere $N$ per primo, i.e. $N_1$, e di scegliere $B$ per secondo, i.e. $B_2$, sarà $P(N_1 \cap B_2) = P(N_1)P(B_2|N_1) = \frac{2}{5}\frac{3}{4} = \frac{3}{10}$.

Si può quindi estendere la [**probabilità totale**](../01/README.md#definizione), dove $C_1, ..., C_n$ è una partizione di $\Omega$:
$$
P(A) = \sum_{i = 1}^n P(A \cap C_i) = \sum_{i = 1}^n P(C_i)P(A|C_i)
$$

## Eventi indipendenti

Tramite la _probabilità condizionata_ è possibile determinare se due eventi $A$ e $B$ sono **indipendenti**, se:
$$
P(A \cap B) = P(A)P(B)
$$

Più eventi $A_1, ..., A_n$ invece, sono **reciprocamente indipendenti** se ogni loro [combinazione](../README.md) è _indipendente_.

Per esempio, $P(N_1 \cap B_2) = \frac{3}{10} \neq P(N_1)P(B_2) = \frac{2}{5}\frac{3}{5} = \frac{6}{25}$ e quindi $N_1$ e $B_2$ non sono indipendenti.

## Teorema di Bayes

Grazie alla **formula di Bayes** è possibile **aggiornare** la probabilità di un evento $A$ ipotizzando $B$ come avvenuto, prima che avvenga realmente.
Questa operazione ci risulta nella nuova probabilità _a posteriori_:
$$
P(A|B) = \frac{P(B|A)P(A)}{P(B)}
$$

Che può essere espresso in termini della _probabilità totale_ come:
$$
P(C_k|A) = \frac{P(A|C_k)P(C_k)}{\sum\limits_{i = 1}^n P(A \cap C_i)} = \frac{P(A|C_k)P(C_k)}{\sum\limits_{i = 1}^n P(C_i)P(A|C_i)}
$$
dove $C_i$ è un elemento di una partizione di $\Omega$.

### Esempio

Dato un sistema anti-spam basato su _parole chiave_ e gli eventi
$$
\begin{split}
S &= \text{"il messaggio è spam"} \\
W &= \text{"il messaggio contiene parole chiave"}
\end{split}
$$
si vuole trovare la probabilità che un messaggio sia _spam se contiene certe parole chiave_.

Stimando $P(S) = \frac{1}{2}$, e ricavando dai _dati accumulati_ $P(W|S) \approx \frac{250}{2000}$ e $P(W|\overline{S}) \approx \frac{5}{1000}$, allora:
$$
P(S|W) = \frac{P(W|S)P(S)}{P(W)} = \frac{P(W|S)P(S)}{P(W \cap S) + P(W \cap \overline{S})} =
\frac{P(W|S)P(S)}{P(S)P(W|S) + P(\overline{S})P(W|\overline{S})} = 0.962
$$
