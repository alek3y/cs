# Distribuzioni continue

## Uniforme

$$X \sim \mathrm{U}(a, b)$$

- **Densità**: $f(x) = \frac{1}{b - a}$, per $x \in (a, b)$
- **Ripartizione**: $F(x) = \frac{x - a}{b - a}$
- **Media**: $E(X) = \frac{a + b}{2}$
- **Varianza**: $\mathrm{Var}(X) = \frac{(b - a)^2}{12}$

## Normale

$$X \sim \mathrm{N}(\mu, \sigma^2)$$

- **Densità**: $f(x) = \frac{1}{\sigma\sqrt{2\pi}}e^{-\frac{1}{2}\left(\frac{x - \mu}{\sigma}\right)^2}$
- **Ripartizione**: $F(x) = \Phi(\frac{x - \mu}{\sigma})$
- **Media**: $E(X) = \mu$
- **Varianza**: $\mathrm{Var}(X) = \sigma^2$
- **Funzione**,
	- per $P(X \leq x)$: `pnorm(q=x, mean=μ, sd=𝜎)`
	- per $P(a \leq X \leq b)$: `pnorm(q=b, mean=μ, sd=𝜎)-pnorm(q=a, mean=μ, sd=𝜎)`

Modella a **curva di campana** la probabilità di fenomeni con _media_ $\mu$ e _varianza_ dei valori $\sigma^2$.

Attraverso la **standardizzazione** di una variabile $X$ si ottiene una **normale standard**:
$$
Z = \frac{X - \mu}{\sigma} \sim \mathrm{N}(0, 1)
$$
in cui $E(Z) = 0$ e $\mathrm{Var}(Z) = 1$, da cui si trova la **funzione di ripartizione** particolare di $Z$:
$$
\Phi(z) = \int_{-\infty}^z \frac{1}{\sqrt{2\pi}}e^{-\frac{x^2}{2}} dx
$$

Per esempio, la probabilità che il diametro $X$ di un tubo prodotto da una macchina che ha media $\mu = 3 \text{mm}$ e varianza $\sigma^2 = 0.09^2 \text{mm}^2$ sia fuori specifica richiesta, cioè che non sia di diametro $2.94 \pm 0.18$ è:
$$
\begin{split}
P(|X - 2.94| > 0.18) &= P(X < 2.94 - 0.18) + P(X > 2.94 + 0.18) = \\
&= P(X < 2.76) + 1 - P(X \leq 3.12) = \\
&= \Phi\left(\frac{2.76 - 3}{0.09}\right) + 1 - \Phi\left(\frac{3.12 - 3}{0.09}\right) \approx \\
&\approx 0.00391 + 1 - 0.90824 = 0.09567
\end{split}
$$

In questo caso la distribuzione può **approssimare** la [binomiale](../01/README.md#binomiale), infatti:
$$
\mathrm{Bin}(n, p) \approx \mathrm{N}(np, np(1 - p))
$$
usato in pratica quando $np(1 - p) \geq 10$.

Per esempio, la probabilità che si danneggino da $800$ a $850$ file su $2400$ ognuno con probabilità $0.35$ è:
$$
P(800 \leq X \leq 850) = \sum_{k = 800}^{850} {2400 \choose k} 0.35^k (1 - 0.35)^{2400 - k} = 0.632893
$$
ovvero `pbinom(850, 2400, 0.35) - pbinom(799, 2400, 0.35)`, oppure con la _normale_, dopo una [correzione di continuità](https://it.wikipedia.org/wiki/Correzione_di_continuit%C3%A0), dove $\mu = 2400 \cdot 0.35 = 840$ e $\sigma = \sqrt{2400 \cdot 0.35 (1 - 0.35)} \approx 23.3666$:
$$
\begin{split}
P(800 \leq X \leq 850) &= P(799.5 < X < 850.5) \approx \\
&\approx \Phi\left(\frac{850.5 - 840}{23.3666}\right) - \Phi\left(\frac{799.5 - 840}{23.3666}\right) = \\
&= \Phi(0.4493586) - \Phi(-1.73324) = \\
&= \Phi(0.4493586) - (1 - \Phi(1.73324)) = \\
&= 0.631887
\end{split}
$$
