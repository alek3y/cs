# IO

I sistemi di **Input**/**Output** servono per permettere alla _CPU_ di comunicare con _periferiche_ esterne (e.g. mouse, tastiera, bluetooth).
Questo avviene tramite un **bus**, cioè dei fili di collegamento tra componenti.

Tra i fili del _bus_, ci sono:
- Fili di **controllo**: che si occupano delle richieste di _IO_
- Fili per **dati**: dedicati al trasferimento di dati

Data la latenza che si genera per la _distanza_ e le richieste di accesso al _bus_, il **northbridge** (i.e. _memory controller_) è integrato nella _CPU_ per cui è direttamente connessa alla _RAM_ e alla _GPU_, mentre il **southbridge** (i.e. _IO controller_) è connesso alla _CPU_ tramite il bus **DMI**.
