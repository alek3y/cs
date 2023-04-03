# Calcolo dei limiti

Attraverso delle **sezioni**, è possibile convergere su un punto $(x_0, y_0)$ da _diverse direzioni_ sul piano $xy$.
Sfruttando le [disuguaglianze](../../04/README.md) invece, è possibile **semplificare** i limiti per trovare una funzione di [confronto](../../../../ct0432/04/01/README.md).

Per esempio, per:
$$
\lim_{(x, y) \to (0, 0)} \frac{y(x^2 + 3y^2)}{x^2 + 5y^2}
$$
si può convergere dalla retta $x = 0$ o dalla retta $y = 0$, per cui:
$$
x = 0 \Rightarrow \lim_{y \to 0} \frac{3y^3}{5y^2} = 0
\hspace{1em} \lor \hspace{1em}
y = 0 \Rightarrow \lim_{x \to 0} \frac{0}{x^2} = 0
$$
Perchè il limite esista, deve convergere su $0$ da **tutte le direzioni**, altrimenti [basta](https://www.symbolab.com/solver/step-by-step/%5Clim_%7B%5Cleft%28x%2Cy%5Cright%29%5Cto%5Cleft%280%2C0%5Cright%29%7D%5Cleft%28%5Cfrac%7Bxy%7D%7Bx%5E%7B2%7D%2By%5E%7B2%7D%7D%5Cright%29?or=input) che **solo uno** sia diverso.

Quindi, va verificato che
$$
\lim_{(x, y) \to (0, 0)} \frac{y(x^2 + 3y^2)}{x^2 + 5y^2} \underset{\left[\frac{0}{0}\right]}{=} 0
$$
**secondo** la proprietà:
$$
\lim_{x \to x_0} f(x) = L \Leftrightarrow \lim_{x \to x_0} |f(x) - L| = 0
$$

Per il _teorema del confronto_, dato che $L = 0$, e che $\frac{x^2 + 3y^2}{x^2 + 5y^2} \leq 1$:
$$
\begin{array}{rcccl}
0 & \leq & | \frac{y(x^2 + 3y^2)}{x^2 + 5y^2} - 0 | & \leq & |y| \\
& \searrow & \downarrow & \swarrow & \\
&& 0 &&
\end{array}
