# Eccezioni e interrupt

Un altro problema di tipo **control hazard**, come per le `beq`, sono le **eccezioni** e le **interruzioni**.

Le **eccezioni** sono problemi (e.g. overflow, opcode invalido, ...) **causati dal codice in esecuzione** dentro le _unità_, mentre gli **interrupt** sono eventi **generati da periferiche** che richiedono l'attenzione del processore.

Per entrambi, la gestione consiste nel:
1. **Interrompere** l'istruzione corrente e successive inserite nella _pipeline_
2. **Salvare** il _PC_ dentro il registro **EPC** (_Exception Program Counter_)
3. **Trasferire il controllo** al sistema operativo su un indirizzo _hard-coded_

Durante il _trasferimento del controllo_, il **tipo di eccezione** può: o essere **salvato su un registro** speciale (e.g. CAUSE per il MIPS), o il salto può avvenire ad indirizzi diversi a seconda dell'eccezione.

Per evitare che la gestione degli errori si blocchi, gli _interrupt_ possono essere **disabilitati** per brevi periodi.
Inoltre, nel caso in cui _unità_ diverse generassero eccezioni diverse, serve un **sistema di priorità** per gestirle.
