# Programmazione dinamica

La **programmazione dinamica** è una tecnica di **progettazione** di algoritmi che:
- Sono **riducibili** a molteplici sottoproblemi annidati
- Possiedono sottoproblemi detti **sovrapponibili**, ovvero che sono riutilizzati tra i vari sottoproblemi

La tecnica si basa quindi sul **salvare** il risultato dei sottoproblemi così da avere la soluzione a disposizione.

Per esempio, dall'albero ricorsivo della funzione di Fibonacci per $f(5)$
```dot process
digraph {
	node [shape=circle]
	edge [dir=none]

	5 [color="#6a4c93"]
	4 [color="#ff595e"]

	{
		node [color="#8ac926"]
		31 [label="3"]
		32 [label="3"]
	}

	{
		node [color="#1982c4"]
		21 [label="2"]
		22 [label="2"]
		23 [label="2"]
	}

	{
		node [shape=none width=0 height=0]
		11 [label="1"]
		12 [label="1"]
		13 [label="1"]
		14 [label="1"]
		15 [label="1"]
		01 [label="0"]
		02 [label="0"]
		03 [label="0"]
	}

	5 -> 4, 31
	4 -> 32, 21
	31 -> 22, 11
	32 -> 23, 12
	21 -> 13, 01
	22 -> 14, 02
	23 -> 15, 03

	{
		node [shape=point width=0]
		_0
		_1
		_2
		_3
		_4
		_5
	}

	{
		edge [style=invis]
		{
			rank=same
			4 -> _0 -> 31
		}
		{
			rank=same
			32 -> _1 -> 21
			21 -> 22
			22 -> _2 -> 11
		}
		{
			rank=same
			23 -> _3 -> 12
			12 -> 13
			13 -> _4 -> 01
			01 -> 14
			14 -> _5 -> 02
		}
		{
			rank=same
			15 -> 03
		}
		{
			edge [weight=100]
			5 -> _0
			4 -> _1
			31 -> _2
			32 -> _3
			21 -> _4
			22 -> _5
		}
	}
}
```
si nota che conviene _salvare_ il risultato di $f(2)$ e $f(3)$ per calcolarlo **una sola volta**.

Durante l'implementazione si può scegliere tra due **tecniche di costruzione** di algoritmi:
- **Top-down**, attraverso la **memoization**: salva le soluzioni in una tabella durante la ricorsione
- **Bottom-up**: ordina i sottoproblemi in base alla dimensione e li risolve dal più piccolo, salvandoli

Quando la soluzione non dipende da **tutti i sottoproblemi** conviene usare la **top-down** perchè evita di calcolarli tutti ed è più intuitiva, altrimenti conviene la **bottom-up** perchè **evita il carico** della ricorsione.
