# Automi per circuiti

## Automi di Moore

In un circuito sequenziale di **Moore** si ha che:
- $O_i = \delta(S_i)$, quindi ogni **output** dipende dallo stato corrispondente
- $S_{i+1} = \lambda(I_i, S_i)$, per cui la **transizione di stato** dipende dall'input e dallo stato precedente

Ogni possibile transizione viene rappresentata dalla **tabella di transizione di stato**, per esempio:

![Esempio di automa di Mealy](assets/01.png)

La **minimizzazione** della tabella di transizione consiste nel trovare un'equazione minimizzata per ogni n-esimo bit che compone lo stato, in modo da trovare la funzione $\lambda$, per esempio:

![Esempio di automa e minimizzazione](assets/02.png)

## Automi di Mealy

Al confronto del circuito di _Moore_, in quello di **Mealy** si ha che:
- $O_i = \delta(I_i, S_i)$
- $S_{i+1} = \lambda(I_i, S_i)$

Quindi, l'output non verrà posizionato sui nodi affianco agli stati, ma sugli archi affianco agli input, per esempio:

![Esempio di automa di Mealy](assets/03.png)

Vanno quindi fatte le due _mappe di Karnaugh_ per i bit di $S_i$ e per quelli di $O_i$, per poi ricavarne le equazioni.

Un esempio completo è:

![Esempio completo di un automa di Mealy](assets/04.png)

## Microistruzioni

Quando ci sono tanti stati diventa complicato disegnare l'automa, per questo è utile scrivere un **microprogramma** attraverso delle **microistruzioni** che seguono il formato:
```
Stato0:
 Out1←1,  # Valori output di Stato0
 Out2←0
 case (In1, In2) of  # Casi input
  (_, 0): Stato0
  (_, 1): Stato1  # Transizione a Stato1

Stato1:
 Out1←0,
 Out2←1
 case (In1, In2) of
  (0, _): Stato1
  (1, _): Stato0
```

Sulla _CPU_ queste microistruzioni possono essere salvate all'interno di una **ROM** ed avere un suo _micro Program Counter_ che tiene traccia dell'istruzione corrente in modo da semplificare l'_ISA_.
