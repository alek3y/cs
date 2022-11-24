# Algebra modulare

## Gruppi

L'argomento è stato già trattato _parzialmente_ nella parte di [Calcolo 1](../../ct0432/02/01/README.md).

Una **monoide** $(A, \ast)$ combina un insieme $A$ e un'operazione qualsiasi $\ast\colon A \times A \to A$ se l'operazione $\ast$ rispetta la proprietà:
- **Associativa**
- **Elemento neutro**

Un **gruppo**, invece, è una **monoide** che rispetta la proprietà dell'**inverso**.
Si dice **abeliano**, inoltre, se rispetta anche la proprietà **commutativa**.

## Divisione

Dalla **divisione euclidea** si ha che:
$$\forall a, b \in \mathbb{Z} : b \neq 0, \exists! q, r \in \mathbb{Z} : a = bq + r$$
infatti $a:b = q$ con resto $r$.

## Modulo

Il modulo restringe l'insieme su un insieme $A_n$ (e.g. $\mathbb{N}_{10} = 0, 1, ..., 9$) e si esprime come:
$$a\, \mathrm{mod}\, n = \mathrm{mod}_n(a)$$

Le proprietà sono:
- $(a + b) \bmod n = (a \bmod n + b \bmod n) \bmod n$
- $(a \cdot b) \bmod n = (a \bmod n \cdot a \bmod n) \bmod n$

### Congruenze

Sia $R \subseteq \mathbb{Z}^2$ una relazione. Se
$$aRb \Leftrightarrow a \bmod n = b \bmod n$$
allora, $R$ si dice **congruenza modulo** $n$.

In pratica, la relazione esprime l'uguaglianza tra due numeri $a$ e $b$ sotto modulo $n$, per esempio  $12 \equiv_{10} 2$ perchè il resto di $12:10$ è $2$.

L'[**insieme quoziente**](../03/02/README.md#classi-di-equivalenza) $\mathbb{Z}/\equiv_n$ di questa relazione, è caratterizzato dal fatto che divide $\mathbb{Z}$ in $n$ classi di equivalenza.
Per esempio, $\mathbb{Z}/\equiv_1 = \{\mathbb{Z}\}$.
