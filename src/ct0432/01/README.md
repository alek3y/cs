# Insiemi

Gli insiemi sono **collezioni non ordinate** di elementi (i.e. $\{1, 2, 3\} = \{3, 1, 2\}$), e vengono indicati con le lettere maiuscole (e.g. $A$, $B$, ...), mentre i suoi elementi vegono indicati come variabili a lettere minuscole (e.g. $a$, $b$, ...).

Due insiemi si dicono **uguali** se:
$$A = B \Leftrightarrow A \subseteq B \land B \subseteq A$$
e quindi se $\forall x, x \in A \land x \in B$.

Gli insiemi possono essere rappresentati in due modi in particolare:
- Rappresentazione **estensiva**: vengono elencati tutti gli elementi; e.g. $A = \{a, b\} \Rightarrow c \not\in A$
- Rappresentazione **intensiva**: vengono descritti attraverso una proprietà o vengono sottointesi; e.g. $A = \{1, 2, 3, 4, ...\} \Rightarrow 5 \in A$

## Operazioni

- **Unione**:
	$$A \cup B = \{x | x \in A \lor x \in B\}$$

	che avrà cardinalità $|A \cup B| = |A| + |B| - |A \cap B|$.

- **Intersezione**:
	$$A \cap B = \{x | x \in A \land x \in B\}$$

	che sarà $\emptyset$ quando i due insiemi sono _disgiunti_.

- **Complementare**:
	$$A^c = \overline A = \{x | x \not\in A \land x \in B \supseteq A\}$$

	dove $B$ è il dominio di $A$.

- **Differenza**:
	$$A \setminus B = \{x | x \in A \land x \not\in B\}$$

- **Differenza simmetrica**:
	$$A \triangle B = (A \setminus B) \cup (B \setminus A) = (A \cup B) \setminus (A \cap B)$$

	cioè l'unione degli elementi che non sono in comune tra $A$ e $B$.

- **Prodotto cartesiano**:
	$$A \times B = \{(a, b) | a \in A \land b \in B\}$$

	che avrà cardinalità $|A \times B| = |A| \cdot |B|$.

	Per esempio $\{1, 2\} \times \{1, 2\} = \{(1, 1), (1, 2), (2, 1), (2, 2)\}$, oppure $\mathbb{R}^2$ che consiste nelle coppie $(x, y)$ del piano cartesiano.

## Insiemi specifici

- $\mathbb{N} = \{0, 1, 2, ...\}$
- $\mathbb{Z} = \{..., -2, -1, 0, 1, 2, ...\}$
- $\mathbb{Q} = \{\frac{p}{q} | p \in \mathbb{Z} \land q \in \mathbb{Z} \setminus \{0\}\}$, tra cui i numeri periodici
- $\mathbb{I} = \mathbb{R} - \mathbb{Q}$, che contiene numeri con parte decimale infinita e non periodica

## Proprietà

Siano $A$, $B$, $C$ insiemi tali che $A, B, C \subseteq U$, dove $U$ è l'insieme **universo**, allora varranno le seguenti proprietà:

- **Idempotenza**:
	1. $$A \cup A = A, \hspace{1em} A \cap A = A$$
	2. $$A \cup \emptyset = A, \hspace{1em} A \cap \emptyset = \emptyset$$
	3. $$A \cup U = U, \hspace{1em} A \cap U = A$$

- **Commutativa**:
	$$A \cup B = B \cup A, \hspace{1em} A \cap B = B \cap A$$

- **Associativa**:
	$$A \cup (B \cup C) = A \cup B \cup C, \hspace{1em} A \cap (B \cap C) = A \cap B \cap C$$

- **Distributiva**:
	$$A \cup (B \cap C) = (A \cup B) \cap (A \cup C), \hspace{1em} A \cap (B \cup C) = (A \cap B) \cup (A \cap C)$$

- **Assorbimento**:
	$$A \cup (A \cap B) = A, \hspace{1em} A \cap (A \cup B) = A$$

- **Complementazione**:
	$$A \cup A^C = U, \hspace{1em} A \cap A^C = \emptyset$$

- **De Morgan**:
	$$(A \cup B)^C = A^C \cap B^C, \hspace{1em} (A \cap B)^C = A^C \cup B^C$$

## Esempio di dimostrazione

_Enunciato_: $A \cup B = A \Leftrightarrow B \subseteq A$

_Dimostrazione_:

Prima, va dimostrato che $A \cup B = A \Rightarrow B \subseteq A$, per cui _diamo per scontato_ che $A \cup B = A$.

Avendo un elemento $b \in B$, pensare che $b \not\in A$ è assurdo perchè se $A \cup B = A$, allora $B$ o è un insieme vuoto, oppure è contenuto dentro $A$. Entrambi i casi portano alla conclusione che $B \subseteq A$.

Poi, va dimostrato che $B \subseteq A \Rightarrow A \cup B = A$, per cui diamo per scontato che $B \subseteq A$.

La forma $A \cup B = \{x | x \in A \lor x \in B\}$, ma dato che $B \subseteq A$ allora $\forall x \in B \Rightarrow x \in A$, quindi la precedente forma può anche essere scritta come $A \cup B = \{x | x \in A\}$, che significa $A \cup B = A$.
