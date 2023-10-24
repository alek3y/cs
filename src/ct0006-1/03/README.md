# Algebra relazionale

Le **interrogazioni** al _database_ possono essere formalmente rappresentate dall'**algebra relazionale**.

Ogni **relazione** `R(A1: T1, ..., An: Tn)` è considerata un **insieme di ennuple** $\{(A_1\colon T_1, ..., A_n: T_n)\}$, dove il **grado** è definito come il numero di attributi e la **cardinalità** il numero di _ennuple_.

Tra gli **operatori** principali, si hanno:
- **Ridenominazione**, e.g. per rinominare l'attributo $A_1$ a $B_1$:

	$$\rho_{A_1 \leftarrow B_1}(R)$$
	che avrà _cardinalità_ e _grado_ uguali ad $R$.

	Diventa utile nelle operazioni tra relazioni con attributi con lo **stesso nome**.

- **Unione** e **differenza**, sulle _ennuple_ di due relazioni $R$ ed $S$ dello **stesso tipo**:

	$$R \cup S$$
	con _cardinalità_ $|R| + |S| - |R \cap S|$, e
	$$R \setminus S$$
	con _cardinalità_ $|R| - |R \cap S|$.

- **Proiezione**, e.g. per **isolare** gli attributi $A_1$ e $A_2$:

	$$\pi_{A_1, A_2}(R) = \{(t.A_1, t.A_2) \mid t \in R\}$$
	di _cardinalità_ **al più** $|R|$ perchè le _ennuple_ non sono distinte se gli attributi scelti **non sono** _superchiave_.

- **Restrizione**, per **filtrare** le _ennuple_ con una **condizione** $\phi$:

	$$\sigma_\phi(R) = \{t | t \in R \land \phi\}$$

	Per esempio, $\sigma_{\text{Età} \geq 18}(\text{Studenti})$ per trovare le _ennuple_ degli studenti maggiorenni.

- **Prodotto**, per il [prodotto cartesiano](../../ct0432/01/README.md#operazioni) tra le _ennuple_ di $R$ ed $S$ aventi attributi **distinti**:

	$$R \times S = \{(t.A_1, ..., t.A_n, u.B_1, ..., u.B_m) \mid t \in R \land u \in S\}$$

	Per esempio, date le relazioni `Studenti` ed `Esami`:

	| <u>Matricola</u> | Nome |
	|:-:|:-:|
	| 11371 | Mario |
	| 12678 | Paolo |

	| <u>Codice</u> | Studente\* | Voto |
	|:-:|:-:|:-:|
	| 10822 | 11371 | 28 |
	| 10913 | 12678 | 30 |

	si possono trovare:
	$$\pi_{\text{Nome}}(\sigma_{\text{Voto} = 30}(\sigma_{\text{Matricola} = \text{Studente}}(\text{Studenti} \times \text{Esami})))$$
	ovvero gli studenti che hanno ottenuto `Voto` uguale a $30$.

Inoltre, dai precedenti operatori si possono ricavare gli **operatori derivati**:

- **Join**, per combinare le _ennuple_ con attributi _distinti_ ma aventi gli **stessi** valori su $A_i$ e $B_j$:

	$$R \underset{A_i = B_j}{\bowtie} S$$

- **Join naturale**, per combinare _ennuple_ attraverso attributi di $R$ ed $S$ aventi lo **stesso nome**:

	$$R \bowtie S$$
	che richiede la _ridenominazione_ degli attributi in comune se espresso con il _prodotto_.

	Per esempio, dati
	```
	Informazioni(CodInfo: string, Anno: int)
		PK(CodInfo)
	Libri(ISBN: string, CodInfo*: string)
		PK(ISBN)
		CodInfo FK(Informazioni)
	```
	si possono associare le due relazioni con $\text{Libri} \bowtie \text{Informazioni}$, oppure:
	$$
	\rho_{\text{ICod} \leftarrow \text{CodInfo}}
	(\pi_{\text{ISBN}, \text{ICod}, \text{Anno}}
	(\sigma_{\text{LCod} = \text{ICod}}
	(\rho_{\text{CodInfo} \leftarrow \text{LCod}}(\text{Libri}) \times
	\rho_{\text{CodInfo} \leftarrow \text{ICod}}(\text{Informazioni}))))
	$$

- **Intersezione**, sulle _ennuple_ delle relazioni $R$ ed $S$ dello _stesso tipo_:

	$$R \cap S = R \setminus (R \setminus S)$$
