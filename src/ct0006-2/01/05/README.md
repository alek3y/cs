# Forme normali

Le **forme normali** si possono ottenere attraverso il processo di **normalizzazione**, che coinvolge la [decomposizione](../04/README.md), e che garantisce la qualità dello schema risultante.

## Forma normale di Boyce-Codd

Uno schema $R(T, F)$ è in **BCNF** sse per ogni $X \rightarrow Y \in F$ tale che $Y \nsubseteq X$ si ha che $X$ è una _superchiave_.

Questa forma **preserva i dati**, garantisce l'**assenza di anomalie** ma **non preserva le dipendenze**.

### Algoritmo di analisi

La conversione in _BCNF_ consiste nel _decomporre_ **ricorsivamente** lo schema $R(T, F)$:
1. Per il caso base, se $R(T, F)$ è in _BCNF_ restituire $\{R(T, F)\}$
2. Altrimenti trovare una $X \rightarrow Y \in F$ che viola la _BCNF_
3. Calcolare gli attributi $T_1 = X_F^+$ e $T_2 = X \cup (T \setminus T_1)$
4. Calcolare le proiezioni $F_1 = \pi_{T_1}(F)$ e $F_2 = \pi_{T_2}(F)$
5. Trovare la decomposizione _ricorsiva_ $\rho_1$ di $R_1(T_1, F_1)$ e $\rho_2$ di $R_2(T_2, F_2)$
6. Restituire $\rho_1 \cup \rho_2$

#### Esempio

Per esempio con $R(ABC, \{AB \rightarrow C, C \rightarrow A\})$ la conversione sarebbe:
1. La _BCNF_ è violata da $C \rightarrow A$ perchè $C$ non è _superchiave_, infatti $C_F^+ \neq ABC$
2. Si calcolano $T_1 = C_F^+ = AC$ e $T_2 = BC$
3. Si calcolano $F_1 = \{C \rightarrow A\}$ e $F_2 = \emptyset$
4. Si trovano le decomposizioni $\rho_1 = \{R_1(AC, \{C \rightarrow A\})\}$ e $\rho_2 = \{R_2(BC, \emptyset)\}$

## Terza forma normale

Uno schema $R(T, F)$ è detto in **3NF** sse per ogni $X \rightarrow Y \in F$ tale che $Y \nsubseteq X$ si ha che $X$ è una _superchiave_ oppure tutti gli attributi $Y \setminus X$ sono _attributi primi_.

Conviene quindi prima [trovare le chiavi](../02/README.md#algoritmo) di $R$ e poi verificarne la proprietà.

Questa forma **preserva i dati e le dipendenze**, ma può comunque possedere **anomalie**.

### Algoritmo di sintesi

La conversione in _3NF_ consiste nel:
1. Iniziare con la _copertura canonica_ $G$ di $F$
2. Rimpiazzare da $G$ le $X \rightarrow A_1, ..., X \rightarrow A_n$ con una singola $X \rightarrow A_1, ..., A_n$
3. Creare un $R_i(XY)$ per ogni $X \rightarrow Y \in G$
4. Rimuovere gli schemi i cui attributi fanno parte di un altro schema più grande
5. Se nessun $R_i$ ha attributi _superchiave_ per $R$, aggiungere un $R_{n+1}(Z)$, dove $Z$ è una chiave di $R$
