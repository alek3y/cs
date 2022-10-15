# Relazioni

Una **relazione** in un insieme $R$, è il **predicato** che **lega** più elementi degli insiemi coinvolti.

Per esempio, considerando le seguenti **proposizioni** con il _soggetto_ e il _complemento_ in $\mathbb{N}$,

| Soggetto | Predicato | Complemento |
|:-:|:-:|:-:|
| 19 | è maggiore di | 5 |
| 3 | è uguale a | 7 |

è possibile costruire una _relazione_ che chiameremo $\geq$, tale che:
$$\geq \space = \{(x, y) | x \text{ è maggiore di } y \lor x \text{ è uguale a } y \}$$

L'insieme di appartenenza del _soggetto_ è detto **dominio**, mentre quello del _complemento_ è detto **codominio**.

Si dice **relazione binaria**, la relazione $R$ che coinvolge due oggetti appartenenti ad $A$ e a $B$, per cui
$$R \subseteq A \times B$$
di conseguenza, il sottoinsieme delle coppie $(a, b) \in A \times B$ sono messe in relazione da $R$, cosa che viene espressa come $aRb$, $R(a, b)$, $(a, b) \in R$ oppure "$a$ è in relazione con $b$ secondo $R$".

## Proprietà

Una relazione $R \subseteq A \times A = A^2$, ha le seguenti proprietà:

- **Riflessiva**:
	$$xRx, \forall x \in A$$

- **Simmetrica**:
	$$xRy \Rightarrow yRx, \forall x, y \in A$$

- **Transitiva**:
	$$xRy \land yRz \Rightarrow xRz, \forall x, y, z \in A$$

Al contrario, se **per ogni elemento coinvolto** la proprietà non vale, la relazione potrà essere:
- **Antiriflessiva** (o _irriflessiva_)
	$$(x, x) \not\in R, \forall x \in A$$

- **Antisimmetrica**
	$$(x, y) \in R \Rightarrow (y, x) \not\in R \lor ((y, x) \in R \land x = y), \forall x, y \in A$$

	che significa che se $x \neq y$, $x$ sarà in relazione con $y$, ma non viceversa.

Per ogni proprietà quindi, basta un unico elemento che non la rispetti perchè non sia soddisfatta.

## Confrontabilità

Due elementi $x, y \in A$, si dicono **confrontabili** secondo la relazione $R$ se vale:
$$xRy \lor yRx$$

altrimenti, si dicono **inconfrontabili** se:
$$\neg (xRy) \land \neg (yRx)$$
che è il contrario (legge di _De Morgan_).
