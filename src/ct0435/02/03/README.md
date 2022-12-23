# Proiezione

Per ricavare la proiezione di un vettore $A$ su $B$, basta **ridimensionare** $B$, in modo che la sua **norma sia uguale alla proiezione** di $A$ su $B$.

![Proiezione sul vettore](assets/01.png)

Chiamiamo, quindi, $c$ il valore scalare che ridimensionerà $B$ come proiezione di $A$ su $B$:
$$||cB|| = c||B|| = ||A|| \cdot \cos(\alpha)$$

Sappiamo che
$$A \cdot B = ||A|| \cdot \cos(\alpha) \cdot ||B|| \Rightarrow \frac{A \cdot B}{||B||} = ||A|| \cdot \cos(\alpha)$$
di conseguenza, è possibile sostituire nel valore di $c||B||$:
$$c||B|| = \frac{A \cdot B}{||B||} \Rightarrow c = \frac{A \cdot B}{||B||^2} = \frac{A \cdot B}{B \cdot B}$$

Un'alternativa è trovare il vettore che va da $cB$ a $A$, cioè $\overrightarrow{(cB)A} = A - cB$.
Questo lo si può ottenere sfruttando la proprietà della perpendicolarità:
$$(A - cB) \cdot B = 0 \Rightarrow A \cdot B - cB \cdot B = 0 \Rightarrow c = \frac{A \cdot B}{B \cdot B}$$
