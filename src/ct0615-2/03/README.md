# Memoria virtuale

La **memoria virtuale** genera, **per ogni processo**, uno _spazio di indirizzamento_ **virtuale** con quantità maggiore di quella disponibile attraverso la **swap**, cioè parte del disco rigido che funge da memoria.

La memoria è suddivisa in **pagine** di dimensione prefissata (come i blocchi della _cache_) indirizzate dal **virtual page number**, mentre il byte specifico è indirizzato dal **page offset** (i.e. primi $\log_2 s$ bit, con $s$ _page size_).
Anche la memoria fisica, avendo la stessa _page size_ viene indirizzata con **physical page number** e **page offset** uguale a quello della memoria virtuale.

Attraverso l'uso di questa memoria, ogni processo:
- può usare **più memoria di quella disponibile**
- ha lo spazio di indirizzamento **isolato e protetto** dagli altri processi
- può essere **caricato ovunque** in memoria, anche in regioni non continue

La CPU lavora solamente con _indirizzi virtuali_, mentre altro hardware si occupa della **traduzione** dello _spazio indirizzato_ in _indirizzo fisico_.
Nel caso in cui la locazione richiesta non sia in memoria, la CPU viene **notificata** e si occuperà del processo di **swapping**, cioè il caricamento da _swap_ a _RAM_.

Le corrispondenze tra gli _indirizzi virtuali_ e _fisici_ vengono salvate all'interno della **page table** associata al processo, che si trova nella _RAM_ all'indirizzo salvato dal **page table register**.
