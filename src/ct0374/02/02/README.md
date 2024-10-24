# Forma normale di Chomsky

Una _CFG_ $G$ è detta in **forma normale di Chomsky** se ogni sua regola è in una delle seguenti forme:
- $A \rightarrow BC$, con $B, C \neq S$: lo _start symbol_ sta solo a sinistra e i _non-terminali_ a destra stanno in coppia
- $A \rightarrow a$: ogni opzione di $A$ ha un solo _terminale_
- $S_0 \rightarrow \epsilon$, se $\epsilon \in L(G)$: lo _start symbol_ $S_0$ è l'unico che può avere questa regola

Si può dimostrare che ogni linguaggio _context-free_ è generato da una _CFG_ in _forma normale di Chomsky_, infatti è possibile **convertire** una qualsiasi _CFG_ in forma normale attraverso delle trasformazioni:
1. Introdurre un **nuovo simbolo iniziale** $S_0 \rightarrow S$
2. **Eliminare le $\epsilon$-regole** $A \to \epsilon$ aggiungendo alle regole padre i casi in cui $A$ è presente oppure no
3. **Eliminare le regole unitarie** $A \rightarrow B$ rimpiazzando il _non-terminale_ a destra con il suo contenuto
4. **Dividi le regole** $A \rightarrow u_1 u_2 \cdots u_n$ per $n > 2$ e $u_i \in (V \cup \Sigma)$ in $A \rightarrow u_1A_1,\, ...,\, A_{n-2} \rightarrow u_{n-1} u_n$
5. **Rimpiazza ogni terminale non solitario** $a \in \Sigma$ con una regola $A \rightarrow a$

L'ordine del quarto e quinto punto non [causa problemi](https://en.wikipedia.org/wiki/Chomsky_normal_form#Order_of_transformations) perchè la divisione include anche i terminali.

Per esempio[^1],
$$
\begin{align*}
&\begin{split}
&S \rightarrow ASA \mid aB \\
&A \rightarrow B \mid S \\
&B \rightarrow b \mid \epsilon
\end{split}
\hspace{0.5em}\underset{1}{\Rightarrow}\hspace{1em}
\begin{split}
&S_0 \rightarrow S \\
&S \rightarrow ASA \mid aB \\
&A \rightarrow B \mid S \\
&B \rightarrow b \mid \epsilon
\end{split}
&&\underset{2}{\Rightarrow}\hspace{1em}
\begin{split}
&S_0 \rightarrow S \\
&S \rightarrow ASA \mid aB \mid a \\
&A \rightarrow B \mid S \mid \epsilon \\
&B \rightarrow b
\end{split}
&\Rightarrow \\[1em]
&\underset{2}{\Rightarrow}\hspace{1em}
\begin{split}
&S_0 \rightarrow S \\
&S \rightarrow ASA \mid aB \mid a \mid SA \mid AS \mid S \\
&A \rightarrow B \mid S \\
&B \rightarrow b
\end{split}
&&\underset{3}{\Rightarrow}\hspace{1em}
\begin{split}
&S_0 \rightarrow ASA \mid aB \mid a \mid SA \mid AS \\
&S \rightarrow ASA \mid aB \mid a \mid SA \mid AS \\
&A \rightarrow b \mid ASA \mid aB \mid a \mid SA \mid AS \\
&B \rightarrow b
\end{split}
&\Rightarrow \\[1em]
&\underset{4}{\Rightarrow}\hspace{1em}
\begin{split}
&S_0 \rightarrow AA_1 \mid aB \mid a \mid SA \mid AS \\
&S \rightarrow AA_1 \mid aB \mid a \mid SA \mid AS \\
&A \rightarrow b \mid AA_1 \mid aB \mid a \mid SA \mid AS \\
&B \rightarrow b \\
&A_1 \rightarrow SA
\end{split}
&&\underset{5}{\Rightarrow}\hspace{1em}
\begin{split}
&S_0 \rightarrow AA_1 \mid UB \mid a \mid SA \mid AS \\
&S \rightarrow AA_1 \mid UB \mid a \mid SA \mid AS \\
&A \rightarrow b \mid AA_1 \mid UB \mid a \mid SA \mid AS \\
&B \rightarrow b \\
&A_1 \rightarrow SA \\
&U \rightarrow a
\end{split}
\end{align*}
$$

[^1]: Verificabile con [CFG to CNF](https://devimam.github.io/cfgtocnf/)
