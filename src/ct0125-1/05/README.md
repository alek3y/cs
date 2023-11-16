# Filesystem

Un **filesystem** è orientato ai **record** quando contiene dei **file** composti da molteplici _record_, ovvero più insiemi di **campi** ognuno dei quali è composto da un gruppo di caratteri.

Si dice orientato ai **byte** invece, quando i _file_ sono una sequenza di singoli _byte_ e non di _record_.

Il compito del _filesystem_ è di **organizzare** e gestire il contenuto e lo spazio della _memoria secondaria_, rendendolo **indipendente** dal dispositivo hardware e prevenendo la perdita delle informazioni.

Nel disco salva anche **copie ridondanti** non modificabili dagli utenti del **super blocco**, contenente **metadati**, cioè informazioni critiche come la posizione della radice e dei blocchi liberi.

I tempi del disco sono riducibili **prevedendo** i blocchi da leggere, sfruttando la [cache](../../ct0615-2/02/README.md) e deframmentandolo.

## File

Il _filesystem_ permette un **accesso** ai file in modo:
- **Sequenziale**, se i _record_ sono letti dall'inizio alla fine del file
- **Causale** (o _diretto_), se ammette salti nella lettura del file

Ogni _record_ **fisico** può contenere più _record_ **logici**, cioè dei dati trattati come _record_ dal software.
Quando ne contiene molteplici i _record_ sono detti **bloccati**, altrimenti vengono detti **non bloccati** (o _sbloccati_).

## Directory

Le **directory** sono dei file contenenti una lista di **nomi** e **posizioni** di altri file nel _filesystem_.

Ogni _nome_ può essere salvato a lunghezza **fissa** come una sequenza di caratteri, oppure **variabile** salvando sia la lunghezza che il nome o salvando l'indirizzo a lunghezza _fissa_ del nome a lunghezza _variabile_ sull'_heap_.

Possono avere ruoli diversi a seconda del _filesystem_:
- **Piatto**: contiene **una sola** _directory_, ogni file ha un nome distinto e la ricerca dei file è lineare
- **Gerarchico**: contiene una **radice** per la _directory_ principale che punta alle varie _directory_ contenenti file

### Link

Nelle _directory_ si possono inoltre creare dei **link**, che fanno da riferimento a file in locazioni diverse:
- **Soft**: contiene il **percorso** del file e perde il suo riferimento quando viene spostato
- **Hard**: punta allo **stesso blocco** fisico del file e perde il riferimento se viene riallocato

### Mount

Fare il **mount** di un altro _filesystem_ permette di accederci da un'unica _radice_ su una _directory_ che fa da **punto di mount**, le cui informazioni vengono salvate dentro **tabelle di mount** del _filesystem_.

Supportano i _soft link_ tra dispositivi, ma non gli _hard link_ dato che i dati dovrebbero essere altrimenti copiati.

## Allocazione

Come per la [memoria principale](../04/README.md), si presenta il problema dell'**allocazione** dello spazio, che può essere:
- **Contigua**

	Alloca i dati su **indirizzi contigui**, disponendo i _record_ adiacenti tra loro.

	Questo conviene su unità _read-only_ come i _CD-R_, ma **peggiora** le prestazioni se il file cambia di dimensioni e aumenta la _frammentazione esterna_.

- **Con liste collegate**

	Alloca il file in maniera **non contigua** attraverso una _linked list_ di **settori**, ognuno dei quali punta al successivo e l'indirizzo del primo viene salvato all'interno della _directory_.

	Durante l'allocazione del primo blocco, il sistema sceglie un'area di memoria **contigua**.

	Utilizzare **grandi** blocchi permette di ridurre l'_I/O_ ma al costo di aumentare la _frammentazione interna_, mentre blocchi **piccoli** sprecano più spazio per la grande quantità di puntatori salvati.

	Per trovare un _record_ però, la ricerca deve partire dal primo blocco, e può essere molto **lenta** se i blocchi sono sparsi nell'unità perchè vanno trovati e copiati in memoria principale.

