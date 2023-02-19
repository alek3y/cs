# Equazioni Differenziali Ordinarie

Un'equazione ordinaria di ordine $n$ sarà del tipo:
$$F(t, y, y', ..., y^{(n)}) = 0$$
dove $t$ è la **variabile indipendente**, $y(t)$ è la **funzione incognita** e $F$ è la funzione che le mette in relazione.

L'**integrale generale** sarà la famiglia di funzioni che rendono la relazione di $F$ vera.

## Primo ordine

Una relazione di primo ordine
$$F(t, y, y') = 0$$
è in **forma normale** quando $y' = f(t, y)$.

Avendo una **condizione iniziale** è possibile trovare il parametro che genera l'_integrale generale_:
$$
\begin{cases}
y'(t) = f(t, y) \\
y(t_0) = y_0
\end{cases}
$$
per cui si ha un **problema di Cauchy**.

### Equazioni a variabili separabili

Per risolvere le equazioni differenziali in forma
$$y' = a(t)b(y)$$
si ha che:
- $b(y) = 0 \Rightarrow \exists k \in \mathbb{R} : y(t) = k \land b(k) = 0$ dove $k$ è la **soluzione costante**
- $b(y) \neq 0$, allora:
$$
\frac{y'}{b(y)} = a(t)
\Rightarrow
\int \frac{y'(t)}{b(y(t))} dt = \int a(t) dt
\underset{z = y(t),\; dz = y'(t)dt}{\Leftrightarrow}
\int \frac{1}{b(z)} dz = \int a(t) dt
$$
oppure, più semplicemente:
$$
y' = a(t)b(y)
\Rightarrow
\frac{dy}{dt} = a(t)b(y)
\Rightarrow
\frac{dy}{b(y)} = a(t)dt
\Rightarrow
\int \frac{1}{b(y)} dy = \int a(t) dt
$$

da cui si risolvono gli integrali e si esplicita la $y$.

Per esempio, con $y' = (t-1)(y-1)$ si ha:
- $(y - 1) = 0 \Leftrightarrow y = 1$, per cui $y(t) = 1$ è la _soluzione costante_
- $y \neq 1$, allora:
$$
\int \frac{y'(t)}{y(t) - 1} dt = \int t - 1 dt \\
\Downarrow \\
\ln|y - 1| = \frac{t^2}{2} - t + c \\
\Downarrow \\
y = 1 + \pm e^{\frac{t^2}{2} - t + c} = 1 + ke^{\frac{t^2}{2} - t},\; k \in \mathbb{R}
$$
dove $k$ include la _soluzione costante_, infatti $k = 0 \Rightarrow y = 1$.
