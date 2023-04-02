# Memoria virtuale

La **memoria virtuale** genera, **per ogni processo**, uno _spazio di indirizzamento_ **virtuale** con quantità maggiore di quella disponibile attraverso la **swap**, cioè parte del disco rigido che fa da memoria.

Attraverso l'uso di questa memoria, ogni processo:
- può usare **più memoria di quella disponibile**
- ha lo spazio di indirizzamento **isolato e protetto** dagli altri processi
- può essere **caricato ovunque** in memoria, anche in regioni non continue

La memoria è suddivisa in **pagine** di dimensione prefissata (come i blocchi della cache) indirizzate dal **virtual page number**, mentre il byte specifico è indirizzato dal **page offset** (i.e. primi $\log_2 s$ bit, con $s$ _page size_).
Anche la memoria fisica, avendo la stessa _page size_ viene indirizzata con **physical page number** e **page offset** uguale a quello della memoria virtuale.

Le corrispondenze tra gli _indirizzi virtuali_ e _fisici_ vengono salvate all'interno della **page table** associata al processo, che si trova nella _RAM_ all'indirizzo salvato dal **page table register** (diverso per ogni processo).

Come per la cache, anche le pagine hanno dei [bit di stato](../02/README.md#bit-di-stato).

## Page fault

La CPU lavora solamente con _indirizzi virtuali_, mentre altro hardware si occupa della **traduzione** dello _spazio virtuale indirizzato_ in _indirizzo fisico_.

Se la locazione richiesta non si trova in memoria, la CPU viene **notificata** con un **page fault** e si occuperà del processo di **swapping**, cioè di copiare i dati dalla _swap_ alla _RAM_, causando un _miss penalty_ molto elevato.

## Translation Lookaside Buffer

Trovandosi in _RAM_ l'accesso alla _page table_ è molto lento, di conseguenza il **TLB** è una cache completamente dedicata al **caching della page table**, in particolare delle righe vicine a quelle lette più di recente.

Nel caso avvenga un **TLB miss**, la pagina verrà caricata se presente nella _page table_, altrimenti verrà generato un _page fault_ che forzerà lo _swapping_.

## Addressing della cache

Esistono tre modi in cui la cache può lavorare:
- **Physically addressed cache**: per accedere alla cache l'_indirizzo virtuale_ del dato dovrà prima essere tradotto in _fisico_ con il _TLB_ (e _page table_ nel caso di _TLB miss_)
- **Virtually addressed cache**: accede con _indirizzi virtuali_ (non usa il _TLB_), ma può causare **aliasing**, e quindi due processi accedono allo stesso blocco se usano lo stesso _indirizzo virtuale_ (capita spesso)
- **Virtually indexed but physically tagged**: traduce comunque l'indirizzo con il _TLB_, ma [calcola](../02/README.md#mapping) l'_index_ dall'_indirizzo virtuale_ e il _tag_ da quello _fisico_

## Sistema operativo

Nel caso di **TLB miss** o **page fault**, la _CPU_ passa il controllo alla routine di gestione del **sistema operativo**.

Facendo ciò passa anche da **user mode** a **supervisor mode** (o _kernel mode_), che permette al sistema di:
- modificare il _page table register_
- modificare le entry _TLB_

Non potendo cambiare l'**execution mode** da _user mode_ a _supervisor mode_, un programma dovrà affidarsi a delle **system call**.
Un esempio su _ARM_ è `svc`, che lancia un'_eccezione_ elevando l'esecuzione in _supervisor mode_ e quindi permettendo al _sistema operativo_ di eseguire il servizio richiesto.

Durante un _page fault_, mentre i dati vengono copiati in _RAM_, il _sistema operativo_ effettua un **context switch** cioè **salva i registri** del processo corrente e **riprende l'esecuzione** di un altro.
Inoltre, per evitare che il nuovo processo acceda a regioni non permesse, la _TLB_ viene **svuotata** a meno che ad ogni pagina non sia **associato** l'**ASID** (_Address Space ID_, ovvero il _PID_), causando _miss_ per processi diversi.

## Page table multilivello

Generare una _page table_ per ogni processo **riempirebbe** la _RAM_, considerata la quantità di _indirizzi virtuali_.
Per questo, _ARM_ utilizza più **livelli** di _page table_ suddividendo il _virtual page number_ in $9$ bit.

Per esempio, con i primi $9$ bit si trova dalla prima _page table_ l'indirizzo della seconda, con i $9$ successivi si trova nella seconda l'indirizzo della terza _page table_, e così via fino all'ultima che dà l'_indirizzo fisico_.

In questo modo, solamente le _page table_ interessate al processo verranno caricate in memoria (e.g. la maggior parte degli indirizzi hanno gli stessi bit più significativi).
