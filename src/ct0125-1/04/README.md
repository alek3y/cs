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

## Overlay

La tecnica di **overlay** consiste nel dividere il programma in **sezioni** e alternativamente caricare quelle **attive** nella stessa area _contigua_ di memoria.

Questo però **complica** l'organizzazione delle sezioni, perchè quelle che interagiscono non possono essere sovrapposte, ma anche perchè le modifiche al programma possono renderle non sovrapponibili.

## Protezione

Sulla memoria si necessita anche della **protezione**, perchè il S.O. non deve essere riscrivibile dall'utente.

In un'ambiente **mono utente**, una tecnica consiste nel far salvare al _kernel_ gli **indirizzi limite** della memoria del programma in dei registri protetti, così da negare l'accesso alla memoria oltre ai limiti.

Un esempio è un sistema ad elaborazione **batch**, in cui solo un programma è inserito in memoria.
