# Thread

I **thread**, al contrario dei _processi_, **condividono**:
- lo **spazio di indirizzamento**, e quindi l'intera _memoria_ del processo
- i **file aperti**

ma mantenendo comunque **isolati**:
- i **registri**, per eseguire codice diverso
- lo **stack**, per salvare variabili diverse
- le **maschere dei segnali**

Dato che la _memoria_ è condivisa, la **creazione** di un _thread_ è **meno impegnativa** di quella di un processo.

Il **ciclo di vita** è analogo ai [processi](../01/README.md#ciclo-di-vita), eccetto per la presenza di altri stati oltre a _bloccato_, ovvero **waiting** per l'attesa di un evento da un'altro thread e **sleeping** per l'attesa di un tempo specifico.

Oltre alle [operazioni dei processi](../01/README.md#operazioni), supportano anche la:
- **Cancellazione**: richiesta di **terminazione** al _thread_, che però potrebbe **mascherarla**
- **Join**: il _thread_ che richiede il _join_ viene **bloccato** fino all'uscita del _thread_ su cui viene richiesto

Su _Linux_, i _thread_ e i _processi_ sono entrambi chiamati **task**, infatti possono entrambi essere generati con la funzione `clone`, che con i giusti parametri può avere lo stesso effetto di `fork`.

Su _Windows_ invece, esistono i **fiber**, cioè delle sottounità di esecuzione nel contesto del _thread_ padre, che vengono _prelasciati_ assieme al _thread_ e _schedulati_ saltando da un _fiber_ ad un'altro.

## Modelli di threading

Tra i **modelli** esistenti, i _thread_ possono essere:
- **A livello utente** (_molti-a-uno_)

	In cui ogni _thread_ vive all'interno dello **stesso contesto di esecuzione** del processo padre, e quindi ogni processo conterrà la sua **tabella dei thread**.

	Questo permette ai S.O. senza _thread_ interni di **supportarli** in modo **portabile**, ammettendo anche la **scelta** dell'_algoritmo di scheduling_ dall'utente, ed evitando l'overhead del _cambio di contesto_.

	Lo svantaggio sono le **prestazioni limitate**, perchè non sono schedulati su più processori dato che il _kernel_ li vede come un **singolo thread**, ma anche il **blocco del processo** intero se un _thread_ fa _I/O_.

- **A livello kernel** (_uno-a-uno_)

	In questa situazione ogni _thread_ avrà il **proprio contesto di esecuzione**.
	Per cui il nucleo conterrà la **tabella dei thread**, oltre a quella dei processi, e applicherà le operazioni con _chiamate di sistema_.

	Questo **migliora le prestazioni** ma **riduce la portabilità**, visto che ogni S.O. avrà interfacce diverse.

- **Ibridi** (_molti-a-molti_)

	Implementando un **thread pool** (i.e. un gestore di _thread_) si possono combinare i due precedenti, per cui per ogni processo i _thread_ potrebbero appartenere allo **stesso contesto** o essere suddivisi su **altri**.

	I contesti su cui sono eseguiti sono chiamati **thread worker**, e sono persistenti nel _kernel_.

	Lo scheduling dei _thread_ a _livello utente_ avviene tramite l'intervento di un **processore virtuale**, assegnato al processo dal nucleo e **notificato** per passare a _livello utente_ quando un _worker_ si blocca.
