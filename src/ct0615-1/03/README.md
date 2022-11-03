# MIPS

L'architettura **MIPS** possiede **32 registri** e definisce una **word** come **32 bit**.

Un esempio, può essere:
```c
a[8] = h + a[8]
```
che tradotto sarà:
```asm
lw \$15, 32(\$4)
add \$15, \$5, \$15
sw \$15, 32(\$4)
```
dove:
- `\$1` indica il registro `1`
- `32(\$4)` indica un indirizzo dentro il registro `\$4` in offset a `32`, quindi `32 + (\$4)` (`0(\$4)` o `(\$4)` rappresenta il contenuto del registro)
- `lw` e `sw` fanno il _load_ e lo _store_ in memoria _RAM_

## Formati delle istruzioni

- **R-Type**, per le istruzioni aritmetiche come `and` e `add`:

	| opcode | rs | rt | rd | shamt | funct |
	|:-:|:-:|:-:|:-:|:-:|:-:|
	| 6 bit | 5 bit | 5 bit | 5 bit | 5 bit | 6 bit |

	dove l'**opcode** e **funct** rappresentano l'istruzione, mentre **rs** è il primo registro, **rt** il secondo e **rd** è il registro di destinazione.

	Per esempio, la funzione `add` è strutturata in:
	```asm
	add rd, rs, rt
	```

- **I-Type**, per le istruzioni per la memoria `lw` e `sw`:

	| opcode | rs | rt | offset |
	|:-:|:-:|:-:|:-:|
	| 6 bit | 5 bit | 5 bit | 16 bit |

	Per esempio,
	```asm
	lw rt, offset(rs)
	```

	Esistono anche delle operazioni aritmetiche immediate per poter usare valori espliciti, per esempio:
	```asm
	addi rt, rs, 5
	```
	che fa la somma `rt = rs + 5`.

- **J-Type**, per le istruzioni per i salti condizionati come `beq`, `bneq`, `j`:

	| opcode | address |
	|:-:|:-:
	| 6 bit | 26 bit |
