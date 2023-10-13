# Progettazione concettuale

## Proprietà

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

## Cardinalità

La **cardinalità** di un'associazione fra `X` e `Y` descrive la **molteplicità** di `X -> Y` e di `Y -> X`:

| | `[1:1]` | `[1:N]` | `[N:1]` | `[N:N]` |
|:-:|:-:|:-:|:-:|:-:|
| Ognuno di `X` con al massimo | uno di `Y` | molti di `Y` | uno di `Y` | molti di `Y` |
| Ognuno di `Y` con al massimo | uno di `X` | uno di `X` | molti di `X` | molti di `X` |

Nello schema, le **associazioni** sono rappresentate come,
- nel caso `[1:1]` **totale**:
	```dot process
	digraph {
		rankdir=LR
		node [shape=record]
		edge [arrowsize=0.5 dir=both]
		A -> B
	}
	```

- nel caso `[N:1]` **totale**:
	```dot process
	digraph {
		rankdir=LR
		node [shape=record]
		edge [arrowsize=0.5 dir=both]
		A -> B [arrowtail=normalnormal]
	}
	```

- nel caso `[1:N]` **parziale**, dove ognuno di `A` è associato con **zero o più** di `B`:
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
	Studenti -> Esami [label="HaSostenuto" arrowhead=normalnormalnonetee]
}
```
ogni studente ha sostenuto _zero o più_ esami, mentre ogni esame è sostenuto da _uno ed un solo_ studente.

## Associazioni

Nel caso in cui l'**associazione contenga proprietà**, come
```dot process
digraph {
	rankdir=LR
	node [shape=record]
	edge [arrowsize=0.5 dir=both]
	Utenti -> Libri [
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
	Prestiti [label="Prestiti | - Data: date\l"]
	Utenti -> Prestiti [label="HaPreso" arrowhead=normalnormalnonetee]
	Prestiti -> Libri [label="Riguarda" arrowtail=normalnonetee]
}
```

Se invece è **ricorsiva** vengono aggiunte delle etichette che determinano il **ruolo** dell'entità, per esempio:
```dot process
digraph {
	rankdir=LR
	node [shape=box]
	edge [arrowsize=0.5 dir=both]

	Persone:w -> Persone:e [
		label="ÈMadreDi"
		headlabel="Figlio"
		taillabel="Madre"
		arrowhead=normalnormalnonetee
		arrowtail=normal
	]
}
```
per cui _una persona può essere madre di più figli_ e _una persona è figlia di una e una sola madre_.

## Ereditarietà

L'**ereditarietà** permette di specializzare una classe in più _sottoclassi_.
```dot process
digraph {
	rankdir=TB
	splines=ortho
	node [shape=box]
	edge [dir=none]

	0 [shape=point width=0]
	Veicoli -> 0 [dir=back arrowtail=onormal]
	0 -> Aerei, Moto
}
```

Perchè l'**integrità** sia preservata però, vanno imposti **vincoli**:
- **estensionali**: le _entry_ fanno parte della superclasse, quindi le **associazioni vengono ereditate**
- **intensionali**: i tipi delle _entry_ devono essere **sottotipi** di quelli nella superclasse

Per esempio, `Pixel` fa parte di `Spazio` (_vincolo estensionale_) e `int` è sottotipo di `real` (_vincolo intensionale_):
```dot process
digraph {
	rankdir=TB
	node [shape=box]
	edge [arrowsize=0.5 dir=both]

	Vettori [
		shape=plain
		label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
			<tr><td>Vettori</td></tr>
			<tr><td align="left">- x: real<br/>- y: real</td></tr>
		</table>>
	]
	Pixel [
		shape=plain
		label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
			<tr><td>Pixel</td></tr>
			<tr><td align="left">- x: int<br/>- y: int</td></tr>
		</table>>
	]

	{
		rank=same
		Vettori -> Spazio [label="FaParteDi" arrowhead=normalnormal arrowtail=normalnormal]
	}

	Vettori -> Pixel [arrowsize=1 arrowtail=onormal arrowhead=none]
}
```

Di conseguenza, le _entità_ vengono estese partendo da altre **aggiungendo** o **ridefinendo** attributi:
```dot process
digraph {
	rankdir=TB
	node [shape=record]
	edge [dir=back]
	Persone [label="{Persone | - Nome: string\l- Possiede: Auto\l}"]
	Studenti [label="{Studenti | - Matricola: int\l- Possiede: AutoUtilitaria\l}"]
	Persone -> Studenti [arrowtail=onormal]
}
```

Oltre ad essere **singole**, le gerarchie possono anche essere **multiple** se è definita da più classi:
```dot process
digraph {
	rankdir=BT
	node [shape=record]

	StudentiLavoratori -> {
		rank=same
		Studenti
		Dipendenti
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

		0 [shape=point width=0.08]
		0 -> A [dir=forward arrowhead=onormal]
		B, C -> 0
	}
	```

- **copertura**, per cui $B \cup C = A$:
	```dot process
	digraph {
		rankdir=TB
		node [shape=record]
		edge [dir=none]

		{
			rank=same
			0, 1, 2 [shape=point width=0]
			1 [width=0.08]
			0 -> 1 -> 2
		}

		A -> 1 [dir=back arrowtail=onormal color="black:black"]

		{
			rank=same
			B
			C
		}

		0 -> B [weight=100]
		2 -> C [weight=100]
	}
	```

## Esempi

### Museo

```
Si vuole costruire una base di dati per memorizzare informazioni su un museo d’arte. Il museo
ha una collezione di oggetti d’arte. Ogni oggetto d’arte ha un numero identificativo, un
artista (se noto), un anno (in cui è stato creato), un titolo e una descrizione. Gli oggetti
d’arte sono classificati in base al loro tipo. I tre tipi principali sono: Dipinto, Scultura e
Statua, più un altro tipo detto Altro per accogliere oggetti che non rientrano in uno dei tre
tipi principali. Un dipinto ha un tipo-pittura (olio, acquarello, etc), un materiale su cui è
stato steso (carta, tela di canapa, legno, etc.), e uno stile (moderno, astratto etc.). Una
scultura o una statua ha un materiale con cui è stata creata (legno, pietra, etc.),
un’altezza, un peso e uno stile. Un oggetto d’arte nella categoria Altro ha un tipo (stampa,
foto etc) e uno stile. Il museo memorizza informazioni sugli artisti: nome, data di nascita,
data di morte (se non è vivente), paese di origine e lo stile principale. Nel museo sono
organizzate delle esposizioni, ognuna delle quali ha un nome, una data di inizio e una data di
fine, ed è collegata a tutti gli oggetti d’arte che sono stati esposti durante l’esposizione.
```

```dot process
digraph {
	rankdir=BT
	node [shape=box]
	edge [arrowsize=0.5 dir=both]

	{
		rank=same

		Oggetti [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Oggetti</td></tr>
				<tr><td align="left">- Id: int<br align="left"/>- Titolo: string<br align="left"/>- Descrizione: string<br align="left"/>- Anno: int<br align="left"/></td></tr>
			</table>>
		]

		Artisti [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Artisti</td></tr>
				<tr><td align="left">- Nome: string<br align="left"/>- Origine: string<br align="left"/>- DataNascita: date<br align="left"/></td></tr>
			</table>>
		]

		Esposizioni [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Esposizioni</td></tr>
				<tr><td align="left">- Nome: string<br align="left"/>- DataInizio: date<br align="left"/>- DataFine: date<br align="left"/></td></tr>
			</table>>
		]
	}

	Deceduti [
		shape=plain
		label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
			<tr><td>Deceduti</td></tr>
			<tr><td align="left">- DataMorte: date</td></tr>
		</table>>
	]

	{
		rank=same

		Dipinti [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Dipinti</td></tr>
				<tr><td align="left">- TipoPittura: string<br align="left"/>- Materiale: string<br align="left"/></td></tr>
			</table>>
		]

		Sculture [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Sculture</td></tr>
				<tr><td align="left">- Altezza: int<br align="left"/>- Peso: int<br align="left"/>- Materiale: string<br align="left"/></td></tr>
			</table>>
		]

		Altro [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Altro</td></tr>
				<tr><td align="left">- Tipo: string</td></tr>
			</table>>
		]
	}

	{
		rank=same
		0, 1, 2 [shape=point width=0]
		1 [width=0.08]
		0 -> 1 -> 2 [dir=none]
	}

	Esposizioni -> Oggetti [label="Riguardante" arrowhead=normalnormal arrowtail=normalnormalnonetee]
	Oggetti -> Artisti [label="FattoDa" arrowhead=normalnonetee arrowtail=normalnormal]

	1 -> Oggetti [arrowsize=1 dir=forward arrowhead=onormal color="black:black" weight=100]
	Dipinti -> 0 [dir=none weight=100]
	Sculture -> 1 [dir=none weight=100]
	Altro -> 2 [dir=none weight=100]

	Deceduti -> Artisti [arrowsize=1 dir=forward arrowhead=onormal weight=100]
	Statue -> Sculture [arrowsize=1 dir=forward arrowhead=onormal]

	Stili -> Oggetti [label="Con" arrowtail=normalnormalnonetee]
	Stili -> Artisti [label="Preferisce" arrowtail=normalnormalnonetee]
}
```

### Scacchi

```
Un'associazione scacchistica vuole costruire una base di dati per la propria gestione.
Nell'associazione ci sono soci ordinari e soci onorari. Un socio ha nome, cognome, anno di
nascita, telefono, indirizzo. Se un socio è ordinario vogliamo sapere anche se ha pagato la
quota associativa per l'anno in corso oppure no. L'associazione organizza tornei a cui
possono partecipare anche degli ospiti (cioè delle persone non dell'associazione: di questi
interessa nome, cognome, anno di nascita, nazionalità). In un torneo si svolgono delle
partite. Le partite hanno due giocatori che sono soci o ospiti e un arbitro, che è sempre un
socio, ha una data e un vincitore (oppure è una patta e non c'è vincitore). Un torneo ha
anche una data d'inizio e una di fine e il vincitore del torneo.
```

```dot process
digraph {
	rankdir=TB
	edge [arrowsize=0.5 dir=both]
	node [shape=box]

	{
		rank=same

		Giocatori [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Giocatori</td></tr>
				<tr><td align="left">- Nome: string<br align="left"/>- Cognome: string<br align="left"/>- AnnoNascita: int<br align="left"/></td></tr>
			</table>>
		]

		Partite [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Partite</td></tr>
				<tr><td align="left">- Data: date</td></tr>
				<tr><td>Vincitore &lt;&gt; NULL<br/>=&gt;<br/>Vincitore = Primo ||<br/>Vincitore = Secondo</td></tr>
			</table>>
		]

		Tornei [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Tornei</td></tr>
				<tr><td align="left">- DataInizio: date<br align="left"/>- DataFine: date<br align="left"/></td></tr>
			</table>>
		]
	}

	{
		rank=same
		0, 1, 2 [shape=point width=0]
		1 [width=0.08]
		0 -> 1 -> 2 [dir=none]
	}

	{
		rank=same

		Ospiti [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Ospiti</td></tr>
				<tr><td align="left">- Nazionalità: string</td></tr>
			</table>>
		]

		Soci [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Soci</td></tr>
				<tr><td align="left">- Telefono: string<br align="left"/>- Indirizzo: string<br align="left"/></td></tr>
			</table>>
		]
	}

	Giocatori -> 1 [arrowsize=1 dir=back arrowtail=onormal color="black:black" weight=100]
	2 -> Soci [dir=none weight=100]
	0 -> Ospiti [dir=none weight=100]

	Giocatori -> 0 [style=invis]
	Giocatori -> 2 [style=invis weight=2]
	Soci -> Partite [label="Arbitra" arrowhead=normalnormalnonetee]

	{
		rank=same
		SociOnorari
		SociOrdinari
	}

	{
		rank=same
		3, 4, 5 [shape=point width=0]
		4 [width=0.08]
		3 -> 4 -> 5 [dir=none]
	}

	3 -> SociOnorari [dir=none weight=100]
	5 -> SociOrdinari [dir=none weight=100]
	Soci -> 4 [arrowsize=1 dir=back arrowtail=onormal color="black:black"]

	Giocatori -> Partite [label="   Primo " arrowhead=normalnormalnonetee]
	Giocatori -> Partite [label=" Secondo " arrowhead=normalnormalnonetee]
	Giocatori -> Partite [label="Vincitore" arrowhead=normalnormalnonetee arrowtail=normalnonetee]
	Giocatori -> Tornei [xlabel="Vince" arrowhead=normalnormalnonetee arrowtail=normalnonetee]
	Partite -> Tornei [label="In" arrowtail=normalnormal]
}
```

L'associazione `Vince`, da `Tornei` a `Giocatori` può essere zero nel caso in cui il torneo sia ancora in corso.

### Aeroporto privato

```
Si vuole realizzare una base di dati per gestire le informazioni relative a un piccolo
aeroporto privato. Ogni aeroplano ha un numero di matricola, che lo identifica, è di un certo
tipo ed è posto in un hangar specifico. Ogni tipo di aeroplano ha un numero di modello, una
capacità e un peso. Ogni hangar ha un numero, una capienza e un’ubicazione. Gli aerei hanno un
unico proprietario che può essere una persona fisica o un’azienda. Per un’azienda si è
interessati alla ragione sociale, all’indirizzo e all’email. Per le persone si vuole
memorizzare nome, cognome, l’indirizzo e più numeri di telefono. I proprietari possono
possedere più aerei e per ciascuno si vuole memorizzare la data di acquisto. Si vuole tenere
traccia delle manutenzioni effettuate agli aerei. Ogni aereo è sottoposto molte volte al
servizio di manutenzione e per ciascun intervento si vuole memorizzare la data di manutenzione,
il tipo di lavoro effettuato e i meccanici che hanno svolto l’intervento e le ore che vi hanno
dedicato (possono essere diverse per i vari meccanici). Per i meccanici si vuole memorizzare
il nome, cognome e lo stipendio e i tipi di aeroplani su cui possono lavorare.
```

```dot process
digraph {
	rankdir=TB
	edge [arrowsize=0.5 dir=both]
	node [shape=box]

	{
		rank=same

		Hangar [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Hangar</td></tr>
				<tr><td align="left">- Numero: int<br align="left"/>- Capienza: int<br align="left"/>- Ubicazione: string<br align="left"/></td></tr>
			</table>>
		]

		Tipi [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Tipi</td></tr>
				<tr><td align="left">- NumeroModello: int<br align="left"/>- Capacità: int<br align="left"/>- Peso: int<br align="left"/></td></tr>
			</table>>
		]
	}

	{
		rank=same

		Proprietari [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Proprietari</td></tr>
				<tr><td align="left">- Indirizzo: string</td></tr>
			</table>>
		]

		Aerei [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Aerei</td></tr>
				<tr><td align="left">- Matricola: int &lt;PK&gt;</td></tr>
			</table>>
		]
	}

	{
		rank=same
		Manutenzioni [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Manutenzioni</td></tr>
				<tr><td align="left">- Data: date<br align="left"/>- Lavoro: string<br align="left"/></td></tr>
			</table>>
		]

		Meccanici [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Meccanici</td></tr>
				<tr><td align="left">- Nome: string<br align="left"/>- Cognome: string<br align="left"/>- Stipendio: int<br align="left"/></td></tr>
			</table>>
		]
	}

	Hangar -> Aerei [label="ContenutoIn", arrowhead=normalnormalnonetee]
	Tipi -> Aerei [label="Di" arrowhead=normalnormalnonetee]

	Aerei -> Manutenzioni [label="Riguarda" arrowhead=normalnormalnonetee weight=100]
	Meccanici -> Manutenzioni [
		label=<<table border="0" cellborder="1" cellspacing="0">
			<tr><td>IntervieneIn</td></tr>
			<tr><td align="left">- Ore: int</td></tr>
			<tr><td border="0"></td></tr>
		</table>>
		arrowhead=normalnormalnonetee
		arrowtail=normalnormal
	]
	Meccanici -> Tipi [label="Specializzato" arrowhead=normalnormal arrowtail=normalnormalnonetee]

	{
		rank=same
		0, 1, 2 [shape=point width=0]
		1 [width=0.08]
		0 -> 1 -> 2 [dir=none]
	}

	{
		rank=same

		Persone [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Persone</td></tr>
				<tr><td align="left">- Nome: string<br align="left"/>- Cognome: string<br align="left"/>- Telefoni: seq string<br align="left"/></td></tr>
			</table>>
		]

		Aziende [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Aziende</td></tr>
				<tr><td align="left">- RagioneSociale: string<br align="left"/>- Email: string<br align="left"/></td></tr>
			</table>>
		]
	}

	Aerei -> Proprietari [
		label=<<table border="0" cellborder="1" cellspacing="0">
			<tr><td>Possiede</td></tr>
			<tr><td align="left">- Data: date</td></tr>
			<tr><td border="0"></td></tr>
		</table>>
		arrowtail=normalnormal
	]

	Proprietari -> 1 [arrowsize=1 dir=back arrowtail=onormal color="black:black" weight=100]
	0 -> Persone [dir=none weight=100]
	2 -> Aziende [dir=none weight=100]
}
```

Dove `Di`, da `Tipi` ad `Aerei` può essere zero se un meccanico è specializzato in tipi di aerei non presenti.

### Scuola di canto

```
Si vuole costruire una base di dati per la gestione di una scuola di canto. La scuola di canto
dà la possibilità di certificare i livelli di preparazione per ciascun studente. Ogni livello
è identificato da un codice e possiede una descrizione che specifica gli obiettivi didattici
che devono essere raggiunti. Per ogni livello ci sono dei brani musicali che lo studente dovrà
interpretare per poter ricevere il certificato che attesta di aver raggiunto un certo livello
di preparazione. Per ogni studente della scuola, identificato dal codice fiscale, è necessario
sapere quando è stato superato un certo livello di preparazione. Ogni studente ha dei recapiti
telefonici per contattarlo e un proprio repertorio di brani, che non necessariamente devono
coincidere con quelli dei livelli di preparazione. Ogni brano deve avere il titolo, la durata
e l’autore che l’ha composto. Gli studenti della scuola si possono esibire in alcuni locali,
di cui si vuole memorizzare il nome e l’indirizzo e le serate organizzate. Per ogni studente
si vuole registrare la serata in cui si è esibito e le canzoni che ha cantato. Per ogni serata
si vuole sapere il numero di consumazioni vendute per poter permettere al locale di richiamare
i cantanti che attirano più pubblico.
```

```dot process
digraph {
	rankdir=TB
	edge [arrowsize=0.5 dir=both]
	node [shape=box]

	{
		rank=same

		Studenti [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Studenti</td></tr>
				<tr><td align="left">- CodFiscale: string &lt;PK&gt;<br align="left"/>- Telefoni: seq string<br align="left"/></td></tr>
			</table>>
		]

		Livelli [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Livelli</td></tr>
				<tr><td align="left">- Codice: string<br align="left"/>- Descrizione: string<br align="left"/></td></tr>
			</table>>
		]
	}

	{
		rank=same

		Serate [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Serate</td></tr>
				<tr><td align="left">- Consumazioni: int<br align="left"/></td></tr>
			</table>>
		]

		Locali [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Locali</td></tr>
				<tr><td align="left">- Nome: string<br align="left"/>- Indirizzo: string<br align="left"/></td></tr>
			</table>>
		]
	}

	{
		rank=same

		Esibizioni

		Brani [
			shape=plain
			label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="5">
				<tr><td>Brani</td></tr>
				<tr><td align="left">- Titolo: string<br align="left"/>- Data: date<br align="left"/>- Autore: string<br align="left"/></td></tr>
			</table>>
		]
	}

	0 [shape=point width=0]
	0 -> Studenti [style=invis constraint=false]

	Studenti -> Livelli [
		label=<<table border="0" cellborder="1" cellspacing="0">
			<tr><td>Supera</td></tr>
			<tr><td align="left">- Data: date</td></tr>
			<tr><td border="0"></td></tr>
		</table>>
		arrowhead=normalnormalnonetee
		arrowtail=normalnormalnonetee
	]

	Livelli -> Brani [label="Contenente" arrowhead=normalnormal arrowtail=normalnonetee]
	Studenti -> Brani [label=" Conosce" arrowhead=normalnormalnonetee arrowtail=normalnormalnonetee]

	Studenti -> Esibizioni [label=" CantaIn" arrowhead=normalnormalnonetee weight=100]

	Esibizioni -> Brani [label="Espone" arrowhead=normalnormal arrowtail=normalnormalnonetee]
	Esibizioni -> Serate [label=" Riguarda" arrowtail=normalnormal]

	Serate -> Locali [label="SiTrovaIn" arrowtail=normalnormal]

	Studenti -> Serate [label="CantaIn" arrowhead=normalnormalnonetee color="orange" fontcolor="orange"]
	Brani -> Serate [label="Espone" arrowhead=normalnormalnonetee arrowtail=normalnormal color="orange" fontcolor="orange"]
}
```

Dove le frecce _arancioni_ indicano lo schema senza `Esibizioni`, in cui le serate riguardano _un solo_ studente.
