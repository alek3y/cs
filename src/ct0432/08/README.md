# Successioni

Una **successione**,
$$
\begin{split}
a\colon\space &A \subseteq \mathbb{N} \to \mathbb{R}\newline
&n \mapsto a_n
\end{split}
$$
è una successione di elementi $a_n$.

Si dice **sottosuccessione** di $a_n$, una successione che ha il dominio di $a_n$ ristretto da $A$ a $B \subset A$.

Per esempio, $A = \{\frac{1}{n}, n \in \mathbb{N}^+\}$ è una successione con $a_n = \frac{1}{n}$.

## Limite di una successione

Sia
$$\lim_{\underset{n \in A \subseteq \mathbb{N}}{n \to +\infty}} a_n = l$$
allora quando:
- $l \in \mathbb{R}$, la successione è **convergente**
- $l = \pm\infty$, la successione è **divergente**
- $\nexists l$, la successione è **indeterminata**

## Funzione di una successione ristretta

Se $(a_n)$ è una restrizione a di $f(x)$, allora si può dire che:
$$\lim_{x \to +\infty} f(x) = l \in \mathbb{R} \cup \{\pm\infty\} \Rightarrow \lim_{n \to +\infty} a_n = l$$
