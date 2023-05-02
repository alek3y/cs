# Assembly

La sintassi si può riassumere principalmente in:

| Istruzione | Descrizione |
|-:|:-|
| `a: .word 5` | Alloca 4 byte contenenti 5 e li salva all'indirizzo `a` |
| `b: .dword 1,2,3,4` | Alloca 32 byte contenenti 1, 2, 3 e 4 |
| `c: .asciiz "hello\n"` | Alloca la stringa con il null byte alla fine |
| `d: .space 10` | Alloca 10 byte non inizializzati |
| `mov x0, #5` | Sposta il numero 5 dentro `x0` |
| `movz x1, #15, LSL #4` | Mette `0xf0` in `x1` azzerando gli altri bit |
| `movk x1, #5` | Modifica `x1` a `0xf5` |
| `cmp x0, #5` | Confronta `x0` e imposta i flag |
| `b label` | Effettua un salto incondizionato a `label` |
| `b.eq`, `b.ne`, `b.gt`, `b.lt label` | Salta all'indirizzo se i flag soddisfano la condizione |
| `bl label` | Salva il _program counter_ e salta alla funzione `label` |
| `ret` | Esce dalla funzione ripristinando il _program counter_ |
| `str w0, [sp]` | Salva `w0` sui primi 4 byte all'indirizzo `sp` |
| `str x0, [sp, #-8]!` | Salva 8 byte su `sp += 8` (_pre-indexed_, e.g. `++sp`) |
| `ldr x0, [sp, #8]` | Carica da `sp+8` (_offset_, quindi `sp` non cambia) su `x0` |
| `ldr x0, [sp], #8` | Carica da `sp` (_post indexed_, e.g. `sp++`) e aggiorna `sp = sp+8` |

## Costrutti

Il costrutto `if` può essere riscritto come:
```arm
b.<!cond> else
// ...
b exit
else:
// ...
exit:
```
dove `<!cond>` è il la condizione `<cond>` negata.

Per il caso del `do ... while`, invece:
```arm
loop:
// ...
b.<cond> loop
```
mentre per il `while`, e di conseguenza il `for`:
```arm
b.<!cond> exit
loop:
// ...
b.<cond> loop
exit:
```

## PCS

Ogni funzione deve seguire le **Procedure Call Standard**, per poter _linkare_ file _oggetto_ di diversi linguaggi:
1. Gli argomenti sono passati da `x0` a `x7`, mentre gli altri vengono salvati sullo _stack_
2. Il valore di ritorno è salvato dentro `x0`
3. L'indirizzo di ritorno, dentro `x30` (o anche `lr`), va ristabilito dopo aver usato `bl`
4. I registri da `x0` a `x15` possono essere modificati dalla funzione
5. I registri da `x19` a `x27` devono, invece, essere ripristinati prima di `ret`
6. I registri `x8`, `x16`, `x17` e `x18` vanno evitati perchè sono riservati
7. Lo _stack pointer_, su `x28` (o `sp`), deve essere ripristinato prima di `ret`
8. L'accesso a `sp` (e.g. `ldr x0, [sp]`) va allineato a 16 byte (i.e. va incrementato di 16 in 16)

## Esempio

Sfruttando le _PCS_ e le [system call](https://chromium.googlesource.com/chromiumos/docs/+/master/constants/syscalls.md#arm64-64_bit) si ha come esempio:
```arm
.data
message: .asciz "hello world\n"

.text

// Allinea il code segment ad un multiplo di 16 per aumentare le istruzioni
// caricate nella cache line (larga 16 byte), aumentando i cache hit
.p2align 4

.global _start
_start:
	bl find_message	// Usa il ritorno come argomento (PCS #1)
	bl print_str

	mov x0, #0	// Imposta l'argomento (PCS #1)
	bl exit

// Argomenti:
// - (void)
find_message:
	adr x0, message	// Restituisce x0 (PCS #2)
	ret

// Argomenti:
// - (int) x0
exit:
	mov x8, #93
	svc #0
	ret

// Argomenti:
// - (char) w0
print_char:
	str w0, [sp, #-16]!	// Allineato a 16 byte (PCS #8)

	mov x0, #1
	mov x1, sp	// Usa lo stack come indirizzo per la stringa
	mov x2, #1
	mov x8, #64
	svc #0	// System call per write(2)

	add sp, sp, #16	// Ripristina lo stack (PCS #7)
	ret

// Argomenti:
// - (char *) x0
.global print_str	// Rende la funzione visibile a *.o linkando
print_str:
	ldrb w1, [x0]	// Rimpiazza x1 (PCS #4)
	cmp w1, #0
	b.eq str_empty

	str lr, [sp, #-16]!	// Salva lr prima del linking (PCS #3)

	str x0, [sp, #-16]!
	ldrb w0, [x0]	// Carica un byte dall'indirizzo x0
	bl print_char
	ldr x0, [sp], #16	// Ripristina x0 assumendo modifiche (PCS #4)

	add x0, x0, #1
	bl print_str	// Step della ricorsione

	ldr lr, [sp], #16	// Ripristina lr e poi sp (PCS #3, #7)
	ret

	str_empty:
		ret
```
