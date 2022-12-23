# Spazio vettoriale

L'insieme $V$ si dice **spazio vettoriale** quando contiene **vettori** ed è definito su un [**campo**](../../ct0432/02/01/README.md) $K$ (cioè un insieme di _scalari_) che rispetta:
- $(K, +)$ è **abeliano** (o _gruppo commutativo_) con elemento neutro $0$
- $(K^\ast, \cdot)$ è **abeliano** con elemento neutro $1$

dove la somma è $+\colon V \times V \to V$, mentre il prodotto $\cdot\colon K \times V \to V$.

Ogni _spazio vettoriale_ deve quindi soddisfare le proprietà:
- Della **somma**:
	1. **Commutativa**: $\vec{u} + (\vec{v} + \vec{w}) = (\vec{u} + \vec{v}) + \vec{w}$
	2. **Associativa**: $\vec{v} + \vec{w} = \vec{w} + \vec{v}$
	3. **Elemento neutro**: $\vec{v} + \vec{0} = \vec{v}$
	4. **Elemento opposto**: $\vec{v} + (-\vec{v}) = \vec{0}$

- Del **prodotto**:
	1. **Distributiva a destra**: $a(\vec{v} + \vec{w}) = a\vec{v} + a\vec{w}$
	2. **Distributiva a sinistra**: $(a + b)\vec{v} = a\vec{v} + b\vec{v}$
	3. **Associativa**: $a(b\vec{v}) = (ab)\vec{v}$
	4. **Elemento neutro**: $1\vec{v} = \vec{v}$

## Sottospazio

Un **sottospazio** $W$ dello _spazio vettoriale_ $V$, entrambi definiti sul _campo_ $K$, si definisce tale se:
$$W \subset V : W \neq \emptyset$$
e le due operazioni $+$ e $\cdot$ su $K$ rispettano la **proprietà di chiusura**, infatti:
- $\vec{v}, \vec{w} \in W \Rightarrow (\vec{v} + \vec{w}) \in W$
- $a \in K \land \vec{v} \in W \Rightarrow a\vec{v} \in W$

Di conseguenza,
$$\forall \vec{v} \in W,\, a \in K : a = 0 \Rightarrow a\vec{v} = \vec{0} \Rightarrow \vec{0} \in W$$
quindi, perchè si possa definire **spazio vettoriale** e rispetti la **proprietà di chiusura**, $\vec{0} \in W,\; \forall W \subset V$.

Per cui, ogni _iperpiano_ $W$ in $R^n$ che passa per l'origine $\vec{0}$ è considerabile _sottospazio vettoriale_, perchè:
$$\forall \vec{v} \in W,\; 0\vec{v} = \vec{0} \in W$$
