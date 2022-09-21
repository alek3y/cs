# Insiemi

Gli insiemi vengono indicati con le lettere maiuscole (e.g. $A$, $B$, ...), mentre i suoi elementi vegono indicati come variabili a lettere minuscole (e.g. $a$, $b$, ...).

Due insiemi sono uguali se contengono gli stessi elementi, oppure:
$$A = B \Leftrightarrow (\forall x (x \in A \Leftrightarrow x \in B))$$

Gli insiemi possono essere rappresentati in due modi in particolare:
- Rappresentazione **estensiva**, per cui vengono elencati tutti gli elementi; e.g. $A = \{a, b\} \Rightarrow c \not\in A$
- Rappresentazione **intensiva**, per cui gli elementi vengono descritti attraverso una proprietà o vengono sottointesi; e.g. $A = \{1, 2, 3, 4, ...\} \Rightarrow 5 \in A$

## Operazioni

- **Unione**: $A \cup B = \{x | x \in A \lor x \in B\}$
- **Intersezione**: $A \cap B = \{x | x \in A \land x \in B\}$, che sarà $\emptyset$ quando i due insiemi sono _disgiunti_
- **Complementare**: $A^c = \{x | x \not\in A\}$
- **Differenza**: $A \setminus B = \{x | x \in A \land x \not\in B\}$
- **Prodotto cartesiano**: $A \times B = \{(a, b) | (a \in A) \land (b \in B)\}$

## Proprietà

$$
A \cup A = A; \hspace{1em} A \cap A = A \\
A \cup B = B \cup A; \hspace{1em} A \cap B = B \cap A \\
A \cup \emptyset = A; \hspace{1em} A \cap \emptyset = \emptyset \\
A \cup B \supseteq A; \hspace{1em} A \cap B \subseteq A \\
A \cup B = A \Leftrightarrow A \supseteq B; \hspace{1em} A \cap B = A \Leftrightarrow A \subseteq B \\
A \cup (B \cap C) = (A \cup B) \cap (A \cup C); \hspace{1em} A \cap (B \cup C) = (A \cap B) \cup (A \cap C)
$$

### Esempi di dimostrazioni

1. $A \cup B = A \Leftrightarrow B \subseteq A$

	Prima, va dimostrato che $A \cup B = A \Rightarrow B \subseteq A$, per cui _diamo per scontato_ che $A \cup B = A$.

	Avendo un elemento $b \in B$, pensare che $b \not\in A$ è assurdo perchè se $A \cup B = A$, allora $B$ o è un insieme vuoto, oppure è contenuto dentro $A$. Entrambi i casi portano alla conclusione che $B \subseteq A$.

	Poi, va dimostrato che $B \subseteq A \Rightarrow A \cup B = A$, per cui diamo per scontato che $B \subseteq A$.

	La forma $A \cup B = \{x | x \in A \lor x \in B\}$, ma dato che $B \subseteq A$ allora $\forall x \in B \Rightarrow x \in A$, quindi la precedente forma può anche essere scritta come $A \cup B = \{x | x \in A\}$, che significa $A \cup B = A$.

## Insiemi specifici

- $\mathbb{N} = \{0, 1, 2, ...\}$
- $\mathbb{Z} = \{..., -2, -1, 0, 1, 2, ...\}$
- $\mathbb{Q} = \{\frac{p}{q} | p \in \mathbb{Z} \land q \in \mathbb{Z} \setminus \{0\}\}$, tra cui i numeri periodici
- $\mathbb{I} = \mathbb{R} - \mathbb{Q}$, che contiene numeri con parte decimale infinita e non periodica
