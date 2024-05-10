# Minimi quadrati

Il **metodo dei minimi quadrati** costruisce la **stima** $\hat{G}(x)$ del modello $G(x)$ minimizzando gli errori **residui**:
$$
Q(\beta_0, \beta_1) = \sum_{i = 1}^n e_i^2 = \sum_{i = 1}^n (y_i - \hat{y}_i)^2 = \sum_{i = 1}^n (y_i - \hat{G}(x_i))^2
$$
ovvero, trovando un [punto stazionario](../../../ct0433/03/06/03/README.md#punti-critici) con il [gradiente](../../../ct0433/03/06/README.md#vettore-gradiente), da cui si ricava che:
$$
\hat\beta_0 = \bar{y} - \hat\beta_1\bar{x}, \hspace{1em} \hat\beta_1 = \frac{s_{xy}}{s_x^2}
$$
dove $s_{xy} = \frac{1}{n-1}\sum\limits_{i = 1}^n (x_i - \bar{x})(y_i - \bar{y})$ e $s_x^2 = \frac{1}{n-1}\sum\limits_{i = 1}^n (x_i - \bar{x})^2$.

Il **coefficiente angolare** $\hat\beta_1$ è anche esprimibile con il coefficiente di [correlazione](../../../ct0111/04/README.md#correlazione) $r_{xy} = \frac{s_{xy}}{s_ys_x}$:
$$
\hat\beta_1 = r_{xy}\frac{s_y}{s_x}
$$
con cui si conferma che $X$ e $Y$ sono **correlate** positivamente o negativamente se $\hat\beta_1 \neq 0$.

## Variazione totale

La **varianza totale** dei valori che assume $\hat{G}$ è la **somma dei quadrati totale**[^1]:
$$
SQ_{\mathrm{tot}} =
SQ_{\mathrm{reg}} + SQ_{\mathrm{res}} =
\sum_{i = 1}^n (\hat{y}_i - \bar{y})^2 + \sum_{i = 1}^n (y_i - \hat{y}_i)^2 =
\sum_{i = 1}^n (y_i - \bar{y})^2 = (n-1)s_y^2
$$

Se ne può ricavare il **coefficiente di determinazione**, che descrive **quanto si adatta** il modello ai dati:
$$
R^2 = \frac{SQ_{\mathrm{reg}}}{SQ_{\mathrm{tot}}} = 1 - \frac{SQ_{\mathrm{res}}}{SQ_{\mathrm{tot}}} \in [0, 1]
$$
che nella _regressione lineare_ vale $R^2 = r^2_{xy}$, e quindi $1$ quando $X$ e $Y$ sono correlate linearmente.

[^1]: [Total sum of squares](https://en.wikipedia.org/wiki/Total_sum_of_squares) = [explained sum of squares](https://en.wikipedia.org/wiki/Explained_sum_of_squares) + [residual sum of squares](https://en.wikipedia.org/wiki/Residual_sum_of_squares)
