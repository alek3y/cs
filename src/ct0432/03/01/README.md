# Immagine e controimmagine

## Immagine

Si dice **immagine** di un valore $x$, il valore $y$ che **assume la funzione** su $x$, cioè $y = f(x)$.

Per esempio, l'immagine di $2$ sulla funzione $f(x) = 2x + 1$ è $f(2) = 5$.

L'immagine di un **insieme di valori** $C \subseteq \mathrm{Dom}(f)$, è l'insieme di tutte le $y$ che assume $f$ su $C$:
$$f(C) = \{f(x) \in \mathrm{Codom}(f) | x \in C\}$$

Quando si parla di **immagine di una funzione** $f(\mathrm{Dom}(f))$ o $\mathrm{Im}(f)$ quindi, si intendono tutti i valori assunti dalla funzione sul _codominio_.

Il codominio però, è diverso dall'immagine di una funzione $f$, perchè $Im(f)$ si calcola, mentre $\mathrm{Codom}(f)$ è specificato nella **firma** della funzione (e.g. $f: A \to B \Rightarrow B = \mathrm{Codom}(f)$).

Per esempio, nel caso della funzione $f(x) = mx + q$, che comprende tutte le rette **non verticali**, l'immagine sarà $f(D) = \begin{cases} \mathbb{R} & \text{se } m \neq 0 \\ \{q\} & \text{se } m = 0, \text{ retta orizzontale} \end{cases}$, con $D = \mathrm{Dom}(f)$.

## Controimmagine

La **controimmagine** di una funzione $f: D \to B$ è rappresentata da $\overleftarrow f(B)$, e viene descritta come:
$$\overleftarrow f(B) = \{x | f(x) \in B \land x \in D\}$$

Analogamente all'immagine, anche il dominio è diverso dalla controimmagine, perchè quest'ultima va calcolata e il diminio è specificato nella **firma**.

### Esempio

Sia $f(x) = -2x^2 + 4x$,
- $\overleftarrow f(\{0\}) = \{0, 2\}$, perchè i punti $x$ del _dominio_ corrispondenti al punto $y = 0$ del _codominio_ sono $\{0, 2\}$
- $\overleftarrow f(\{2\}) = \{1\}$, dato che sul punto $x = 1$ si ha il vertice $y = 2$
- $\overleftarrow f([-1, 3)) = \left[1 - \frac{\sqrt{6}}{2}, 1 + \frac{\sqrt{6}}{2}\right]$, dato che i punti del _codominio_ sono $(-\infty, 2]$ (dove $2$ è il vertice), per cui i punti del _dominio_ che corrispondono a $[-1, 3)$ sono limitati dalle $x$ su $y = -1$, quindi $x = 1 \pm \frac{\sqrt{6}}{2}$
