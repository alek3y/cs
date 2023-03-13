# Tipi di curve

Una curva parametrica $r(t)$ con $t \in I = [a, b]$, può essere:
- **Chiusa**, se la fine della curva **si riconnette** all'inizio, cioè:
$$
r(a) = r(b)
$$

- **Semplice**, se **non si interseca mai** (escludendo gli estremi per quando è _chiusa_), cioè:
$$
r(t_1) = r(t_2) \Leftrightarrow t_1 = t_2
$$
	dove $t_1 \in I$ e $t_2 \in \mathring{I} = I \setminus \{a, b\}$.

	La circonferenza (e.g. $x^2 + y^2 = 1$) con $I \neq [0, 2\pi]$ **non è semplice**, perchè $r(t) = r(t + 2\pi k)$.

- **Regolare**, se:
$$
r \in C^1 \land r'(t) \neq \vec{0},\, \forall t \in I
$$

	Da cui si può ricavare il **versore tangente**:
$$
T(t) = \frac{r'(t)}{\|r'(t)\|}
$$

- **Regolare a tratti**, se $r \in C^0$ ed esistono finiti sottointervalli di $I$ su ognuno dei quali $r$ è **regolare**

Per esempio, se $r(t) = (t^3 + t^4, \cos(\frac{\pi t}{2}))$ con $t \in [-2, 2]$:
- **Non è regolare** anche se $r \in C^1$, perchè quando $t = 0$, $r'(t) = (0, -\frac{\pi}{2}\sin(0)) = (0, 0)$
- **Non è chiusa**, perchè $r(-2) \neq r(2)$
- **Non è semplice**, perchè $\cos(\frac{\pi t}{2})$ e $t^3 + t^4$ non hanno la stessa asse di simmetria
