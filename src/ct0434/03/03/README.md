# Catene e anticatene

Un insieme $A$ si dice **catena** se è **totalmente ordinato** secondo una relazione $R$ **d'ordine** (anche _parziale_) e quindi ogni elemento viene confrontato a due a due risultando in un unica sequenza ordinata da $R$.

Si dice **anticatena** invece, se **ogni elemento** è **inconfrontabile** a due a due.

## Ordinamento ben fondato

Una relazione $R$ **d'ordine parziale** si dice **ben fondata** su un insieme $X$, quando
$$\exists m \in A : \neg (sRm), \forall s \in A \text{ con } s \neq m \hspace{2em} \forall A \subseteq X : A \neq \emptyset$$
cioè quando **esiste** un elemento **minimo** $m$ **su ogni sottoinsieme di** $X$, per cui $s$ non è mai in relazione con $m$, per ogni $s \in A$ (escluso $m$ nel caso in cui la relazione sia _riflessiva_).

Per esempio su $(\mathbb{N}, <)$, $m = 0$ perchè non c'è alcuna $s \in \mathbb{N}$ per cui $s < 0$.

Quando uno dei sottoinsiemi $A \subseteq X$ è **senza un minimo** e la sequenza di confronti continua all'infinito qualsiasi sia l'elemento iniziale, allora si ha una **catena discendente infinita**.

Per esempio, con $(\mathbb{R}^\geq, <)$ si finirà per ottenere una _catena discendente infinita_, perchè esistono dei sottoinsiemi (e.g. $\mathbb{R}^+$, quindi senza lo $0$) che non avranno un minimo $m$.
