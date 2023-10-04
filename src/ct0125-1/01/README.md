# Processi e thread

Quando una _singola CPU_ è presente viene usato lo **pseudo parallelismo**, per cui ad ogni processo è assegnato uno alla volta una _CPU virtuale_ con _registri_ isolati.

Anche lo **spazio di indirizzamento** è isolato, e contiene le regioni:
- `text`: contente il **codice** del programma
- `data`: contente le **variabili** del programma
- `stack`: contente gli **stack frame** (i.e. variabili locali) delle chiamate delle funzioni in esecuzione

Riguardo i _processi_ i sistemi operativi permettono la:
- **Creazione**

	Un processo è _creato_ durante l'**inizializzazione** del sistema o tramite una **chiamata di sistema**.

	Sono considerati in **foreground** se in uso da utenti, altrimenti sono in **background** (o _demoni_) se forniscono servizi come la gestione di stampa.

	Su _Unix_ un processo può essere **clonato** con la funzione `fork`, copiando l'intera memoria e registri ma mantenendo _spazi di indirizzi_ separati, mentre su _Windows_ può essere **generato** con `CreateProcess`.

- **Chiusura**

	Un processo può essere _chiuso_ per un uscita:
	- **normale**: come `exit(0)`
	- **con errore**: come `exit(1)`
	- **forzata con un'eccezione**: come con `SIGSEGV` (i.e. _Segmentation fault_)
	- **forzata da un altro processo**: come con `SIGKILL`

	All'uscita di un processo con _figli_, il S.O. può o **distruggere i figli** o **consentire la loro esecuzione**.
	Contrariamente a _Unix_, _Windows_ non possiede un concetto di _gerarchia di processi_.
