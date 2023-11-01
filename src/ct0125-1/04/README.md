# Memoria

Le **gerarchie** di memoria si suddividono in:
- [**Memoria cache**](../../ct0615-2/02/README.md): la più veloce e la prima a cui la _CPU_ accede
- **Memoria principale**: contenente i dati volatili dei programmi in esecuzione
- **Memoria secondaria**: necessita la copia dei dati in memoria _principale_

La **gestione** della memoria consiste nel decidere **quando** copiare i dati di un programma (i.e. **fetch**), **dove** collocarli (i.e. **posizionamento**) e a **chi** rimpiazzare i dati se non c'è abbastanza spazio (i.e. **sostituzione**).

L'**allocazione** può essere:
- **Contigua**:

	Utilizzata se vanno memorizzati i dati in un **unico blocco** di indirizzi sequenziali.
	Se però è richiesta una grande porzione di memoria può essere **impossibile** trovare un blocco libero abbastanza grande.

	Sui vecchi sistemi **mono utente**, l'intera memoria era allocata al singolo utente e la gestione era compito del programmatore, anche con sistemi di terze parti come **IOCS** (_Input/Output Control System_).

- **Non contigua**:

	Divide la memoria del programma in **segmenti**, che si possono trovare in parti diverse della memoria.
	Questo **aumenta** il numero di processi in memoria, ma aumenta anche l'_overhead_.

## Organizzazione

Se l'ambiente è **mono utente**, il singolo processo utilizza tutta la memoria libera disponibile.

Nel caso di insufficiente memoria _contigua_ disponibile esiste la tecnica di **overlay**, che consiste nel dividere il programma in **sezioni** e alternativamente caricare quelle **attive** nella stessa area _contigua_ di memoria.

Questo però **complica** l'organizzazione delle sezioni, perchè quelle che interagiscono non possono essere sovrapposte, ma anche perchè le modifiche al programma possono renderle non sovrapponibili.

La **protezione** del S.O. avviene salvando gli **indirizzi limite** della memoria del processo in un registro protetto, così da lanciare un'[eccezione](../../ct0615-2/01/04/README.md#eccezioni-e-interrupt) nel caso vengano superati.

Se l'ambiente è a **multiprogrammazione** invece, la memoria può essere organizzata in:
- **Partizioni fisse**

	Inizialmente ogni programma veniva compilato con indirizzi **assoluti**, di conseguenza ad ogni processo caricato veniva assegnata una _partizione_ di memoria con posizione e dimensione _fissa_.

	Questo semplificava il parallelismo, ma forzava l'**attesa** ai processi da caricare sulla stessa _partizione_.

	L'alternativa è stata l'introduzione di compilazione con indirizzi **rilocabili**, per cui ogni processo è caricato dinamicamente su _partizioni_ diverse.

	La _protezione_ del S.O. e gli altri processi avviene associando ad ogni _partizione_ un **registro base** e **limite** che limitano gli indirizzi accessibili in memoria dal processo.

	Un lato negativo è la **frammentazione interna** alle _partizioni_, dato che la memoria non in uso dal processo nella _partizione_ non può essere in alcun modo usata.

- **Partizioni variabili**

	L'alternativa alle _partizioni fisse_ è creare nuove _partizioni_ della **stessa dimensione** del processo.

	Questo risolve la _frammentazione interna_, ma introduce **frammentazione esterna** causata dai buchi lasciati in memoria quando il processo termina.
	Tra i modi per risolvere, ci sono:
	- **Coalescenza**: combina più blocchi liberi vicini in un singolo blocco
	- **Compattazione**: compatta i blocchi in uso in memoria, comportando _overhead_ significativo

	Inoltre, la scelta di posizione in memoria avviene secondo le **strategie** di:
	- **Best-fit**: nel più piccolo spazio in grado di contenere il processo
	- **First-fit**: nel primo spazio libero, riducendo l'_overhead_
	- **Worst-fit**: nello spazio più grande disponibile, lasciando un grande buco per il prossimo processo

	Per poter **gestire la memoria libera** però, vanno salvate le informazioni sulle partizioni.
	Una tecnica consiste nel salvare una **bitmap**, dove ogni bit indica lo stato di un'unità di memoria, un'altra nell'usare una **linked list**, dove ogni elemento è una _partizione_ di un processo o di memoria libera.

- **Swapping**

	Invece di avere una _partizione_ per processo, si tiene l'intera memoria per il **singolo processo** in esecuzione, spostando gli atri temporaneamente nella _memoria secondaria_.

	Questo comporta grande _overhead_ al cambio di contesto, dato che è richiesto l'accesso in _memoria secondaria_.
	Di conseguenza, una soluzione può essere l'uso di _partizioni variabili_ con _swapping_.

## Memoria virtuale

La [**memoria virtuale**](../../ct0615-2/03/README.md) permette l'uso di un **indirizzamento virtuale** molto più ampio di quello **reale**.

Blocchi _virtuali_ contigui vengono tradotti in blocchi _reali_ non necessariamente contigui, e possono essere:
- **Pagine**, con la **paginazione**, se ogni blocco ha dimensione _fissa_
- **Segmenti**, con la **segmentazione**, se ogni blocco può avere dimensione _diversa_
