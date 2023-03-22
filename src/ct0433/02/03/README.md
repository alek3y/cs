# Coordinate polari

Un'altro modo di rappresentare una curva, è attraverso le [coordinate polari](../../../ct0435/01/README.md#coordinate-polari):
$$
v = (x, y) = (\rho\cos(\theta), \rho\sin(\theta))
$$
dove $\rho \in \mathbb{R}^\geq$ è la **distanza** dall'origine **al variare** di $\theta$, cioè l'**angolo di rotazione** intorno all'origine.

Di conseguenza, una **curva** bidimensionale sarà caratterizzata dal parametro $\theta$:
$$
\begin{split}
r\colon [0, 2\pi[ &\to \mathbb{R}^2 \\
\theta &\mapsto r(\theta) = (f(\theta)\cos(\theta), f(\theta)\sin(\theta))
\end{split}
$$

Operazioni come cambiare il segno di $x$ o $y$ portano a [riflettere la curva](https://www.desmos.com/calculator/gbw6hw09qh) sugli assi.

## Vettore tangente

Il **vettore tangente** corrisponde alla derivata di $r(\theta)$, cioè:
$$
r'(\theta) = (f'(\theta)\cos(\theta) - f(\theta)\sin(\theta), f'(\theta)\sin(\theta) + f(\theta)\cos(\theta))
$$
per cui la **velocità scalare** sarà:
$$
v(\theta)^2 = \|r'(\theta)\|^2 = f'(\theta)^2 + f(\theta)^2
$$

## Tipi di curva

In questo caso, la curva si dice:
- **Regolare** se $v(\theta)^2 = f'(\theta)^2 + f(\theta)^2 \neq 0$
- **Chiusa** se $f(a) = f(b) \land a = b + 2k\pi$
- **Semplice** se $\nexists \theta_1 \in I, \theta_2 \in \mathring{I} : \theta_1 = \theta_2 + 2k\pi \land f(\theta_1) = f(\theta_2)$
