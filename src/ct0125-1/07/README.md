# Casi di studio

## Linux

Possiede un _kernel_ [**monolitico**](../README.md#tipi) con componenti **modulari** che rispecchiano quelli _a livelli_, che ne permette il facile **porting** su nuove piattaforme e **integrazioni** con nuovi moduli caricati a richiesta.

### Processi e thread

Lo [**scheduler**](../03/README.md) ha code [round robin](../03/README.md#algoritmi) per ognuna delle 140 priorità su due **run queue** che sono **invertite** dopo averle svuotate, evitando l'**attesa infinita** con una _deadline_ prima di cui dev'essere eseguito ogni processo.

Lo _scheduler_ si occupa anche dell'invio dei [**segnali**](../01/README.md#segnali) e la ricezione degli **interrupt**, ed è al più [**soft real-time**](../03/README.md#real-time).

### Memoria

La memoria sfrutta la [**memoria virtuale**](../04/README.md#memoria-virtuale) con [**paginazione**](../04/01/README.md), implementata con una _page table_ da [tre livelli](../../ct0615-2/03/README.md#page-table-multilivello).

Principalmente la memoria **fisica** è suddivisa dal _kernel_ in **tre zone**:
- `ZONE_DMA`: per i dati trasferiti da vecchio hardware con il [DMA](../../ct0615-2/05/02/README.md#direct-memory-access)
- `ZONE_NORMAL`: per la maggior parte dei dati direttamente indirizzabili dal _kernel_
- `ZONE_HIGHNORMAL`: per il resto dei dati non indirizzabili direttamente da processori a 32 bit

L'allocazione dei _page frame_ avviene con l'algoritmo **buddy** che, se non trova un blocco libero abbastanza grande ne dimezza uno, unendo le metà avanzate, finché non combacia con la dimensione richiesta.

Inoltre ogni _page frame_ è suddiviso in **slab**, cioè piccole strutture dati che sono singolarmente allocabili.

Le pagine sono sostituite, e salvate su un'unità o [inode](../05/README.md#allocazione) di _swap_, [a orologio](../04/01/README.md#sostituzione) dalla coda di **pagine inattive** che vengono spostate in quella delle **attive**, contenenti il _working set_, se sono state riferite per la seconda volta.

Lo **swapping** è gestito dal _demone_ **kswapd**, che fa [write back](../../ct0615-2/02/README.md#conflitti) sulle pagine modificate nella _cache di swap_.

### Filesystem

Le operazioni sui file sono **astratte** dai vari [filesystem](../05/README.md) attraverso il **virtual filesystem**.

Un _filesystem_ che lo implementa è **ext2**, che suddivide l'unità in **gruppi** di blocchi e ne salva la dimensione e il numero sul _super blocco_, tracciandone gli _inode_ con una [**bitmap**](../05/README.md#spazio-libero).

Altri esempi sono il **procfs** sulla memoria principale, che contiene informazioni dettagliate sul sistema e sull'hardware, e **network file system** che sfrutta l'architettura _client/server_.

## Windows

Sviluppato sul _kernel_ **NT**, che fornisce l'interfaccia alle **chiamate di sistema**, è suddiviso in due sottolivelli:
- **Executive**: contiene la maggior parte dei servizi, come la gestione dei processi e dell'_I/O_
- **Kernel**: estende l'_executive_ ed effettua lo _scheduling_ e gestisce gli _interrupt_

Gli _interrupt_ sono gestiti dall'**Interrupt Service Routine** che li maschera in base ad un livello di **priorità**.

Un servizio particolare dell'_executive_ è il **gestore degli oggetti** che si occupa di gestire **risorse** come processi, file e dispositivi, rappresentati come **oggetti** logici a cui è assegnato un identificatore chiamato **handle**.

Un altro componente è l'**Hardware Abstraction Layer**, che si occupa di astrarre l'_hardware_ dal software.

### Processi e thread

Un [**processo**](../01/README.md) è descritto dai blocchi, come il [PCB](../01/README.md#descrittore):
- **EPROCESS** nell'_executive_: descrive il processo e all'utente fornisce il **Process Environment Block**
- **KPROCESS** nel _kernel_: dedicato alle informazioni di _scheduling_

Anche i [**thread**](../02/README.md) hanno il blocco **ETHREAD**, ma il **Thread Environment Block** è contenuto nel **KTHREAD**.
Inoltre hanno un loro **Thread Local Storage** isolato dagli altri _thread_ che può essere indicizzato e modificato.

Utilizza un [modello di threading](../02/README.md#modelli-di-threading) **ibrido** assegnando ad ogni processo una sua **thread pool**, schedulandoli a [round robin](../03/README.md#algoritmi) con diverse code per ognuna delle 32 priorità assegnabili.

Nei _thread_ **dinamici** con priorità 0-15 la priorità aumenta in base agli eventi e diminuisce alla fine del quanto, mentre nei _thread_ **in tempo reale** con priorità 16-31 la priorità non cambia.

Sia i _thread_ che i _processi_ possiedono una **priorità base** oltre cui la priorità non può scendere.

### Memoria

La memoria è gestita dal **Virtual Memory Manager** (_VMM_) contenuto nell'_executive_, che usa ottimizzazioni come il [**prefetching**](../04/01/README.md#sostituzione), il [**copy on write**](../04/01/README.md) e la **lazy allocation** per allocarle solo quando vengono usate.

Le pagine hanno dimensione [**fissa**](../04/01/README.md) e sono salvate su [due livelli](../../ct0615-2/03/README.md#page-table-multilivello), il primo dei quali contiene molteplici **Page Directory Entry** (_PDE_) che puntano ad una _page table_ differente con le rispettive **Page Table Entry** (_PTE_).

L'**allocazione** delle pagine avviene effettuando:
1. **Reserve**: la _VMM_ riserva lo spazio di memoria richiesto del processo
2. **Commit**: viene creata una _PTE_ dello spazio riservato
3. **Access**: il processo accede alla memoria

che se _contigue_ potranno essere **raggruppate** dal _VMM_ per gestirle come singola, riducendo l'uso della [TLB](../../ct0615-2/03/README.md#translation-lookaside-buffer).

La [**sostituzione**](../04/01/README.md#sostituzione) avviene con l'algoritmo _LRU_ su un _working set_ scelto dal _VMM_, con _swapping_ sulle pagine nella zona **paged pool** di memoria e senza per quelle nella zona **nonpaged pool** dedicata al _kernel_.

### Filesystem

L'_hardware_ delle unità esterne sono gestite dai **volume driver**, su cui sono poi implementati i **filesystem driver** a cui possono essere applicati i **filesystem filter driver** per operazioni come la compressione.

Un esempio è il _filesystem_ **NTFS**, in cui ogni file ha una **Master File Table**, composta da **attributi** che sono **non residenti** se sono altrove sul disco, ed è formato da più gruppi di pagine chiamati **cluster** contenenti:
- **Virtual Cluster Number**: posizione relativa all'inizio del file a cui appartiene
- **Logical Cluster Number**: posizione assoluta sul volume

Il contenuto dei file è salvato sulla loro **default data stream** principale, ma possono avere anche altri **alternate data stream** associati ad un nome per memorizzare dati secondari come i [metadati](../05/README.md).
