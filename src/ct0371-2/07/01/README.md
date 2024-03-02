# Taglio delle aste

Nel problema del **taglio delle aste** si vuole cercare il **massimo guadagno** $r_n$ ottenibile dalla vendita di pezzi ricavati dal taglio di un'asta lunga $n$, i cui prezzi $p_i$ dipendono dalla lunghezza $i$ venduta.

Per esempio se $n = 7$ e
$$
\Set{(i, p_i) | i \leq n} = \{(1, 1), (2, 5), (3, 8), (4, 9), (5, 10), (6, 17), (7, 17)\}
$$
conviene tagliare l'asta in pezzi da $2, 2, 3$ o $6, 1$ invece che $5, 2$ perchè si guadagna $18$ invece che $15$.

## Ricorsione

Un'asta lunga $n$ può essere **tagliata o meno** in ogni posizione $1 \leq i \leq n-1$, totalizzando
$$
\underbrace{2 \cdot 2 \cdots 2}_{n-1} = 2^{n-1}
$$
tagli e quindi portando la **complessità** a $\Theta(2^n)$.

Il **ricavo** per l'asta $r_n$ sarà quindi definito ricorsivamente come:
$$
r_n = \max(p_n, r_1 + r_{n-1}, r_2 + r_{n-2}, ..., r_{n-1} + r_1)
$$
dove $p_n$ è il ricavo dell'**asta senza tagli** e $r_i + r_{n-i}$ è il ricavo dato dal **taglio su $i$**.

Per il problema si dice che valga la proprietà di **sottostruttura ottima** perchè la **soluzione ottima** cercata, in questo caso $r_n$, è esprimibile da combinazioni di _soluzione ottime_ di sottoproblemi.
