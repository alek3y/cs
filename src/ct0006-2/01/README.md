# Normalizzazione

Nella **teoria della normalizzazione** la notazione di base utilizzata è:
- $A$, $B$ e $C$ per **attributi** singoli
- $T$, $X$ e $Y$ per **insiemi di attributi**
- $F$ e $G$ per **dipendenze funzionali**
- $R(T, F)$ per lo **schema** $R$ con attributi $T$ e dipendenze $F$
- $r$ per un'**istanza** di $R$ contenente più ennuple
- $t$, $u$ e $v$ per **ennuple** di una istanza $r$
- $t.X$ per gli $X$ **attributi dell'ennupla** $t$

Inoltre, durante la modellazione è usata per eliminare le **anomalie** di:
- **Aggiornamento**: è presente **ridondanza** dei dati
- **Inserimento** e **cancellazione**: i dati **non sono conservabili** senza relazionarli con altri dati autonomi

## Dipendenze

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
3. Aggiungere gli attributi $Y$ delle dipendenze trovate a $Z$
4. Ripetere finchè $Z$ viene aggiornato

Per esempio, se $X = AB$ e $F = \{A \rightarrow C, AC \rightarrow D, E \rightarrow F\}$ allora $Z_1 = ABC$ e $Z_2 = ABCD$.

## Chiavi

Degli attributi $X$ di $R(T, F)$ si possono definire come:
- **Superchiave** quando $X_F^+ = T$
- **Chiave** quando $X$ è _superchiave_ e $\forall A \in X,\ (X \setminus \{A\})_F^+ \neq T$
- **Attributi primi** se ognuno appartiene ad almeno una _chiave_

### Algoritmo

Dato il **candidato** $X :: (Y)$, la cui $X$ è una **possibile chiave** mentre $Y$ sono i possibili attributi da aggiungere se non lo fosse, si possono trovare **tutte le chiavi** di $R(T, F)$ attraverso:
1. Iniziare con il candidato $Z :: (T \setminus Z)$, dove $Z = T \setminus \Set{Y | X \rightarrow Y \in F}$ sono attributi indipendenti
2. Estrarre il primo candidato $X :: (Y)$ assicurandosi che $X$ non contenga alcuna chiave già trovata
	- Se $X$ è _superchiave_ si può aggiungere alle chiavi trovate
	- Altrimenti ai candidati si unisce $\Set{XA :: (W \setminus \{A\}) | A \in W}$, dove $W = Y \setminus X_F^+$
3. Ripetere finchè rimangono candidati validi

Serve anche a **verificare la primalità** degli attributi, dato che altrimenti il processo è molto **inefficiente**.

#### Esempio

Per esempio, se $T = ABCDEF$ e $G = \{AB \rightarrow C, E \rightarrow A, A \rightarrow E, B \rightarrow F\}$ i passaggi sono:
1. Il candidato iniziale è $BD :: (ACEF)$
2. $BD$ non è _superchiave_ quindi si aggiungono i candidati dagli attributi $ACEF \setminus BDF$
3. Il primo candidato $BDA :: (CE)$ tra i rimanenti è valido
4. $BDA$ è _superchiave_ quindi anche **chiave**
5. Il primo candidato $BDC :: (E)$ è valido
6. $BDC$ non è _superchiave_ quindi si aggiungono i candidati
7. Il primo candidato $BDE :: ()$ è valido
8. $BDE$ è _superchiave_ quindi anche **chiave**
9. Il rimanente candidato $BDCE$ contiene la chiave $BDE$ quindi non è valido

## Forma e copertura canonica

Le _dipendenze funzionali_ $F$ sono dette **in forma canonica** sse per ogni $X \rightarrow Y \in F$:
- $|Y| = 1$
- $X$ non ha attributi **estranei**, cioè $\nexists A \in X : X \setminus \{A\} \rightarrow Y \in F^+$
- $X \rightarrow Y$ non è **ridondante**, cioè $X \rightarrow Y \not\in (F \setminus \{X \rightarrow Y\})^+$

Se $G$ è in _forma canonica_ e $F \equiv G$, ovvero **equivalgono** per $F^+ = G^+$, allora è detta **copertura canonica**.

### Algoritmo

Trovare la _copertura canonica_ di $F$ consiste nel:
1. Iniziare con l'insieme di dipendenze _decomposte_ $G = \Set{X \rightarrow A | X \rightarrow Y \in F \land A \in Y}$
2. Rimpiazzare da $G$ le $X \rightarrow A$ per cui $|X| > 1$, con $Z \rightarrow A$ dove $Z$ è $X$ meno gli attributi _estranei_, rimuovendo un $B \in X$ alla volta e verificando che comunque $A \in (Z \setminus \{B\})_G^+$
3. Rimuovere le $X \rightarrow A$ _ridondanti_, verificando che $A \in X_{G \setminus \{X \rightarrow A\}}^+$

#### Esempio

Per esempio, se $F = \{A \rightarrow BC, B \rightarrow C, A \rightarrow B, AB \rightarrow C\}$:
1. Decomponendo si ottiene $G = \{A \rightarrow C, B \rightarrow C, A \rightarrow B, AB \rightarrow C\}$
2. Si rimpiazza $AB \rightarrow C$ con $B \rightarrow C$ perchè $C \in B_G^+ = BC$ finendo senza poter rimuovere $B$
3. Si rimuove $A \rightarrow C$ perchè $C \in A_{\{B \rightarrow C, A \rightarrow B\}}^+ = ABC$
