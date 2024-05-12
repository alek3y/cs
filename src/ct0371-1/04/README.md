# Teoria NP

Oltre a quelli visti, esistono anche problemi detti **intrattabili**, perchè non hanno soluzioni o richiedono tempo **non polinomiale**. Un esempio è il [problema della clique massima](../03/README.md#problema-della-clique-massima).

Un **problema** si può definire come la [relazione](../../ct0434/03/README.md) tra le **istanze** $\mathcal{I}$ degli input e le **soluzioni** $\mathcal{S}$ degli output:
$$
\mathscr{P} \subseteq \mathcal{I} \times \mathcal{S}
$$
e si possono suddividere in:
- **Problemi indecidibili**, se è **impossibile** costruirne una soluzione
- **Problemi decidibili**, se possiedono un algoritmo a **tempo finito**, e sono detti **trattabili** quando hanno tempo $O(n^k)$, con in genere $k \leq 4$, e **intrattabili** quando hanno tempo $O(k^n)$, con $k$ costante

Quelli più semplici vengono detti **decisionali** perchè $\mathcal{S} = \{\mathbf{0}, \mathbf{1}\}$ e quindi la soluzione da trovare è **binaria**, altrimenti vengono detti di **ottimizzazione** perchè tra molteplici soluzioni va cercata la migliore.

Le _istanze_ di problemi $\mathscr{P}$ _decisionali_ si suddividono quindi in **istanze positive** $\mathcal{I}^+$, ossia quelle per cui il problema restituisce $\mathbf{1} \in \mathcal{S}$, e **istanze negative** $\mathcal{I}^-$, per cui il problema restituisce $\mathbf{0} \in \mathcal{S}$.

Un esempio di problema di _ottimizzazione_ è il **travelling salesman problem**, che data una lista di città e di distanze tra ognuna di esse, cerca il **percorso più corto** che le visita tutte e **torna** alla città di origine.
