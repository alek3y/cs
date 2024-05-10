# Valutare il modello

Per valutare il _modello lineare_, è necessario ridefinirlo aggiungendo gli **errori normali** $\epsilon_i \sim \mathrm{N}(0, \sigma^2)$:
$$
Y_i = \beta_0 + \beta_1 x_i + \epsilon_i
$$
che vengono poi **stimati** dagli $e_i$ dei [minimi quadrati](../01/README.md).
In questo modo $\beta_1$ è **non distorto** e **normale**.

## Intervalli di confidenza

Dato che anche $\hat\beta_1$ è _normale_, si possono trovare gli **intervalli di confidenza** con la [statistica T](../../03/02/README.md):
$$
\hat\beta_1 \pm t_{\alpha/2}\widehat{\mathrm{SE}}(\hat\beta_1)
$$
che avrà $n-2$ **gradi di libertà**.

## Verifica delle ipotesi

Il **sistema d'ipotesi** con $H_0: \beta_1 = \beta_1^0$ può essere valutato con il [test T](../../04/02/README.md) da $n-2$ **gradi di libertà**:
$$
T = \frac{\hat\beta_1 - \beta_1^0}{\widehat{\mathrm{SE}}(\hat\beta_1)}
$$

In particolare, si può dire che $X$ **prevede** $Y$ linearmente, se $H_0: \beta_1 = 0$ vs. $H_A: \beta_1 \neq 0$ è [significativo](../../04/03/README.md).

## Analisi dei residui

Anche se $R^2 \approx 1$, non è detto che i **residui stimati** $e_i$ rispettino le **assunzioni** applicate agli $\epsilon_i$, ovvero:
- **Normalità**, nel caso i quantili seguano un modello lineare su un [Q-Q plot](https://en.wikipedia.org/wiki/Q%E2%80%93Q_plot)
- **Varianza costante**, nel caso gli $e_i$ rimangano ugualmente concentrati nel [grafico a dispersione](../../01/03/README.md#grafici-a-dispersione) sulle $X$

## Modello log-log

Se $X$ e $Y$ non sono linearmente correlati o i residui non seguono le assunzioni, il modello usato è **inadatto**.
Per correggerlo si può tentare di trasformare le variabili su un **modello log-log**:
$$
\log Y = \gamma_0 + \gamma_1\log x + \epsilon
$$

## Previsioni

Mediante il _modello di regressione_ è possibile **predire** $G(x)$, per un $x_\ast$:
$$
\hat{y}_\ast = \hat\beta_0 + \hat\beta_1 x_\ast
$$
che avrà però una concreta **incertezza**, descritta dalla varianza:
$$
\mathrm{Var}(\hat{y}_\ast) = \sigma^2\left(1 + \frac{1}{n} + \frac{(x_\ast - \bar{x})^2}{(n-1)s_x^2}\right)
$$
con cui si trova l'**intervallo di previsione**, con _distribuzione T_ da $n-2$ **gradi di libertà**:
$$
\hat{y}_\ast \pm t_{\alpha/2}\sqrt{\widehat{\mathrm{Var}}(\hat{y}_\ast)}
$$
