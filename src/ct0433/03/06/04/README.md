# Derivate parziali seconde

Le **derivate parziali seconde** di una funzione $f\colon \mathbb{R}^n \to \mathbb{R}$ si ottengono derivando le _prime_ rispetto alle $n$ variabili, di conseguenza in totale ci saranno $n^2$ _derivate parziali seconde_.

Per esempio, se $f(x, y) = x^2 + \cos(x)\sin(y) + 3y$ le derivate parziali sono:
$$
\begin{gathered}
\frac{\partial f}{\partial x}(x, y) = 2x - \sin(x)\sin(y) \\[0.8em]
\frac{\partial f}{\partial y}(x, y) = \cos(x)\cos(y) + 3
\end{gathered} \Rightarrow
\begin{alignat}{2}
\frac{\partial}{\partial x} \frac{\partial f}{\partial x}(x, y) &= \frac{\partial^2 f}{\partial x^2}(x, y) &&= 2 - \cos(x)\sin(y) \\[0.8em]
\frac{\partial}{\partial y} \frac{\partial f}{\partial x}(x, y) &= \frac{\partial^2 f}{\partial y \partial x}(x, y) &&= -\sin(x)\cos(y) \\[0.8em]
\frac{\partial}{\partial x} \frac{\partial f}{\partial y}(x, y) &= \frac{\partial^2 f}{\partial x \partial y}(x, y) &&= -\sin(x)\cos(y) \\[0.8em]
\frac{\partial}{\partial y} \frac{\partial f}{\partial y}(x, y) &= \frac{\partial^2 f}{\partial y^2}(x, y) &&= -\cos(x)\sin(y)
\end{alignat}
$$
da cui si nota che le **derivate parziali miste** (e.g. $\frac{\partial^2 f}{\partial y \partial x}$) sono uguali (nella maggior parte dei casi).

## Matrice Hessiana

Come per il [_gradiente_](../README.md#vettore-gradiente), le _derivate parziali seconde_ vengono raggruppate nella **matrice Hessiana**:
$$
H_f = \begin{pmatrix}
\frac{\partial^2 f}{\partial x_1^2} & \frac{\partial^2 f}{\partial x_1 \partial x_2} & \cdots & \frac{\partial^2 f}{\partial x_1 \partial x_n} \\[0.5em]
\frac{\partial^2 f}{\partial x_2 \partial x_1} & \frac{\partial^2 f}{\partial x_2^2} & \cdots & \frac{\partial^2 f}{\partial x_2 \partial x_n} \\[0.5em]
\vdots & \vdots & \ddots & \vdots \\[0.5em]
\frac{\partial^2 f}{\partial x_n \partial x_1} & \frac{\partial^2 f}{\partial x_n \partial x_2} & \cdots & \frac{\partial^2 f}{\partial x_n^2}
\end{pmatrix} \Leftrightarrow
(H_f)_{ij} = \frac{\partial^2 f}{\partial x_i \partial x_j}
$$
che descrive la _curvatura_ di $f\colon \mathbb{R}^n \to \mathbb{R}$ in modo analogo alle [derivate ordinarie seconde](../../../../ct0432/05/07/README.md#convessità-e-concavità).

Inoltre, se tutte le $(H_f)_{ij} \in C^0$ per $i,j = 1, ..., n$, la matrice $H_f$ è **simmetrica** sulla _diagonale principale_.

Per esempio, se $f(x, y) = x^2 + y^3 + sin(xy)$:
$$
H_f(x, y) = \begin{pmatrix}
\frac{\partial^2 f}{\partial x^2}(x, y) & \frac{\partial^2 f}{\partial x \partial y}(x, y) \\[0.5em]
\frac{\partial^2 f}{\partial y \partial x}(x, y) & \frac{\partial^2 f}{\partial y^2}(x, y)
\end{pmatrix} = \begin{pmatrix}
2 - y^2\sin(xy) & \cos(xy) - xy\sin(xy) \\
\cos(xy) - xy\sin(xy) & 6y - x^2\sin(xy)
\end{pmatrix}
$$
che se valutata in $(\pi, 1)$ diventa:
$$
H_f(\pi, 1) = \begin{pmatrix}
2 - \sin(\pi) & \cos(\pi) - \pi\sin(\pi) \\
\cos(\pi) - \pi\sin(\pi) & 2 - \pi^2\sin(\pi)
\end{pmatrix} = \begin{pmatrix}
2 & -1 \\
-1 & 6
\end{pmatrix}
$$

## Formula di Taylor

Se in $\mathbb{R}$ la funzione $f \in C^2$ è approssimata dal [polinomio di Taylor](../../../../ct0432/07/01/README.md) di _ordine_ $2$ (ponendo $x = x_0 + h$) come
$$
f(x_0 + h) = f(x_0) + f'(x_0)h + \frac{1}{2}f''(x_0)h^2 + o(h^2)
$$
in $\mathbb{R}^n$ la funzione può essere approssimata da:
$$
f(\vec{x}_0 + \vec{h}) = f(\vec{x}_0) + \nabla f^T(\vec{x}_0)\vec{h} + \frac{1}{2}\vec{h}^T H_f(\vec{x}_0) \vec{h} + o(\|\vec{h}\|^2)
$$
anch'essa in ordine $2$ perchè $H_f$ fa da derivata seconda.

Espandendo la formula in $\mathbb{R}^2$, assomiglia al polinomio originale:
$$
\begin{split}
f(x_0 + h, y_0 + k) =
&\phantom{.} f(x_0, y_0) + \left(\frac{\partial f}{\partial x}(x_0, y_0)h +
\frac{\partial f}{\partial x}(x_0, y_0)k\right) + \phantom{.} \\
&+ \frac{1}{2}\left(\frac{\partial^2 f}{\partial x^2}(x_0, y_0)h^2 +
2 \cdot \frac{\partial^2 f}{\partial y \partial x}(x_0, y_0)hk +
\frac{\partial^2 f}{\partial y^2}(x_0, y_0)k^2\right) + o(\|(h, k)\|^2)
\end{split}
$$

## Caratterizzare i punti critici

Se la funzione $f\colon \mathbb{R}^n \to \mathbb{R} \in C^2$ ha un _punto critico_ in $\vec{x}_0$, allora $\nabla f(\vec{x}_0) = \vec{0}$ e quindi nella _formula di Taylor_ l'unico **termine rilevante** che rimane per approssimare $f$ è:
$$
\vec{h}^T H_f(\vec{x}_0)\vec{h}
$$

Per esempio, se $f(x, y) = x^2 + \frac{3}{2}(y - 1)^2 + 1$ e $\vec{x}_0 = (0, 1)$:
$$
\nabla f(0, 1) = \begin{pmatrix}
2 \cdot 0 \\ 3 \cdot 1 - 3
\end{pmatrix} = \vec{0} \\[0.4em]
\Updownarrow \\
H_f(x, y) = \begin{pmatrix}
2 & 0 \\ 0 & 3
\end{pmatrix} \Rightarrow
\vec{h}^T H_f(0, 1) \vec{h} =
\begin{pmatrix}x & y\end{pmatrix}
\begin{pmatrix}
2 & 0 \\ 0 & 3
\end{pmatrix}
\begin{pmatrix}x \\ y\end{pmatrix} =
2x^2 + 3y^2
$$
per cui $f$ verso $\vec{x}_0$ tenderà ad un [paraboloide ellittico](../../01/README.md#paraboloide-ellittico) con il vertice sull'origine, da cui $\vec{x}_0$ è **punto di minimo**.
