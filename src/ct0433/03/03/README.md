# Topologia

I concetti di **topologia** della [retta reale](../../../ct0432/02/03/README.md) si estendono anche su $\mathbb{R}^n$.

## Intorno sferico

Un **intorno sferico** di centro $x_0 \in \mathbb{R}^n$ e raggio $r > 0$ è definito come:
$$
U_r(x_0) = \{x \in \mathbb{R}^n : \|x - x_0\| < r\}
$$

## Punti interni ed esterni

Un punto $x_0 \in \mathbb{R}^n$ è definito, rispetto ad un **insieme di punti** $E \subseteq \mathbb{R}^n$:
- **interno**, se $x_0 \in E \land \exists r > 0 : U_r(x_0) \subset E$
- **esterno**, se $x_0 \not\in E \land \exists r > 0 : U_r(x_0) \subset E^C = \mathbb{R}^n \setminus E$
- **di frontiera**, se $\forall r > 0, U_r(x_0) \cap E \neq \emptyset \land U_r(x_0) \cap E^C \neq \emptyset$

Da cui si ricavano gli insiemi dei punti:
- $\mathring{E}$, o **parte interna**: cioè l'insieme dei **punti interni** ad $E$
- $\partial E$, o **bordo**: cioè l'insieme dei punti **di frontiera** ad $E$
- $\bar{E} = E \cup \partial E$, o **chiusura**

## Proprietà

Un insieme $E \subseteq \mathbb{R}^n$ si dice:
- **Aperto**: se $E = \mathring{E}$ e quindi se $E$ non ha alcun punto di $\partial E$
- **Chiuso**: se $E = \bar{E}$ e quindi se $E$ contiene tutti i punti di $\partial E$
- **Limitato**: se $\exists r > 0 : E \subset U_r((0, 0))$
- **Connesso**: se dati $x_{1,2} \in E$, esiste una curva $z \in C^0$ per cui $x_{1,2} \in \gamma(z) \subseteq E$

Per esempio, $E = \{(x, y) \in \mathbb{R}^2 : 1 \leq (x-1)^2 + (y+1)^2 \leq 3\}$ è l'area tra le circonferenze con centro $(1, -1)$ di raggio $1$ e $\sqrt{3}$ ed è _chiusa_ perchè $\partial E \subset E$, _limitata_ perchè $E \subset U_{10}((0, 0))$ e _connessa_.
