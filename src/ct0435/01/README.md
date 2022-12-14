# Numeri complessi

I numeri complessi appartengono all'insieme $\mathbb{C}$ e sono in forma $a + bi$, dove $i = \sqrt{-1}$, o in specifico:
$$i^2 = -1$$

## Operazioni

- **Somma**: $(a + bi) + (c + di) = (a + c) + (b + d)i$
- **Prodotto**: $(a + bi) \cdot (c + di) = ac + adi + bci + bdi^2 = ac - bd + (ad + cb)i$
- **Coniugato**: $\overline z = a - bi$
- **Modulo**: $|z| = \sqrt{a^2 + b^2}$
- **Inverso**: $z^{-1} = \frac{\overline z}{|z|^2} = \frac{a - bi}{a^2 + b^2}$
- **Divisione**: $\frac{w}{z} = w \cdot z^{-1} = w \cdot \frac{\overline z}{|z|^2}$

### Esempi

1. $$z = 7+i, \space w = 4 - i$$

	$$z \cdot w = (7 + i) \cdot (4 - i) = 28 - 7i + 4i - i^2 = 28 - 3i + 1 = 29 - 3i$$

2. $$z = 3 + 4i, \space w = 2 - 4i$$

	$$z + w = (3 + 4i) + (2 - 4i) = 5 + 0i = 5$$

3. $$z \cdot \overline z = (a + bi)(a - bi) = a^2 - abi + abi - b^2i^2 = a^2 - b^2i^2 = a^2 + b^2$$

4. $$z = 2 + i$$

	$$z^{-1} = \frac{\overline{2 + i}}{2^2 + 1^2} = \frac{2 - i}{5}$$

# Coordinate polari

È possibile rappresentare un certo numero complesso $z = a + bi$ in un piano chiamato **piano complesso**, dove nelle ascisse verrà messa la parte _reale_, mentre nelle ordinate la parte _immaginaria_.

![Rappresentazione di un numero nel piano complesso](assets/01.png)

Dalla rappresentazione si possono ricavare le proprietà:

1. Il modulo $|z|$, è linea obliqua che si forma dall'origine al numero $z$
2. Conoscendo $\alpha$ e un raggio $r = |z|$, è possibile ricavare il numero complesso
3. $a = r\cos(\alpha)$ e $b = r\sin(\alpha)$, dove $r$ è il raggio

Per cui, se ne ricava la forma delle **coordinate polari**:
$$z = a + bi = r\cos(\alpha) + r\sin(\alpha)i = |z|(\cos(\alpha) + \sin(\alpha)i)$$

### Esempi

1. $$z = 2-3i$$

	$$r = |z| = \sqrt{2^2 + (-3)^2} = \sqrt{13}$$
	$$z = |z| \cdot \frac{z}{|z|} = \sqrt{13} \cdot \frac{2-3i}{\sqrt{13}} = \sqrt{13} \left(\frac{2}{\sqrt{13}} - \frac{3}{\sqrt{13}}i\right)$$
	$$a = |z| \cdot \cos(\alpha) = \sqrt{13} \cdot \frac{2}{\sqrt{13}} = 2$$
	$$b = |z| \cdot \sin(\alpha) = \sqrt{13} \cdot \frac{-3}{\sqrt{13}} = -3$$

## Operazioni

- **Moltiplicazione**:

	$$z = |z|(\cos(\alpha) + \sin(\alpha)i)$$
	$$w = |w|(\cos(\beta) + \sin(\beta)i)$$
	$$z \cdot w = |z \cdot w|(\cos(\alpha + \beta) + \sin(\alpha + \beta)i)$$

## Forma esponenziale

Attraverso la formula di eulero,
$$e^{i\alpha} = \cos(\alpha) + \sin(\alpha)i$$
è possibile accorciare la forma polare con $z \neq 0$ in:
$$z = |z|e^{i\alpha}$$

Questo semplificherà anche l'operazione di **moltiplicazione** che, con $z = re^{i\alpha}$ e $w = se^{i\beta}$, diventerà $z \cdot w = (r \cdot s)e^{i(\alpha + \beta)}$.
