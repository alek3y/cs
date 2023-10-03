# Basi di dati

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

- nel caso `[N:1]`:
	```dot process
	digraph {
		rankdir=LR
		node [shape=record]
		edge [arrowsize=0.5 dir=both]
		A -> B [arrowtail=normalnormal]
	}
	```

- nel caso `[1:N]`, dove ognuno di `A` è associato con **zero o più** di `B`:
	```dot process
	digraph {
		rankdir=LR
		node [shape=record]
		edge [arrowsize=0.5 dir=both]
		A -> B [arrowhead=normalnormalnonetee]
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
ogni studente ha sostenuto _zero o più_ esami, mentre ogni esame è sostenuto da _uno ed un solo_ studente.

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

Se invece è **ricorsiva** vengono aggiunte delle etichette che determinano il **ruolo** dell'entità, per esempio:
```dot process
digraph {
	rankdir=LR
	node [shape=box]
	edge [arrowsize=0.5 dir=both]

	Persona:w -> Persona:e [
		label="ÈMadreDi"
		headlabel="Figlio"
		taillabel="Madre"
		arrowhead=normalnormalnonetee
		arrowtail=normal
	]
}
```
per cui _una persona può essere madre di più figli_ e _una persona è figlia di una e una sola madre_.

### Ereditarietà

L'**ereditarietà** permette di specializzare una classe in più _sottoclassi_.
```dot process
digraph {
	rankdir=TB
	splines=ortho
	node [shape=box]
	edge [dir=none]

	0 [shape=point width=0]
	Veicolo -> 0 [dir=back arrowtail=onormal]
	0 -> Aereo, Moto
}
```

Perchè l'**integrità** sia preservata però, vanno imposti **vincoli**:
- **estensionali**: le _entry_ fanno parte della superclasse, quindi le **associazioni vengono ereditate**
- **intensionali**: i tipi delle _entry_ devono essere **sottotipi** di quelli nella superclasse

Per esempio, `Pixel` fa parte di `Spazio` (_vincolo estensionale_) e `int` è sottotipo di `real` (_vincolo intensionale_):
```dot process
digraph {
	rankdir=TB
	node [shape=box fontname="Times"]
	edge [arrowsize=0.5 dir=both]

	Vettore [
		shape=plain
		label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
			<tr><td>Vettore</td></tr>
			<tr><td>- x: real<br/>- y: real</td></tr>
		</table>>
	]
	Pixel [
		shape=plain
		label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
			<tr><td>Pixel</td></tr>
			<tr><td>- x: int<br/>- y: int</td></tr>
		</table>>
	]

	{
		rank=same
		Vettore -> Spazio [label="FaParteDi" arrowhead=normalnormal arrowtail=normalnormal]
	}

	Vettore -> Pixel [arrowsize=1 arrowtail=onormal arrowhead=none]
}
```

Di conseguenza, le _entità_ vengono estese partendo da altre **aggiungendo** o **ridefinendo** attributi:
```dot process
digraph {
	rankdir=TB
	node [shape=record]
	edge [dir=back]
	Persona [label="{Persona | - Nome: string\l- Possiede: Auto\l}"]
	Studente [label="{Studente | - Matricola: int\l- Possiede: AutoUtilitaria\l}"]
	Persona -> Studente [arrowtail=onormal]
}
```

Oltre ad essere **singole**, le gerarchie possono anche essere **multiple** se è definita da più classi:
```dot process
digraph {
	rankdir=BT
	node [shape=record]

	StudenteLavoratore -> {
		rank=same
		Studente
		Dipendente
	} [arrowhead=onormal]
}
```

Inoltre, possono anche essere applicati dei **vincoli** di:
- **disgiunzione**, per cui $B \cap C = \emptyset$:
	```dot process
	digraph {
		rankdir=BT
		splines=ortho
		node [shape=record]
		edge [dir=none]

		subgraph {
			rank=same
			B
			C
		}

		0 [shape=point width=0.1]
		0 -> A [dir=forward arrowhead=onormal]
		B, C -> 0
	}
	```

- **copertura**, per cui $B \cup C = A$:
	```dot process
	digraph {
		rankdir=BT
		splines=ortho
		node [shape=record]
		edge [dir=none]

		0 [shape=point width=0]
		0 -> A [dir=forward arrowhead=onormal color="black:invis:black"]

		{
			rank=same
			B
			C
		} -> 0
	}
	```

### Esempi

#### Museo

```
Si vuole costruire una base di dati per memorizzare informazioni su un museo d’arte. Il
museo ha una collezione di oggetti d’arte. Ogni oggetto d’arte ha un numero
indetificativo, un artista (se noto), un anno (in cui è stato creato), un titolo e una
descrizione. Gli oggetti d’arte sono classificati in base al loro tipo. I tre tipi principali
sono: Dipinto, Scultura e Statua, più un altro tipo detto Altro per accogliere oggetti che
non rientrano in uno dei tre tipi principali. Un dipinto ha un tipo-pittura (olio, acquarello,
etc), un materiale su cui è stato steso (carta, tela di canapa, legno, etc.), e uno stile
(moderno, astratto etc.). Una scultura o una statua ha un materiale con cui è stata
creata (legno, pietra, etc.), un’altezza, un peso e uno stile. Un oggetto d’arte nella
categoria Altro ha un tipo (stampa, foto etc) e uno stile. Il museo memorizza
informazioni sugli artisti: nome, data di nascita, data di morte (se non è vivente), paese
di origine e lo stile principale. Nel museo sono organizzate delle esposizioni, ognuna
delle quali ha un nome, una data di inizio e una data di fine, ed è collegata a tutti gli
oggetti d’arte che sono stati esposti durante l’esposizione.
```

```dot process
digraph {
	rankdir=BT
	node [shape=box fontname="Times"]
	edge [arrowsize=0.5 dir=both]

	{
		rank=same

		Oggetto [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Oggetto</td></tr>
				<tr><td>- Id: int<br align="left"/>- Titolo: string<br align="left"/>- Descrizione: string<br align="left"/>- Anno: int<br align="left"/></td></tr>
			</table>>
		]

		Artista [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Artista</td></tr>
				<tr><td>- Nome: string<br align="left"/>- Origine: string<br align="left"/>- DataNascita: date<br align="left"/></td></tr>
			</table>>
		]

		Esposizione [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Esposizione</td></tr>
				<tr><td>- Nome: string<br align="left"/>- DataInizio: date<br align="left"/>- DataFine: date<br align="left"/></td></tr>
			</table>>
		]
	}

	Deceduto [
		shape=plain
		label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
			<tr><td>Deceduto</td></tr>
			<tr><td>- DataMorte: date</td></tr>
		</table>>
	]

	{
		rank=same

		Dipinto [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Dipinto</td></tr>
				<tr><td>- TipoPittura: string<br align="left"/>- Materiale: string<br align="left"/></td></tr>
			</table>>
		]

		Scultura [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Scultura</td></tr>
				<tr><td>- Altezza: int<br align="left"/>- Peso: int<br align="left"/>- Materiale: string<br align="left"/></td></tr>
			</table>>
		]

		Altro [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Altro</td></tr>
				<tr><td>- Tipo: string</td></tr>
			</table>>
		]
	}

	{
		rank=same
		0, 1, 2 [shape=point width=0]
		1 [width=0.08]
		0 -> 1 -> 2 [dir=none]
	}

	Esposizione -> Oggetto [label="Riguardante" arrowhead=normalnormal arrowtail=normalnormalnonetee]
	Oggetto -> Artista [label="FattoDa" arrowhead=normalnonetee arrowtail=normalnormal]

	1 -> Oggetto [arrowsize=1 dir=forward arrowhead=onormal color="black:black" weight=100]
	Dipinto -> 0 [dir=none weight=100]
	Scultura -> 1 [dir=none weight=100]
	Altro -> 2 [dir=none weight=100]

	Deceduto -> Artista [arrowsize=1 dir=forward arrowhead=onormal weight=100]
	Statua -> Scultura [arrowsize=1 dir=forward arrowhead=onormal]

	Stile -> Oggetto [label="Con" arrowtail=normalnormalnonetee]
	Stile -> Artista [label="Preferisce" arrowtail=normalnormalnonetee]
}
```
