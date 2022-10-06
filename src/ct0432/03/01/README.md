# Immagine e controimmagine

## Immagine

L'immagine di una funzione $f: D \to \mathbb{R}$ verrà rappresentata da $f(D)$ o $\mathrm{Im}(f)$, ed è costituita dai valori assunti dalla funzione:
$$f(D) = \{y \in \mathbb{R} | y = f(x) \text{ per qualche } x \in D\}$$

In poche parole, l'**immagine** di una funzione fa parte del suo **codominio**, cioè a tutti i valori di $f(x)$. \
Il _codominio_ però, **è diverso dall'immagine** (che non viene calcolata), visto che è interamente specificato dalla **firma** della funzione (e.g. $f: A \to B$).

Per esempio, nel caso della funzione $f(x) = mx + q$, che comprende tutte le rette **non verticali**, l'immagine sarà $f(D) = \begin{cases} \mathbb{R} & \text{se } m \neq 0 \\ \{q\} & \text{se } m = 0, \text{ retta orizzontale} \end{cases}$

## Controimmagine

La controimmagine di una funzione $f: D \to B$ è rappresentata da $\overleftarrow f(B)$, e viene descritta come:
$$\overleftarrow f(B) = \{x \in D | f(x) \in B\}$$

In poche parole, la **controimmagine** di una funzione fa parte del suo **dominio**, cioè a tutti i valori assunti da $x$. Il _dominio_ **non è la stessa cosa della controimmagine** (visto che non viene calcolata), dato che viene specificato nella firma della funzione.

### Esempio

Sia $f(x) = -2x^2 + 4x$,
- $\overleftarrow f(\{0\}) = \{0, 2\}$, perchè i punti $x$ del _dominio_ corrispondenti al punto $y = 0$ del _codominio_ sono $\{0, 2\}$
- $\overleftarrow f(\{2\}) = \{1\}$, dato che sul punto $x = 1$ si ha il vertice $y = 2$
- $\overleftarrow f([-1, 3)) = \left[1 - \frac{\sqrt{6}}{2}, 1 + \frac{\sqrt{6}}{2}\right]$, dato che i punti del _codominio_ sono $(-\infty, 2]$ (dove $2$ è il vertice), per cui i punti del _dominio_ che corrispondono a $[-1, 3)$ sono limitati dalle $x$ su $y = -1$, quindi $x = 1 \pm \frac{\sqrt{6}}{2}$
