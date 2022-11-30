# Numeri primi

Un numero $p > 1$ è detto **primo** se è divisibile solamente da $1$ e $p$. \
Si dice **composto** invece, se $p = ab$ con $a, b \neq 1, p$.

Nella sequenza di numeri primi $\{2, 3, 5, 7, ...\}$ sono anche presenti **infiniti** numeri **primi gemelli**, cioè coppie di numeri primi nella forma $(p, p+2)$.

## MCD e mcm

Avendo $a, b \in \mathbb{N} : a, b > 1$, scoposti in fattori primi:
$$
a = p_1^{n_1} \cdot ... \cdot p_k^{n_k} \hspace{1em}
b = p_1^{m_1} \cdot ... \cdot p_k^{m_k}
$$
si ha che:
$$\mathrm{MCD}(a, b) = p_1^{\min(n_1, m_1)} \cdot ... \cdot p_k^{\min(n_k, m_k)}$$
$$\mathrm{mcm}(a, b) = p_1^{\max(n_1, m_1)} \cdot ... \cdot p_k^{\max(n_k, m_k)}$$

## Teoremi

1. **Teorema sull'esistenza della fattorizzazione**:

	Ogni numero $n \geq 2$ è esprimibile come prodotto di un numero finito di **fattori primi**:
	$$n = p_1^{n_1} \cdot p_2^{n_2} \cdot ... \cdot p_k^{n_k}$$

	Per esempio, $100 = 2^2 \cdot 3^0 \cdot 5^2$.

2. **Teorema fondamentale dell'aritmentica**:

	Ogni numbero $n \geq 2$ ha la fattorizzazione in fattori primi **unica**.

3. **Piccolo teorema di Fermat**:

	Sia $p$ primo e $a \in \mathbb{Z}$ tali che $p \not\left|\right. a$, allora:
	$$a^{p - 1} \equiv_p 1$$

	In alternativa, si può generalizzare con $p$ primo e $a \in \mathbb{Z}$ in:
	$$a^p \equiv_p a$$

4. **Teorema di Eulero**:

	Sia $\phi(n)$ la funzione di Eulero che ritorna il numero di interi _coprimi_ con $n$ da $1$ ad $n$, si ha che:
	$$a^{\phi(n)} \equiv_n 1 \Leftrightarrow \mathrm{MCD}(a, n) = 1$$

5. **Teorema cinese del resto**:

	Il sistema di equazioni
$$
\begin{cases}
x \equiv_{n_1} a_1 \\
x \equiv_{n_2} a_2 \\
\vdots \\
x \equiv_{n_k} a_k \\
\end{cases}
$$
	ammette un **unica soluzione** (in modulo $n_1 \cdot ... \cdot n_k$) se solo se:
$$\mathrm{MCD}(n_i, n_j) = 1,\; \forall (i, j) \in (\mathbb{N}^+)^2 : i \neq j$$
	cioè quando tutte le coppie sono coprime.
