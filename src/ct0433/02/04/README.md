# Lunghezza

Per calcolare la **lunghezza** di un **arco di curva** basta [approssimare il sostegno](https://www.desmos.com/calculator/psxir9t2cm) con dei segmenti di rette:
$$
l(p) = \sum_{i=1}^n \|r(t_i) - r(t_{i-1})\| \leq l(\gamma)
$$
dove $p$ è la curva **poligonale** che approssima $\gamma$.

Se **al variare** della _poligonale_ $p$, $\sup_p(l(p)) < +\infty$ allora $l(\gamma) = \sup_p(l(p))$ per cui $\gamma$ è detta **rettificabile**.
In quel caso:
$$
l(\gamma) = \int_a^b \|r'(t)\| dt
$$

Per esempio, con $r(t) = (\cos(t), \sin(t), t)$ e $t \in [0, 2\pi]$:
$$
l(\gamma) = \int_0^{2\pi} \sqrt{(-\sin(t))^2 + \cos(t)^2 + 1^2} dt = 2\sqrt{2}\pi
$$
