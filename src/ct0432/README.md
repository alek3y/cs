# Calcolo 1

## Connettivi

- $\lor$, o, _or_: $\mathcal{P} \lor \mathcal{L}$ è vera quando lo è una delle due
- $\land$, e, _and_: $\mathcal{P} \land \mathcal{L}$ è vera quando lo sono entrambe
- $non$, negazione, _not_: $non \mathcal{P}$ è vera quando $\mathcal{P}$ è falsa
- $\Leftrightarrow$, se e solo se: $\mathcal{P} \Leftrightarrow \mathcal{L}$ è vera se entrambe le preposizioni concordano (sono entrambe _vere_ o _false_)
- $\Rightarrow$, implica, se [...] allora: $\mathcal{P} \Rightarrow \mathcal{L}$
- $|$, $:$, tale che: $\mathcal{P} : \mathcal{L}$

## Quantificatori

- $\forall$, per ogni, quantificatore universale: $\forall x \in \mathbb{N} \Rightarrow x \geq 0$
- $\exists$, esiste almeno, quantificatore esistenziale: $\exists x \in \mathbb{C} : x^2 = -1$
- $\exists!$, esiste un solo, quantificatore esistenziale: $\exists! x \in \mathbb{R} : \sqrt{x} = 0$

## Insiemi

- $\in$, appartiene: $x \in \mathbb{R}$ (anche scritto $\mathbb{R} \ni x$) significa che $x$ è contenuto nell'insieme dei numeri reali
- $\not\in$, non appartiene: $i \not\in \mathbb{R}$
- $\emptyset$, insieme vuoto
- $\#A$, $|A|$, cardinalità, numero di elementi: $|\mathbb{R}| = \infty$
- $\subseteq$, sottoinsieme, _subset_, **è contenuto** (i.e. è contenuto o è uguale): $A \subseteq B$ significa che $A$ è sottoinsieme di $B$ e cioè che tutti gli elementi di $A$ appartengono a $B$ (i.e. $\forall x \in A \Rightarrow x \in B$)
- $\supseteq$, soprainsieme, _superset_, **contiene** (i.e. contiene o è uguale): $B \supseteq A$ significa che $B$ è soprainsieme di $A$ (i.e. $(\exists x \in B : x \not\in A) \lor (B = A)$), o anche che $A$ è sottoinsieme di $B$
- $\subset$, $\subsetneq$, sottoinsieme proprio, **è contenuto strettamente** (i.e. è contenuto ma non è uguale): $A \subset B \text{ se } (\forall x \in A \Rightarrow x \in B) \land (\exists y \in B : y \not\in A)$
- $\supset$, $\supsetneq$, soprainsieme proprio, **contiene strettamente** (i.e. contiene ma non è uguale)
