# Controller

Per poter trasferire dati con un dispositivo, il sistema operativo dovrà comunicare con il **controller** inviandogli _comandi_ e leggendo il suo _stato_, operazioni descritte all'interno del **driver**.

Per comunicare con la _CPU_, il controller possiede dei registri:
- **Command**, per inviare comandi
- **Status**, per leggere lo stato del dispositivo
- **Data Write**, per inviare i dati
- **Data Read**, per leggere i dati

che vengono utilizzati attraverso `load` e `store` su _indirizzi virtuali_, se la _CPU_ usa **Memory Mapped I/O**, oppure attraverso **istruzioni speciali** se l'_ISA_ lo supporta.

## Polling

In questo metodo, il processore **controlla periodicamente** il registro di stato per capire se ci sono dati da trasferire, e nel caso li scrive in _RAM_ per utilizzi futuri.

Diventa vantaggioso in caso di **trasferimenti** ad intervalli **prevedibili**, anche perchè il costo per la lettura del _registro di stato_ è noto.
Rimane comunque costoso occupare la _CPU_ nel trasferimento di dati in _RAM_.

## Interrupt-driven

Questa alternativa al _polling_ sfrutta gli **interrupt** per notificare il processore in caso di nuovi dati disponibili.

In questo caso è vantaggioso nel caso di **rari trasferimenti** ad **intervalli imprevedibili**, perchè altrimenti la _CPU_ continuerebbe a saltare tra il processo e l'[interrupt handler](../../01/04/README.md).

## Direct Memory Access

I metodi precedenti spendono la maggior parte del tempo per trasferire i dati sulla _RAM_.
Questo può essere risolto delegando il compito ad **hardware dedicato**, cioè il **Direct Memory Access**, salvando tempo _CPU_.

L'unico incarico della _CPU_ sarà specificare la quantità dei dati e gli indirizzi coinvolti al _DMA_, per poi ricevere un _interrupt_ quando il trasferimento è completato.

Dato che la comunicazione con la _RAM_ è diretta, il _DMA_ non usa il sistema di traduzione della _CPU_, e quindi:
- la _scrittura_ in _RAM_ **non aggiornerebbe la cache**, mentre la lettura da _RAM_ potrebbe essere **obsoleta** se un _write-back_ è in sospeso.

	Per risolvere il problema le alternative sono:
	- Passare l'_I/O_ del _DMA_ **attraverso la cache**, sprecando però spazio e invalidando dati utili alla _CPU_
	- Far **invalidare** al _SO_ il dato su _cache_ per richieste _I/O_ in lettura (i.e. scrittura su _RAM_) e **forzare** un _write-back_ per richieste in scrittura (i.e. lettura da _RAM_).
	- Modificare l'**hardware per invalidare** dei blocchi della cache

- gli _indirizzi virtuali_ comporterebbero l'**uso della page table** (in _RAM_) e la [TLB](../../03/README.md#translation-lookaside-buffer) che avrebbe gli **stessi problemi della cache**, mentre gli _indirizzi fisici_ **limiterebbero** i dati trasferiti **alla dimensione di una pagina** perchè la _memoria virtuale_ non è contigua nella _memoria fisica_.

	Questi problemi possono essere risolti:
	- incaricando il _SO_ di fornire **tabelle per la traduzione** degli indirizzi virtuali
	- **frammentando** il trasferimento in sotto-trasferimenti grandi quanto le pagine

	forzando quindi il _SO_ a collaborare per trasferimenti più grandi.
