# Sistemi operativi

Al **sistema operativo** è assegnato il compito di **controllare** il _tempo_ e lo _spazio in memoria_ di:
- risorse hardware, gestite in modalità **kernel**
- applicazioni software, eseguite in modalità **utente**

L'astrazione delle risorse permette di rendere i _software_ indipendenti dall'_hardware_.

## Storia

L'evoluzione dei _sistemi operativi_ è suddivisa in:

1. **Iᵃ generazione** (1945-1955):

	All'inizio non possedevano un S.O. e venivano riprogrammati cablando i circuiti.

	Successivamente, vennero introdotti S.O. a **monoprogrammazione**, che permettevano la lettura e l'esecuzione di **un job** (i.e. programma) **alla volta**.

2. **IIᵃ generazione** (1955-1965):

	Si diffusero S.O. **batch** (_single-stream batch-processing_), che permettevano la lettura di più _job_, assieme ai rispettivi dati in input, e la loro **esecuzione consecutiva** senza alcun intervento.

	In memoria era solamente presente il compilatore e successivamente il programma, oltre che il S.O.

3. **IIIᵃ generazione** (1965-1980):

	Divennero a **multiprogrammazione**, i.e. _CPU_ condivisibile da più utenti, alternando l'esecuzione tra i _job_ della _pool_ (i.e. insieme di _job_ da eseguire), cosa che portò ai **primi algoritmi di scheduling**.

	All'inizio veniva cambiato _job_ mentre _aspettava il completamento di operazioni I/O_, i.e. **spooling** usando il disco come _buffer_ per i dati da scrivere.
	In seguito venne introdotto il **time-sharing**, per cui lo _scheduler_ da un _tempo fisso_ ad ogni _job_, per poi effettuare un **cambio di contesto**.

	Avendo ogni processo in memoria, fu necessaria l'invenzione della [**memoria virtuale**](../ct0615-2/03/README.md).

	Per i _job_ che necessitano del controllo in tempi precisi, vennero introdotti i sistemi **real-time**.

	Inoltre, fu in questo periodo che venne sviluppato **Internet** con il **TCP/IP**.

4. **IVᵃ generazione** (1980-):

	Con i **personal computer** e le **workstation** furono introdotte le **interfacce grafiche** e il **multithread**.

	Tra i nuovi sistemi vennero sviluppati i:
	- sistemi _paralleli_, cioè aventi più _CPU_
	- sistemi _distribuiti_, cioè composti da più computer connessi

	Sopra _Internet_ venne diffuso il modello **client/server** e il **World Wide Web**.

	Nella programmazione invece, si diffuse l'**Object Oriented Programming** e il software **open source**.

4. **Vᵃ generazione** (1990-):

	In questo periodo si sviluppò la tecnologià per la **mobilità**, come i S.O. mobili e il risparmio energetico.

## Componenti

La comunicazione tra dispositivi avviene attraverso i **bus**, cioè un insieme di **tracce** o collegamenti elettrici.
Nelle trasmissioni di due dispositivi, il _bus_ è detto **porta**.
Quando sono di più invece, è detto **canale** di _I/O_.

Tra i _bus_ più importanti, con velocità nell'**ordine dei MHz**, ci sono:
- **USB** (_Universal Serial Bus_): dedicata a dispositivi _plug and play_ lenti
- **FSB** (_Front-side bus_): collega la _CPU_ alla _RAM_
- **SATA** (_Serial AT Attachment_): per la connessione di dischi esterni
- **SCSI** (_Small Computer System Interface_): alternativa a _SATA_
- **PCIe** (_Peripheral Component Interconnect Express_): collegamento standard per periferiche aggiuntive
- **DMI** (_Direct Media Interface_): connette la _CPU_ con il controller di periferiche
- **AGP** (_Accelerated Graphic Port_): per il collegamento di schede video

Mentre, tra i componenti **software** del S.O. ci sono:
- **Scheduler**
- Gestore della **memoria**
- Gestore del **I/O**
- Gestore dell'**IPC**
- Gestore del **filesystem**

## Tipi

Ogni S.O. è progettato in base al suo scopo, tra cui in particolare:
- **Embedded** (e.g. elettrodomestici): in grado di gestire efficacemente le risorse su dispositivi limitati
- **Smartphone** (o _PDA_ con _telefonia_)
- **Real-time**: per programmi che richiedono poco timeout
- **Server**: supportano molteplici connessioni e servizi
- **Multiprocessore**: in grado di gestire la _coerenza_ tra processori
- **Mainframe**: per potenti computer centralizzati
- **Distribuito**: gestisce le risorse di più computer e tollera guasti, un _middleware_ ne semplifica lo sviluppo

Come struttura invece, può essere:
- **monolitico**: le _componenti_ fanno parte del _kernel_, per cui è _efficiente_ ma difficile da _debuggare_
- **a livelli**: migliora il sistema _monolitico_ isolandolo in livelli, come il _THE_ (_Technische Hogeschool Eindhoven_)

	```dot process
	digraph {
		rankdir=LR
		node [shape=record fontname="Times"]
		System [label="4. Applicazioni utente (user space)\l | 3. Gestione I/O (kernel space)\l | 2. Gestione IPC\l | 1. Gestione della memoria\l | 0. Scheduling\l"]
	}
	```

- **microkernel**: _limitato_ ma _modulare_ e quindi _estensibile_, per cui i livelli comunicano in stile _client/server_

## Architettura

### Protezione

Tra i meccanismi di protezione di un S.O., il processore:
- Impedisce ai processi di accedere ad area di memoria protette
- Implementa le modalità **kernel** e **utente**
- Gestisce **interrupt** ed **eccezioni** generati dai dispositivi passando il controllo al S.O.

Due esempi di dispositivi che generano _interrupt_ sono il **timer**, che ne genera uno periodicamente per evitare che i processi usino il processore troppo a lungo, ed il **clock**, che fornisce l'_ora del giorno_.

### Avvio

L'avvio invece, avviene tramite il **bootstrapping**, per cui:
1. Il **BIOS** (_Basic Input Output System_) inizializza l'hardware
2. Carica il **boot sector** dal disco in memoria
3. Il codice viene eseguito che **carica il S.O.** dal disco

dove il _BIOS_ è oggi rimpiazzato dall'**EFI** (_Extensible Firmware Interface_).

### Processi

Un **processo** è un programma in esecuzione che possiede:
- uno **spazio degli indirizzi**
- risorse tra cui **registri**, **file descriptor** e **segnali**
- un **descrittore**
- un **UID** (_User Identifier_) dell'utente che ha avviato il processo

Tra i registri, esiste il **Program Status Word** contenente lo _stato_ e la _modalità_ in cui si trova il processo.

Per eseguire istruzioni privilegiate, il processo genera una **trap** (i.e. un'eccezione che porta la _CPU_ in modalità _kernel_ passando il controllo al S.O.) tramite cui viene eseguita una **system call**.
Un esempio di _system call_ è l'accesso ad un dispositivo _I/O_ da parte di un processo.

Possono anche comunicare tra di loro tramite una tecnica chiamata **IPC** (_Inter-process Communication_).

### Filesystem

Tra i costrutti supportati dai filesystem **unix**, ci sono:
- **File speciali** tra cui **dispositivi a blocchi** su cui la scrittura avviene a _blocchi_ che possono essere interpretati da dei _driver_, e i **dispositivi a caratteri** su cui la scrittura avviene _un byte alla volta_
- **Pipe** dei _file buffer_ che permettono lo scambio di informazioni tra processi
