# Primo ordine

Una relazione di primo ordine
$$F(t, y, y') = 0$$
è in **forma normale** quando $y' = f(t, y)$.

L'_integrale generale_ si può trovare in base a $F$, cioè se è:
- Del tipo $y' = f(t)$, per cui $\int y' dt = \int f(t) dt$
- [A variabili separabili](#equazioni-a-variabili-separabili)
- [Lineari omogenee e complete](#equazioni-lineari)

Il _parametro_ che genera la famiglia di soluzioni invece, si può trovare con una **condizione iniziale**:
$$
\begin{cases}
y'(t) = f(t, y) \\
y(t_0) = y_0
\end{cases}
$$
per cui si ha un **problema di Cauchy**.

## Equazioni a variabili separabili

Per risolvere le equazioni differenziali in forma
$$y' = a(t)b(y)$$
con $y$ e $y'$ **funzioni** di $t$, si ha che:
- $b(y) = 0 \Rightarrow \exists k \in \mathbb{R} : b(k) = 0$ e quindi $y(t) = k$ è **soluzione costante**
- $b(y) \neq 0$, allora:
$$
\frac{y'}{b(y)} = a(t)
\Rightarrow
\int \frac{y'}{b(y)} dt = \int a(t) dt
\underset{z = y,\; dz = y'dt}{\Leftrightarrow}
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
- $(y - 1) = 0 \Leftrightarrow y = 1$, per cui $y(t) = 1$ è _soluzione costante_
- $y \neq 1$, allora:
$$
\int \frac{y'}{y - 1} dt = \int t - 1 dt \\
\Downarrow \\
\ln|y - 1| = \frac{t^2}{2} - t + c \\
\Downarrow \\
y = 1 \pm e^{\frac{t^2}{2} - t + c} = 1 + ke^{\frac{t^2}{2} - t},\; k \in \mathbb{R}
$$
dove $k$ include la _soluzione costante_, infatti $k = 0 \Rightarrow y = 1$.

## Equazioni lineari

Un'equazione differenziale si dice **lineare** se ha forma:
$$
h(t)y' + g(t)y = f(t) \\
\Downarrow \\
y' + a(t)y = b(t)
$$
con $h(t) \neq 0$ dato che, se lo fosse, non sarebbe un'equazione differenziale.

### Lineari omogenee

Quando $b(t) = 0$, l'equazione si dice **lineare omogenea** e va usato il metodo **a variabili separabili**:
$$
y' = -a(t)y \Rightarrow \int \frac{1}{y} dy = -\int a(t) dt \\
\Downarrow \\
y = ce^{-\int a(t) dt},\; c \in \mathbb{R}
$$

Per esempio, avendo $y' -\sin(t)y = 0$:
$$
y' = \sin(t)y \Rightarrow \int \frac{1}{y} dy = \int \sin(t) dt \\
\Downarrow \\
y = ce^{-\cos(t)},\; c \in \mathbb{R}
$$

### Lineari complete

Se $b(t) \neq 0$ invece, l'equazione si dice **lineare completa** e può essere risolta con:
$$
y = ce^{-\int a(t)dt} + e^{-\int a(t)dt}\int b(t) e^{\int a(t)dt} dt
$$
dove $ce^{-A(t)}$ è l'_integrale generale_ dell'**omogenea associata** (i.e. quando $b(t) = 0$), mentre l'altro membro è la **soluzione particolare** dell'equazione completa.

Per esempio, avendo $y' -\sin(t)y = \sin(t)$, l'_omogenea associata_ è $z' -\sin(t)z = 0 \Rightarrow z = ce^{-\cos(t)}$, mentre la _soluzione particolare_ è:
$$
\bar{y} = e^{-\cos(t)}\int \sin(t)e^{\cos(t)}dt = e^{-\cos(t)}\left(-e^{\cos(t)}\right) = -1
$$
di conseguenza la soluzione sarà:
$$y = z + \bar{y} = ce^{-\cos(t)} - 1$$

Se $y\left(\frac{\pi}{2}\right) = 1$ fosse _condizione iniziale_, il _problema di Cauchy_ avrebbe come soluzione $y = 2e^{-\cos(t)} - 1$.
