# Algebra relazionale

Le **interrogazioni** al _database_ possono essere formalmente rappresentate dall'**algebra relazionale**.

Ogni **relazione** `R(A1: T1, ..., An: Tn)` è considerata un **insieme di ennuple** $\{(A_1\colon T_1, ..., A_n: T_n)\}$, dove il **grado** è definito come il numero di attributi e la **cardinalità** il numero di _ennuple_.

## Operatori principali

- **Ridenominazione**, e.g. per rinominare l'attributo $A_1$ a $B_1$:

	$$\rho_{A_1 \leftarrow B_1}(R)$$
	che avrà _cardinalità_ e _grado_ uguali ad $R$.

	Diventa utile nelle operazioni tra relazioni con attributi con lo **stesso nome**.

- **Unione** e **differenza**, sulle _ennuple_ di due relazioni $R$ ed $S$ dello **stesso tipo**:

	$$R \cup S$$
	con _cardinalità_ $|R| + |S| - |R \cap S|$, e
	$$R - S$$
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

## Operatori derivati

Dagli operatori principali è possibile ricavare degli **operatori derivati**, tra cui:

- **Join**, per combinare le _ennuple_ con attributi _distinti_ ma aventi gli **stessi** valori su $A_i$ e $B_j$:

	$$R \underset{A_i = B_j}{\bowtie} S = \sigma_{A_i = B_j}(R \times S)$$

- **Natural join**, come la _join_ ma attraverso attributi di $R$ ed $S$ aventi lo **stesso nome**:

	$$R \bowtie S$$
	che, se espresso con il _prodotto_, richiede la _ridenominazione_ degli attributi in comune.

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

- **Outer join**, come la _join_ ma tenendo le _ennuple_ **non combinabili** impostando i nuovi attributi a `NULL`:

	$$R \overset{\leftrightarrow}{\bowtie} S$$

- **Intersezione**, sulle _ennuple_ delle relazioni $R$ ed $S$ dello _stesso tipo_:

	$$R \cap S = R - (R - S)$$

- **Divisione**, per raggruppare tutte le _ennuple_ di $R$ associate a tutti gli attributi di $S \neq \emptyset$:

	$$R \div S = \pi_X(R) - \pi_X((\pi_X(R) \times S) - R)$$
	dove $R$ contiene gli attributi $X$ e $Y$, mentre $S$ contiene solamente gli attributi $Y$.

	Per esempio, siano $E = \pi_{\text{Studente},\text{Materia}}(\text{Esami})$ e $A = \pi_{\text{Materia}}(\sigma_{\text{Studente} = \text{"76366"}}(E))$, allora $E \div A$ restituisce gli studenti che hanno fatto **al minimo** tutti gli esami $A$.

	Inoltre, le _ennuple_ associate **esattamente** (né più né meno) a tutte quelle in $S$ si possono ottenere con:
	$$R \div S \cap (\pi_X(R) - \pi_X(R - (\pi_X(R) \times S)))$$
	dove il secondo termine contiene le _ennuple_ in $R$ associate **al più** a quelle in $S$.

## Operatori speciali

- **Proiezione generalizzata**, per dare un nome a delle espressioni:

	$$\pi_{\text{Expr}_1 \text{ as } A_1, ..., \text{Expr}_n \text{ as } A_n}(R)$$

	Per esempio, $\pi_{\text{Codice}, \text{Prezzo}*\text{Qty}\text{ as } \text{Totale}}(\text{Acquisti})$ saranno _ennuple_ di tipo `(Codice, Totale)`.

- **Raggruppamento**, per raggruppare più _ennuple_ attraverso delle **funzioni di aggregazione**:

	$$_{A_1, ..., A_n}\gamma_{f_1, ..., f_n}(R)$$

	dove $f_1, ..., f_n$ sono espressioni che _aggregano_ più _ennuple_ che hanno gli stessi $A_1, ..., A_n$:
	- $\mathrm{sum}(A_i)$: **somma** i valori sull'attributo $A_i$
	- $\mathrm{avg}(A_i)$: fa la **media** di tutti i valori di $A_i$
	- $\mathrm{count}(\ast)$: **conta** le _ennuple_
	- $\mathrm{min}(A_i)$ e $\mathrm{max}(A_i)$: trovano il **minimo** e il **massimo** dei valori su $A_i$
	- $\text{-distinct}(A_i)$, suffisso alle precedenti funzioni per **rimuovere i duplicati**

	Per esempio, dato
	$$_{\text{Candidato}}\gamma_{\mathrm{count}(\ast), \mathrm{min}(\text{Voto}), \mathrm{max}(\text{Voto}), \mathrm{avg}(\text{Voto})}(\text{Esami})$$
	si trovano il _numero di esami_, il _minimo_, il _massimo_ e la _media dei voti_ di ogni `Candidato`.

## Operatori sui multiinsiemi

Se nelle relazioni sono presenti più _ennuple_ uguali, vanno sfruttati gli operatori per i [multiinsiemi](../../ct0434/10/README.md#multiinsieme):

- **Proiezione senza rimuovere duplicati**:

	$$\pi^b_{A_1, ..., A_n}(R)$$

- **Eliminazione duplicati**, per rimuovere le _ennuple_ con gli stessi valori:

	$$\delta(R)$$

- **Ordinamento**, per ordinare [lessicograficamente](https://it.wikipedia.org/wiki/Ordine_lessicografico) le _ennuple_ con priorità da $A_1$ fino ad $A_n$:

	$$\tau_{A_1, ..., A_n}(R)$$

- **Unione**, **intersezione** e **differenza**, e.g. con un _ennupla_ $t$ che appare $n$ volte in $R$ e $m$ volte in $S$:

	$$R \cup^b S$$
	su cui $t$ appare $n + m$ volte.

	$$R \cap^b S$$
	su cui $t$ appare $\min(n, m)$ volte.

	$$R -^b S$$
	su cui $t$ appare $\max(0, n-m)$ volte.

## Trasformazioni

Agli operatori possono essere applicate delle regole di equivalenza per **manipolare** le espressioni:
- $\pi_{A_1}(\pi_{A_1, A_2}(R)) = \pi_{A_1}(R)$
- $\sigma_{\phi_1}(\sigma_{\phi_2}(R)) = \sigma_{\phi_1 \land \phi_2}(R)$
- $R \times (S \times T) = (R \times S) \times T$
- $R \times S = S \times R$
- $\sigma_{\phi}(_{A_1}\gamma_{f_1}(R)) = _{A_1}\gamma_{f_1}(\sigma_{\phi}(R))$