- **Tabellare**

	Come _FAT32_, salva i riferimenti ai blocchi in una **tabella** globale a parte, assieme all'indice del blocco successivo.
	L'indice del primo blocco di ogni file sarà poi salvato nella _directory_ in cui risiede.

	Questo evita di dover caricare ogni blocco in memoria per la ricerca, migliorando i tempi di accesso.
	Nel caso contenga molti blocchi però, può diventare molto grande e ridurre le prestazioni del _filesystem_.

- **Indicizzata**

	Come l'allocazione _tabellare_, ma ogni file ha la sua _tabella_ chiamata **blocco indice** che oltre ai blocchi contiene anche il puntatore ad una possibile tabella successiva.

	Su _Linux_ i _blocchi indice_ sono detti **inode**.

## Gestione spazio

Normalmente la dimensione dei blocchi è **costante**, ma più sono grandi meno tempo sarà occupato a passare ai successivi, al costo di maggior _frammentazione interna_ e di conseguenza possibile spreco di spazio.

Il disco può anche essere suddiviso in **quote** assegnate ai diversi utenti, limitando i file memorizzabili.

### Spazio libero

Lo **spazio libero** può essere gestito usando una **lista libera**, cioè una _linked list_ contenente i blocchi liberi che verranno allocati partendo dalla testa e liberati aggiungendoli in coda.

Questo porta ad un overhead molto basso, ma per evitare di allocare blocchi **sparsi** andrà forzato il _filesystem_ ad effettuare una ricerca di blocchi vicini dentro la lista.

Un'alternativa è usare una **bitmap** di ogni blocco, in cui i singoli bit indicano se il blocco è libero o occupato.
Peggiora però l'overhead associato alla ricerca di blocchi liberi.

## Backup

Una tecnica per salvare **copie** ridondanti di dati è l'esecuzione di **backup periodici**, che possono essere:
- **Fisici**: duplicano i dati bit per bit, semplificando il processo ma riducendo la portabilità
- **Logici**: memorizzano i dati e la loro struttura nel _filesystem_, spesso comprimendoli

Un esempio di backup _logico_ sono i **backup incrementali**, che copiano i file modificati dall'ultimo backup.

## Integrità

Una tecnica dei _filesystem_ per mantenere la **consistenza** dei dati è effettuare modifiche con le **transazioni**, cioè un gruppo **atomico** di più operazioni di cui viene fatto il **rollback** in caso di errori.

Sono realizzati scrivendo lo stato delle operazioni in un file di **log**, e prendendo nota del completamento della _transazione_.
Il _rollback_ avverrà quindi all'ultimo **checkpoint**, cioè all'ultimo stato consistente del disco.

Inoltre alcuni _filesystem_ implementano la **paginazione shadow**, che effettua le operazioni su un blocco libero, per poi copiarle sul blocco originale alla fine della _transazione_.

Un esempio sono i **log structured filesystem** come _NTFS_ e _ext3_, che rendono l'intero disco il file di _log_ e scrivono i nuovi dati in sequenza nello spazio libero, causando però scarse prestazioni in lettura.

La ricerca sull'intero disco è evitabile salvando le posizioni dei file nella **cache** o dedicando un _log_ ai _metadati_.

Inoltre il _filesystem_, essendo _frammentato_, dovrà compattare i dati alla fine del file _log_ per ricavare spazio.

## Controllo accessi

Un altro compito è quello di **proteggere** i dati dagli utenti, attraverso **privilegi** come _Read_, _Write_ e _Execute_.
Nel _filesystem_, l'insieme dei _privilegi_ e il relativo file è detto **dominio di protezione**.

La gestione dei vari _domini_ avviene quindi con meccanismi come:
- **Access control matrix**

	Salva i _privilegi_ in una matrice contenente gli **utenti sulle righe** e i **file sulle colonne**.

	Può diventare **inefficiente** con l'aumento dei file e degli utenti, ed è critico mantenerla protetta.

- **Access control list**

	Salva i _privilegi_ all'interno di **liste di utenti** (per ogni file), oppure **liste di file** (per ogni utente).

	Risparmia spazio rispetto alla _matrice_ perchè non possiede elementi vuoti, ma rallenta la ricerca.

- **Capability list**

	Salva nella lista dei **capability**, cioè identificatori che permettono l'accesso agli utenti che li possiedono.
	Ogni _capability_ sarà quindi creato per un oggetto e assegnato ai vari utenti.
