# Dipendenze funzionali

Dato uno schema $R(T, F)$, sono detti **dipendenze funzionali** quei vincoli:
$$
X \rightarrow Y \in F
$$
dove $X \cup Y \subseteq T$, per cui l'unicità di $Y$ dipende da $X$.

Sono anche dette **derivate** se $F$ le **implica logicamente**, cioè:
$$
F \models X \rightarrow Y \Leftrightarrow F \vdash X \rightarrow Y
$$
che accade sse ogni istanza $r$ **soddisfa** $X \rightarrow Y$, ovvero:
$$
\forall t, u \in r,\ t.X = u.X \Rightarrow t.Y = u.Y
$$

## Assiomi

La **derivazione** di altre _dipendenze_ da $F$ avviene sfruttando gli **assiomi di Armstrong**:
- **Riflessività**, $Y \subseteq X \Rightarrow X \rightarrow Y$
- **Aumento**, $X \rightarrow Y \Rightarrow XW \rightarrow YW$
- **Transitività**, $X \rightarrow Y \land Y \rightarrow Z \Rightarrow X \rightarrow Z$

da cui si possono ricavare le **regole derivate** di:
- **Unione**, $X \rightarrow Y \land X \rightarrow Z \Rightarrow X \rightarrow YZ$
- **Decomposizione**, $X \rightarrow YZ \Rightarrow X \rightarrow Y$
- **Indebolimento**, $X \rightarrow Y \Rightarrow XZ \rightarrow Y$

## Chiusura

Si dice **chiusura** di $F$, l'insieme di tutte le dipendenze derivabili da $F$:
$$
F^+ = \Set{X \rightarrow Y | F \vdash X \rightarrow Y}
$$
che però risulta essere algoritmicamente **inefficiente** da trovare.

La **chiusura** di un attributo $X \subseteq T$ invece, è l'insieme degli attributi la cui dipendenza per $X$ è _derivabile_:
$$
X_F^+ = \Set{A \in T | F \vdash X \rightarrow A}
$$
e semplifica la risoluzione del **problema dell'implicazione**, cioè la verifica che $X \rightarrow Y \in F^+$, perchè:
$$
F \vdash X \rightarrow Y\ \Leftrightarrow\ Y \subseteq X_F^+
$$

Trovare la _chiusura_ di $X$ consiste quindi nel cominciare con $X^+ = X$ e aggiungerci iterativamente gli attributi che dipendono da quelli dentro $X^+$.

### Algoritmo

Trovare la _chiusura_ $X_F^+$ di un insieme di attributi $X$ consiste nel:
1. Iniziare con l'insieme $Z = X$
2. Trovare tutte le dipendenze $W \rightarrow Y \in F$ per cui $W \subseteq Z$
3. Aggiungere a $Z$ gli attributi $Y$ delle dipendenze trovate
4. Ripetere finchè $Z$ viene aggiornato

Per esempio, se $X = AB$ e $F = \{A \rightarrow C, AC \rightarrow D, E \rightarrow F\}$ allora $Z_1 = ABC$ e $Z_2 = ABCD$.
