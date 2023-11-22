# Distribuzioni continue

## Uniforme

$$X \sim U(a, b)$$

- **Densità**: $f(x) = \frac{1}{b - a}$, per $x \in (a, b)$
- **Ripartizione**: $F(x) = \frac{x - a}{b - a}$
- **Media**: $E(X) = \frac{a + b}{2}$
- **Varianza**: $\mathrm{Var}(X) = \frac{(b - a)^2}{12}$

## Normale

$$X \sim N(\mu, \sigma^2)$$

- **Densità**: $f(x) = \frac{1}{\sigma\sqrt{2\pi}}e^{-\frac{1}{2}\left(\frac{x - \mu}{\sigma}\right)^2}$
- **Media**: $E(X) = \mu$
- **Varianza**: $\mathrm{Var}(X) = \sigma^2$
- **Funzione**,
	- per $P(X \leq x)$: `pnorm(q=x, mean=μ, sd=𝜎²)`
	- per $P(a \leq X \leq b)$: `pnorm(q=b, mean=μ, sd=𝜎²)-pnorm(q=a, mean=μ, sd=𝜎²)`

Modella a **curva di campana** la probabilità di fenomeni con _media_ $\mu$ e _varianza_ dei valori $\sigma^2$.
