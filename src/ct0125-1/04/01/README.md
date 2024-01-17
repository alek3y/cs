# Paginazione

Ogni **entry** della [page table](../../../ct0615-2/03/README.md) ha un **page frame**, i.e. l'indirizzo in memoria principale della pagina, e un **bit di residenza** ad $1$ se la pagina non è presente e dell'_entry_ va quindi letto l'indirizzo di memoria secondaria.

Le pagine possono anche essere **condivise** tra processi, riducendo l'uso complessivo di memoria e velocizzando anche la creazione di nuovi processi (e.g. con `fork`), grazie alla tecnica del **copy on write**.

## Tabella inversa

Oltre alla [tabella multilivello](../../../ct0615-2/03/README.md#page-table-multilivello), esiste la **tabella inversa delle pagine** che memorizza un'_entry_ **per ogni** _page frame_ presente in memoria principale.

Ogni _entry_ contiene un'etichetta dell'indirizzo virtuale, con cui si possono verificare le **collisioni**.

L'**hash** della **pagina virtuale** verrà usato come indice nella tabella, e nel caso di collisioni sarà presente un puntatore alla successiva _entry_, il cui indice corrisponde con il **numero del frame** in memoria principale.

Dato che la dimensione della tabella è fissa, si può usare una **hash anchor table** per ridurre le collisioni.

## Sostituzione

Dati i [principi di località](../../../ct0615-2/02/README.md), un processo tende a riferire le **proprie pagine** che tendono ad essere **adiacenti**.

Il caricamento delle pagine può avvenire **a richiesta**, cioè quando il processo vi fa riferimento, oppure **a previsione** (o _prefetching_), per cui il S.O. prevede quanta memoria preallocare.

Quando avviene un [page fault](../../../ct0615-2/03/README.md#page-fault), il S.O. dovrà **trovare** l'indirizzo di memoria secondaria, **caricare** la pagina nel _page frame_ ed aggiornare la tabella delle pagine.

Tra i meccanismi di **sostituzione** le strategie possono essere **globali** se sostituiscono una delle pagine di tutti i processi, oppure **locali** se considerano le pagine del singolo processo, come:
- **Ottimale**

	La miglior strategia possibile, e quindi **ideale**, che sostituisce una pagina a cui non verrà mai fatto riferimento in futuro, facendo anche il minimo numero di _page fault_.

- **Casuale**

	Sostituisce ogni pagina con la **stessa probabilità**, ed è quindi equa, **semplice** e veloce.

- **First In First Out** (_FIFO_)

	Sostituisce la pagina **più vecchia**, e perciò può sostituire anche pagine molto usate.
	Non conviene per via dell'**anomalia di Belady**, per cui una coda più lunga potrebbe anche portare a più _page fault_.

- **Least Recently Used** (_LRU_)

	Sostituisce la pagina che è stata **usata meno volte** di recente, attraverso un **contatore** di riferimenti su ogni pagina o spostandole in testa ad una lista ad ogni riferimento.

	In entrambi i casi aumenta l'_overhead_, dato che ogni riferimento causa un'operazione molto costosa.

	Inoltre, può anche portare a **continui** _page fault_ se la prossima pagina da riferire è quella meno recentemente utilizzata all'interno di un ciclo.

- **Least Frequently Used** (_LFU_)

	Sostituisce la pagina **meno intensamente** riferita, cioè quella a cui non si fa riferimento spesso.

	Potrebbe sostituire pagine recenti al posto di una pagina che in passato era molto riferita.

- **Not Frequently Used** (_NFU_)

	Sostituisce la pagina che **non è stata recentemente** riferita, cioè quella con il contatore minimo che viene periodicamente incrementato se il _bit di riferimento_ della pagina è a $1$.

- **Not Recently Used** (_NRU_)

	Approssima l'_LRU_ salvando bit di **riferimento** e **modifica**, con priorità $00$, $01$, $10$ e $11$ rispettivamente.

	Se una pagina è riferita prima dell'azzeramento periodico dei bit, può essere non _riferita_ ma _modificata_.

- **Second chance**

	Variante del _FIFO_, sceglie la pagina **più vecchia** se il suo _bit di riferimento_ è impostato a $0$, altrimenti la sposta in coda per considerarla come nuovo arrivo.

	Questo assicura che le pagine attive abbiano minor probabilità di essere sostituite.

- **Sostituzione a orologio**

	Variante del _FIFO_, organizza le pagine in una **lista circolare**, tiene un **puntatore** alla pagina più vecchia e la sceglie se il _bit di riferimento_ è $0$, altrimenti lo azzera e sposta il puntatore alla pagina successiva.

- **Sostituzione Far**

	Modella l'accesso delle pagine come un **grafo**, dove i riferimenti sono gli **archi** mentre le pagine i **nodi**.
	Analizzando il programma, si sceglie come padre la pagina con l'istruzione che accede alle pagine figlie.

	Dal _grafo_ è scelta la pagina **più lontana** da tutte le pagine **riferite**, comportando un _overhead_ elevato.

- **Modello Working Set**

	L'insieme di pagine che il processo riferisce dall'istante $t-w$ a $t$ è detto il suo **working set** $W(t, w)$.

	Aumentando la finestra $w$ aumentano i _page frame_ assegnati al processo, riducendo quindi i _page fault_.
	Dopo una certa soglia però l'effetto sarà minimo, dato che le pagine riferite tendono ad essere vicine.

	Se invece la finestra $w$ fosse troppo piccola, il sistema potrebbe entrare in una situazione di **thrashing**, in cui il processore è per la maggior parte del tempo occupato a gestire i _page fault_.

	Un'implementazione salva il **tempo dell'ultimo riferimento** di ogni pagina, e al tempo $t$ sostituisce quelle riferite l'ultima volta prima del tempo $t-w$, che quindi non rientrano nella finestra $w$.

- **Working Set Clock** (_WSClock_)

	Unisce il _working set_ con la _sostituzione a orologio_, evitando di iterare tutte le pagine ad ogni _page fault_.

	Quando avviene un _page fault_, la pagina **puntata** è sostituita se il _bit di riferimento_ è $0$ e il _tempo dell'ultimo riferimento_ è precedente a $t - w$, altrimenti li aggiorna e riprova con la pagina successiva.

- **Page Fault Frequency** (_PFF_)

	Regola $w$ del _working set_ in base alla **frequenza** dei _page fault_ del processo, aggiornandolo solamente dopo ogni _page fault_, invece che dopo ogni riferimento.

## Dimensione della pagina

Ogni S.O. fornisce una **dimensione** di pagina diversa, favorendone una **piccola** se preferisce ridurre la frammentazione interna e la memoria per contenere il _working set_, aumentando però lo spazio richiesto dalla page table, altrimenti **grande** se preferisce migliorare le prestazioni e ridurre le operazioni di _I/O_.
