# Chiavi

Degli attributi $X$ di $R(T, F)$ si possono definire come:
- **Superchiave** quando $X_F^+ = T$
- **Chiave** quando $X$ è _superchiave_ e $\forall A \in X,\ (X \setminus \{A\})_F^+ \neq T$
- **Attributi primi** se ognuno appartiene ad almeno una _chiave_

### Algoritmo

Dato il **candidato** $X :: (Y)$, la cui $X$ è una **possibile chiave** mentre $Y$ sono i possibili attributi da aggiungere se non lo fosse, si possono trovare **tutte le chiavi** di $R(T, F)$ attraverso:
1. Iniziare con il candidato $T \setminus \Set{Y | X \rightarrow Y \in F} :: \Set{X | X \rightarrow Y \in F}$
2. Estrarre il primo candidato $X :: (Y)$ assicurandosi che $X$ non contenga alcuna chiave già trovata
	- Se $X$ è _superchiave_ si può aggiungere alle chiavi trovate
	- Altrimenti ai candidati si unisce $\Set{XA :: (W \setminus \{A\}) | A \in W}$, dove $W = Y \setminus X_F^+$
3. Ripetere finchè rimangono candidati validi

Serve anche a **verificare la primalità** degli attributi, dato che altrimenti il processo è molto **inefficiente**.

#### Esempio

Per esempio, se $T = ABCDEF$ e $G = \{AB \rightarrow C, E \rightarrow A, A \rightarrow E, B \rightarrow F\}$ i passaggi sono:
1. Il candidato iniziale è $BD :: (ABE)$
2. $BD$ non è _superchiave_ quindi si aggiungono i candidati dagli attributi $ABE \setminus BDF$
3. Il prossimo candidato $BDA :: (E)$ è valido perchè non contiene chiavi
4. $BDA$ è _superchiave_ quindi anche **chiave**
5. Il prossimo candidato $BDE :: (A)$ è valido
8. $BDE$ è _superchiave_ quindi anche **chiave**
