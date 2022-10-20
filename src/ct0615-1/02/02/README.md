# Minimizzazione

Attraverso questo processo è possibile **ottimizzare** il circuito in forma canonica, e quindi ridurre le operazioni logiche da effettuare.

Gli input che **non influenzano l'output** (uguale per $2^n$ righe), sono chiamati **don't care** e possono essere ignorati nell'equazione.

Per esempio, con la tabella di verità:

| A | B | C | E |
|:-:|:-:|:-:|:-:|
| 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 0 |
| 0 | **1** | 0 | **1** |
| 0 | **1** | 1 | **1** |
| 1 | 0 | 0 | 0 |
| 1 | 0 | 1 | 0 |
| 1 | **1** | 0 | **1** |
| 1 | **1** | 1 | **1** |

ci sono due gruppi di bit output a $1$ grandi $2 = 2^1$ e quindi sono validi.
Come si può notare, l'input $A$ e $C$ sono **don't care**, perchè non hanno influenza sull'output.

Di conseguenza l'equazione diventa:
$$E = B$$

## Mappe di Karnaugh

Un'altra tecnica usata per la _minimizzazione_ sono le **mappe di Karnaugh**.

Queste mappe consistono nel creare una tabella di verità del tipo:

| C\AB | 00 | 01 | 11 | 10 |
|:-:|:-:|:-:|:-:|:-:|
| **0** | 0 | 1 | 1 | 0 |
| **1** | 0 | 1 | X | 0 |

dove la prima cella delle due ultime righe è il valore di $C$, mentre le altre celle sono gli output dell'equazione.

Le combinazioni di $A$ e $B$ sono messe come $00$, $01$, $11$ e $10$ perchè rispettano il **codice di Gray**, cioè hanno al massimo un bit di distanza, per cui rendono la _mappa_ valida.

Quando nella tabella troviamo un valore non definito $X$, la funzione si dice **funzione incompleta**, per cui l'output è **don't care** e si può scegliere il valore che permetta la miglior minimizzazione.

In quel caso, si metterà $X = 1$, perchè così è possibile creare un blocco di $4 = 2^2$ output, da cui si nota che $A$ e $C$ sono **don't care** (perchè pur cambiando non influenzano l'output) mentre $B$ sarà l'input da cui costruire l'equazione:
$$E = B$$

I gruppi possono anche essere costruiti **collegando i bordi** della tabella, per esempio con:

| CD\AB | 00 | 01 | 11 | 10 |
|:-:|:-:|:-:|:-:|:-:|
| **00** | **1** | 0 | 0 | **1** |
| **01** | **1** | 0 | 0 | **1** |
| **11** | **1** | 0 | 0 | **1** |
| **10** | **_1_** | _1_ | _1_ | **_1_** |

si ha che in gruppo in grassetto è composto da $8 = 2^3$ combinazioni, mentre quello in corsivo da $4$.
Il **mintermine** del gruppo in grassetto è $E_1 = \neg B$ perchè gli altri sono _don't care_, mentre quello del gruppo in corsivo è $E_2 = C \cdot \neg D$, quindi:
$$E = \neg B + C \cdot \neg D$$
