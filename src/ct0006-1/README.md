# Basi di Dati

## Progettazione concettuale

### Proprietà

I tipi **primitivi** di dati sono:
- `int`
- `real`
- `bool`
- `date`
- `string`

I tipi **non primitivi**, invece, sono:
- `[A: T, B: S, ...]` (i.e. record), dove `A` e `B` sono etichette mentre `T` ed `S` sono tipi
- `(A; B; ...)` (i.e. enumerazione; e.g. `(M; F)`), dove `A` e `B` sono etichette
- `seq T` (i.e. sequenza; e.g. `seq int`), dove `T` è un tipo

Tra le limitazioni applicate ai dati, ci sono i **vincoli di integrità**:
- **statici**: il dominio del dato è limitato durante l'**inserimento**
- **dinamici**: il cambiamento è limitato durante la **modifica**

### Cardinalità

La **cardinalità** di un'associazione fra `X` e `Y` descrive la **molteplicità** di `X -> Y` e di `Y -> X`:

| | `[1:1]` | `[1:N]` | `[N:1]` | `[N:N]` |
|:-:|:-:|:-:|:-:|:-:|
| Ognuno di `X` con al massimo | uno di `Y` | molti di `Y` | uno di `Y` | molti di `Y` |
| Ognuno di `Y` con al massimo | uno di `X` | uno di `X` | molti di `X` | molti di `X` |

Nello schema, le **associazioni** sono rappresentate come,
- nel caso `[1:1]`:
	```dot process
	digraph {
		rankdir=LR
		node [shape=record]
		edge [arrowsize=0.5 dir=both]
		A -> B
	}
	```

- nel caso `[1:N]`:
	```dot process
	digraph {
		rankdir=LR
		node [shape=record]
		edge [arrowsize=0.5 dir=both]
		A -> B [arrowhead=normalnormal]
	}
	```

- nel caso `[N:1]`, dove ognuno di `A` è associato con **al minimo** uno di `B`:
	```dot process
	digraph {
		rankdir=LR
		node [shape=record]
		edge [arrowsize=0.5 dir=both]
		A -> B [arrowtail=normalnormalnonetee]
	}
	```

Per esempio, nel caso di:
```dot process
digraph {
	rankdir=LR
	node [shape=record]
	edge [arrowsize=0.5 dir=both]
	Studente -> Esame [label="HaSostenuto" arrowhead=normalnormalnonetee]
}
```
ogni studente può aver sostenuto più esami, mentre ogni esame è sostenuto da uno ed un solo studente.

### Associazioni

Nel caso in cui l'**associazione contenga proprietà**, come
```dot process
digraph {
	rankdir=LR
	node [shape=record]
	edge [arrowsize=0.5 dir=both fontname="Times"]
	Utente -> Libro [
		label=<<table border="0" cellborder="1" cellspacing="0">
			<tr><td>HaInPrestito</td></tr>
			<tr><td align="left">- Data: date</td></tr>
			<tr><td border="0"></td></tr>
		</table>>
		arrowhead=normalnormalnonetee
		arrowtail=normalnonetee
	]
}
```
la relazione viene trasformata in un'ulteriore entità:
```dot process
digraph {
	rankdir=LR
	node [shape=record]
	edge [arrowsize=0.5 dir=both]
	Prestito [label="Prestito | - Data: date\l"]
	Utente -> Prestito [label="HaPreso" arrowhead=normalnormalnonetee]
	Prestito -> Libro [label="Riguarda" arrowtail=normalnonetee]
}
```

### Ereditarietà

Con l'**ereditarietà** è possibile estendere le _entità_ partendo da altre **aggiungendo** o **ridefinendo** attributi:
```dot process
digraph {
	rankdir=TB
	node [shape=record]
	edge [arrowsize=0.5 dir=both]
	Persona [label="{Persona | - Nome: string\l- AnnoNascita: int\l- LingueParlate: seq string\l- Possiede: Auto\l}"]
	Studente [label="{Studente | - Matricola: int\l- AnnoIscrizione: int\l- Possiede: AutoUtilitaria\l}"]
	Persona -> Studente [arrowhead=none arrowtail=onormal]
}
```
