# Errori di sequenze

## Distanza di Hamming

La **distanza di Hamming** è una funzione che trova il numero di bit diversi tra due sequenze.

Per esempio $H(1010_2, 1100_2) = 2$, dove il primo argomento è la **codifica corretta**, mentre il secondo è la **codifica letta**.

## Bit di parità

Aggiungendo un bit di parità ad una sequenza di bit, è possibile rilevare se la sequenza è corretta oppure no, nel caso ci sia un **singolo errore**.

Per esempio se a $1010$ si aggiunge il bit di parità, che è $1$ se il numero di bit a uno è **dispari** ed è a $0$ se è **pari**, si ottiene $10100$, e se dovesse cambiare a $11100$ si sà che è sbagliato perchè ci sono $3$ bit a uno e quindi il bit di parità dovrebbe essere $1$.
