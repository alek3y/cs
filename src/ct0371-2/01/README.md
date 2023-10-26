# Dizionari

Un **dizionario** è una funzione $R\colon D \to C$ dove $d \in D$ sono le **chiavi** e $c \in C$ sono i **valori**.

Il **tipo di dato** sarà quindi un insieme $S \subseteq D \times C$, mentre le **operazioni** sono:
- `search(Dizionario S, Chiave K) -> Elem | NIL`
- `insert(Dizionario S, Chiave K, Elem V)` che aggiorna se `K` è già presente
- `delete(Dizionario S, Chiave K)` che assume che `K` sia presente
