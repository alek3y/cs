# Forma e copertura canonica

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
