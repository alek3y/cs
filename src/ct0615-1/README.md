# Architettura degli elaboratori (M. 1)

## Von Neumann

Tutte le macchine moderne si basano sulla semplice architettura di _Von Neumann_, che è composta da:
- **Processore**: che ha il compito di processare gli input
- **Memoria**: su cui vengono salvati i dati e il programma (concetto di _stored-program_, introdotto da questa architetettura)
- **Periferiche**: che si interfacciano con l'esterno

In un computer, la memoria può essere:
- **Principale**, come la _RAM_, che è **volatile** (per cui i dati vengono persi al riavvio)
- **Secondaria**, come un _Hard Disk_, che è **non volatile**

Oltre alla memoria sono presenti anche dei **bus**, cioè i fili che fanno da canale per la comunicazione dei vari componenti.

## Astrazione

L'informatica si basa sull'**astrazione**, cioè il processo che permette di **semplificare** tutti i particolari di un componente più complesso, così che possa essere utilizzato come in modo semplice senza doverne conoscere il contenuto.

Questo processo però, causa l'omissione di tutti i dettagli a basso livello che riguardano quel componente.

In un processore, l'astrazione è fornita dal suo **Instruction Set Architecture**, tramite cui è possibile dialogare.

### Livelli

0. _Hardware e firmware_
	1. Hardware: composti da fili e porte logiche
	2. Firmware: basato sulle porte logiche traduce ed esegue le istruzioni dell'_ISA_
1. Linguaggio macchina (_ISA_): è un linguaggio semplice per la macchina, dato che permette la facile individazione delle **istruzioni** e degli **operandi**
2. Sistema operativo: integra l'_ISA_ per permettere la facile gestione di più risorse
3. Assembler
4. Linguaggio ad alto livello: permette la **portabilità** tra diversi _ISA_
