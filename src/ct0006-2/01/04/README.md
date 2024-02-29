# Decomposizione di schemi

Le [anomalie](../README.md) possono essere risolte attraverso la **decomposizione** dello schema in schemi **più piccoli**.

Dato uno schema $R(T, F)$, si definisce come **proiezione** di $F$ su $Z \subseteq T$ l'insieme:
$$
\pi_Z(F) = \Set{X \rightarrow Y \in F^+ | X \cup Y \subseteq Z}
$$
attraverso cui si trova una **decomposizione** di $R(T, F)$, cioè un insieme di schemi
$$
\rho = \{R_1(T_1, F_1), ..., R_n(T_n, F_n)\}
$$
per cui $\bigcup_i T_i = T$ e $F_i = \pi_{T_i}(F)$ filtrando per ogni $R_i$ le dipendenze con attributi coinvolti in $T_i \neq \emptyset$.

Per esempio su $R(ABCDE, \{AB \rightarrow CD, B \rightarrow E\})$ si risolvono le anomalie decomponendolo in:
- $R_1(ABCD, \{AB \rightarrow CD\})$
- $R_2(BE, \{B \rightarrow E\})$

### Algoritmo

Il calcolo della _proiezione_ $\pi_{T_i}(F)$ avviene dal:
1. Iniziare con l'insieme $P = \emptyset$
2. Per ogni $X \subset T_i$ trovare $Y = X_F^+ \setminus X$
3. Aggiungere a $P$ la dipendenza $X \rightarrow (Y \cap T_i)$

#### Esempio

Per esempio su $\pi_{AB}(\{A \rightarrow B, B \rightarrow C, C \rightarrow A\})$, si ha:
1. L'insieme delle proiezioni iniziale è $P = \emptyset$
2. Si trovano le $X \subset AB$ da usare dall'[insieme delle parti](../../../ct0434/02/README.md#insiemi-delle-parti) di $AB$
3. Per $X = A$ si trova $Y = A_F^+ \setminus A = BC$
4. A $P$ si aggiunge $A \rightarrow (BC \cap AB) = A \rightarrow B$
5. Per $X = B$ si trova $Y = B_F^+ \setminus B = AC$
6. A $P$ si aggiunge $B \rightarrow (AC \cap AB) = B \rightarrow A$

## Preservazione dei dati

Per dire che la _decomposizione_ **preservi i dati**, si deve avere che:
$$
r = \pi_{T_1}(r) \bowtie ... \bowtie \pi_{T_n}(r)
$$
per ogni istanza $r$ di $R(T, F)$, assicurandosi che dalle tabelle decomposte si **ricavino gli stessi dati** di $r$.

Per **verificare** che una decomposizione $\rho = \{R_1(T_1, F_1), R_2(T_2, F_2)\}$ _preservi i dati_ basta verificare che
$$
T_1 \cap T_2 \rightarrow T_1 \in F^+ \lor\ T_1 \cap T_2 \rightarrow T_2 \in F^+
$$
trovando la [chiusura](../01/README.md#algoritmo) $(T_1 \cap T_2)_F^+$ e verificando che contenga $T_1$ o $T_2$.

Per esempio, con $R(ABCD, \{A \rightarrow BC\})$ si trova $T_1 = ABC$, $T_2 = AD$ e $A_F^+ = ABC = T_1$.

## Preservazione delle dipendenze

Per dire che la _decomposizione_ **preservi le dipendenze**, si deve avere che:
$$
G = \bigcup_i \pi_{T_i}(F) \equiv F
$$

### Algoritmo

Per **verificare** che le dipendenze siano _preservate_ basta assicurarsi che $Y \subseteq X_G^+$, per ogni $X \rightarrow Y \in F$.

Mentre $X_G^+ = X_{\bigcup_i \pi_{T_i}(F)}^+$ si può ricavare con:
1. Iniziare con l'insieme $Z = X$
2. Trovare tutti gli schemi $R_i(T_i, F_i) \in \rho$
3. Aggiungere a $Z$ gli attributi $(Z \cap T_i)_F^+ \cap T_i$
4. Ripetere finchè $Z$ viene aggiornato

#### Esempio

Per esempio se $F = \{A \rightarrow B, B \rightarrow C, C \rightarrow A\}$ e $\rho = \{R_1(AB), R_2(BC)\}$, si ha:
1. Verifica che $B \subseteq A_G^+$
	1. L'insieme iniziale è $Z = A$ e il primo schema è $R_1(AB)$
	2. A $Z$ si aggiunge $(Z \cap T_1)_F^+ \cap T_1 = (A \cap AB)_F^+ \cap AB = ABC \cap AB = AB$
	3. Il prossimo schema è $R_2(BC)$
	4. A $Z$ si aggiunge $(Z \cap T_2)_F^+ \cap T_2 = (AB \cap BC)_F^+ \cap BC = ABC \cap BC = BC$
	5. Quindi è verificato perchè $B \subseteq Z = ABC$
2. Verifica che $C \subseteq B_G^+$
	1. L'insieme iniziale è $Z = B$ e il primo schema è $R_1(AB)$
	2. A $Z$ si aggiunge $B_F^+ \cap AB = AB$
	3. Il prossimo schema è $R_2(BC)$
	4. A $Z$ si aggiunge $B_F^+ \cap BC = BC$
	5. Quindi è verificato perchè $C \subseteq Z = ABC$
3. Verifica che $A \subseteq C_G^+$
	1. L'insieme iniziale è $Z = C$ e il primo schema è $R_1(AB)$
	2. A $Z$ si aggiunge $\emptyset_F^+ \cap AB = \emptyset$
	3. Il prossimo schema è $R_2(BC)$
	4. A $Z$ si aggiunge $C_F^+ \cap BC = BC$
	5. Il prossimo schema è $R_1(AB)$ di nuovo
	6. A $Z$ si aggiunge $B_F^+ \cap AB = AB$
	7. Quindi è verificato perchè $A \subseteq Z = ABC$

## Relazione tra dati e dipendenze

Data una _decomposizione_ $p = \{R_1(T_1), ..., R_n(T_n)\}$ di $R(T, F)$ che **preserva le dipendenze** e per cui esiste un $T_i$ _superchiave_ di $R$ allora $p$ si dice che **preserva anche i dati**.
