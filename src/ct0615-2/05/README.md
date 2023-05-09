# I/O

I sistemi di **Input**/**Output** servono per permettere alla _CPU_ di comunicare con _periferiche_ esterne (e.g. mouse, tastiera, bluetooth).
Questo avviene tramite un **bus**, cioè dei fili di collegamento tra componenti.

Tra i fili del _bus_, ci sono:
- Fili di **controllo**: che si occupano delle richieste di _I/O_
- Fili per **dati**: dedicati al trasferimento di dati

Data la latenza che si genera per la _distanza_ e le richieste di accesso al _bus_, il **northbridge** (i.e. _memory controller_) è integrato nella _CPU_ per cui è direttamente connessa alla _RAM_ e alla _GPU_, mentre il **southbridge** (i.e. _I/O controller_) è connesso alla _CPU_ tramite un bus più lento.

## Bus sincrono e asincrono

I dispositivi che comunicano su un **bus sincrono** dispongono dello **stesso clock** e lo sfruttano per accordare gli accessi al _bus_.
Questo però è limitato a bus **corti**, perchè altrimenti il segnale di _clock_ si disallinea.

In un **bus asincrono** invece, ogni dispositivo ha un **clock diverso**, che permette il trasferimento a **lunghe distanze** e a **velocità diverse** (e.g. USB e SATA).
Questo richiederà un _costo_ iniziale dato dall'**handshake** per sincronizzare i due dispositivi, che avviene su due fili aggiuntivi chiamati **Req** e **Ack**.

La **richiesta di invio dati** `req` è impostata dal _mittente_, mentre la **conferma di lettura** `ack` dal _destinatario_:
```c
bool req = 0, ack = 0;
char data[2];	// 16+2 bit di fili condivisi

void sender() {
	update(data); req = 1;
	while (ack == 0);
	req = 0;
}

void receiver() {
	while (req == 0);
	read(data); ack = 1;
	while (req == 1);
	ack = 0;
}
```
