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

	Divennero a **multiprogrammazione**, alternando l'esecuzione tra i _job_ della _pool_ (i.e. insieme di _job_ da eseguire), cosa che porto all'invenzione dei **primi algoritmi di scheduling**.

	All'inizio veniva cambiato _job_ mentre _aspettava il completamento di operazioni I/O_, i.e. **spooling**.
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
