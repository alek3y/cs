# Classi decisionali

Le due classi principali sono la **classe P**, cioè l'insieme $\mathrm{P}$ di tutti i problemi _decisionali_ $\mathscr{P}$ **risolvibili** in tempo _polinomiale_, e la **classe NP**, cioè l'insieme $\mathrm{NP}$ dei problemi che sono **verificabili** in tempo _polinomiale_.

Da queste definizioni si può quindi dedurre che $\mathrm{P} \subseteq \mathrm{NP}$.

Un problema è _verificabile_ se esiste un **algoritmo di verifica** che ha come input un $i \in \mathcal{I}^+$ e un **certificato**, cioè una dimostrazione della soluzione. Si considera verificato se il _certificato_ è una **soluzione valida** per $i$.

Di conseguenza, per i $\mathscr{P} \in \mathrm{NP}$ è **facile** verificare le istanze $\mathcal{I}^+$ e **forse difficile** verificare quelle $\mathcal{I}^-$.

## Riducibilità

Le ulteriori classi dipendono dalla **riducibilità polinomiale** di due problemi $\mathscr{P}_1$ e $\mathscr{P}_2$, definita come:
$$
\mathscr{P}_1 \leq_P \mathscr{P}_2
$$
per cui esiste un algoritmo _polinomiale_ che **trasforma** le _istanze_ di $\mathscr{P}_1$ in _istanze_ equivalenti di $\mathscr{P}_2$.

Per la _riducibilità_ vale che la trasformazione deve **mantenere la positività** delle _istanze_, ed inoltre è:
- **Riflessiva**, ovvero _riducibile_ a se stesso:
	$$
	\mathscr{P} \leq_P \mathscr{P}
	$$

- **Transitiva**, ovvero _riducibile_ a catena:
	$$
	\begin{cases}
	\mathscr{P}_1 \leq_P \mathscr{P}_2 \\
	\mathscr{P}_2 \leq_P \mathscr{P}_3
	\end{cases} \;\Rightarrow\;
	\mathscr{P}_1 \leq_P \mathscr{P}_3
	$$

	Questo è dimostrabile perchè **per ipotesi esistono** gli algoritmi
	$$
	A_{12}\colon \mathcal{I}_1 \to \mathcal{I}_2 \\
	A_{23}\colon \mathcal{I}_2 \to \mathcal{I}_3
	$$
	e di conseguenza si può costruire $A_{13}$ come:
	$$
	\begin{split}
	A_{13}\colon \mathcal{I}_1 &\to \mathcal{I}_3 \\
	i &\mapsto A_{23}(A_{12}(i))
	\end{split}
	$$

- **Non sempre simmetrica**, ovvero:
	$$
	\exists \mathscr{P}_{1,2} : \mathscr{P}_1 \leq_P \mathscr{P}_2 \land \neg(\mathscr{P}_2 \leq_P \mathscr{P}_1)
	$$

## Classe Co-NP

La **classe Co-NP** è l'insieme $\text{Co-NP}$ di tutti i problemi _decisionali_ per cui il loro complemento è _verificabile_.

Il **complemento di un problema** $\bar{\mathscr{P}}$ è definito come il problema $\mathscr{P}$ in cui le _istanze positive_ $\mathcal{I}^+$ si **invertono di ruolo** con le _istanze negative_ $\mathcal{I}^-$, e quindi saranno queste ultime ad essere **facili** da verificare.

## Classe NP-hard

La **classe NP-hard** è l'insieme $\text{NP-hard}$ di ogni problema _decisionale_ $\mathscr{P}$ per cui vale che:
$$
\forall \mathscr{P}' \in \mathrm{NP},\; \mathscr{P}' \leq_P \mathscr{P}
$$
per cui i problemi sono **almeno** tanto difficili quanto quelli in $\mathrm{NP}$.

## Classe NP-C

La **classe NP-C** è l'insieme $\text{NP-C}$ di ogni problema _decisionale_ $\mathscr{P}$, detto **NP completo**, per cui vale che:
$$
\mathscr{P} \in \mathrm{NP} \;\land\; \mathscr{P} \in \text{NP-hard}
$$
da cui si può ricavare il **teorema fondamentale della NP completezza**:
$$
\mathrm{P} \cap \text{NP-C} \neq \emptyset \Rightarrow \mathrm{P} = \mathrm{NP}
$$
anche se non è ancora stato trovato un problema dell'intersezione o provato che non ne esistono.

Si può dimostrare perchè **per ipotesi** $\exists \mathscr{P} \in \mathrm{P} \cap \text{NP-C}$ e, siccome $\mathscr{P} \in \text{NP-C}$ si ha che, **per definizione** ogni $Q \in \mathrm{NP}$ è **riducibile** a $\mathscr{P}$.
Di conseguenza, dato che $\mathscr{P} \in \mathrm{P}$, anche $Q \in P$ e quindi $P = NP$.

Inoltre, dato un problema $\mathscr{P} \in \mathrm{NP}$ è possibile **riconoscerlo NP completo** se:
$$
\exists Q \in \text{NP-C} : Q \leq_P \mathscr{P}
$$
perchè ogni problema in $\mathrm{NP}$ è _riducibile_ a $Q$, e quindi lo sono anche a $\mathscr{P}$ per la _transitività_.
