# Assiomi

Un assioma è un _enunciato_, che anche se non dimostrato, **è considerato vero** perchè _evidente_.

Sia $G$ un **insieme** e $\ast$ un'**operazione binaria** (operazione con due operandi) appartenente ad un **gruppo** $(G, \ast)$. \
$\forall a, b \in G, a \ast b \in G \Rightarrow$ sono soddisfatti i seguenti assiomi:
1. **Proprietà associativa**: $(a \ast b) \ast c = a \ast (b \ast c), \forall a, b, c \in G$
2. **Esistenza dell'elemento neutro**: $\exists e \in G : a \ast e = a, \forall a \in G$
3. **Esistenza dell'inverso**: $\forall a \in G, \exists a^{-1} : a \ast a^{-1} = e$ cioè l'elemento neutro

Per cui l'insieme dei numeri reali $\mathbb{R}$ soddisfa quattro assiomi:

1. **Somma** ($+$)
	- Associativa: $a + (b + c) = (a + b) + c$
	- Esistenza dell'elemento neutro: $a + 0 = a$
	- Esistenza dell'inverso: $a + (-a) = 0$

	Inoltre, $(\mathbb{R}, +)$ è chiamato **gruppo commutativo** perchè soddisfa anche:
	- Commutativa: $a + b = b + a$

2. **Prodotto** ($\cdot$)
	- Associativa: $a \cdot (b \cdot c) = (a \cdot b) \cdot c$
	- Esistenza dell'elemento neutro: $a \cdot 1 = a$
	- Esistenza dell'inverso: $a \cdot a^{-1} = 1$

	In più, $(\mathbb{R}, +, \cdot)$ è chiamato **corpo commutativo** (o campo) perchè soddisfa anche:
	- Commutativa: $a \cdot b = b \cdot a$
	- Distributiva: $a \cdot (b + c) = a \cdot b + a \cdot c$

3. **Ordinamento**

	L'insieme $\mathbb{R}$ soddisfa anche una **relazione d'ordine** $(\mathbb{R}, \leq)$, per cui $x \leq y \lor y \geq x$.

	Possiede anche due proprietà per cui $\forall a, b \in \mathbb{R} : a \leq b$ si ha:
	- Compatibilità con la somma: $a + c \leq b + c, \forall c \in \mathbb{R}$
	- Compatibilità con il prodotto: $a \cdot c \leq b \cdot c, \forall c \in \mathbb{R}^+ \cup \{0\}$

	Un insieme che soddisfa anche questo assioma si dice **corpo commutativo ordinato**, come lo è $\mathbb{R}$.

4. **Completezza**

	Per ogni coppia di insiemi $A, B$ contenuti in $\mathbb{R}$ tale che $a \leq b$ per ogni $a \in A$ e $b \in B$, esiste un elemento $c \in \mathbb{R}$ (detto elemento separatore) tale che $a \leq c \leq b$.

	Nel caso di $\mathbb{Q}$, esso non soddisfa questo assioma, perchè $\nexists c \in \mathbb{Q} : c^2 = 2$.

	Quindi, $\mathbb{R}$ rispetta anche questo assioma.
