# Algebra booleana

Una funzione logica può essere espressa come:
- **Tabella di verità**
- **Equazione**

Le operazioni logiche fondamentali sono quindi:
- **AND** ($\cdot$)
- **OR** ($+$)
- **NOT** ($\neg$)

## Proprietà

- **Identità**: $A + 0 = A$, $A \cdot 1 = A$
- **Nullo**: $A + 1 = 1$, $A \cdot 0 = 0$
- **Idempotente**: $A + A = A$, $A \cdot A = A$
- **Inverso**: $A + \neg A = 1$, $A \cdot \neg A = 0$
- **Commutativa**: $A + B = B + A$, $A \cdot B = B \cdot A$
- **Associativa**: $A + (B + C) = A + B + C$, $A \cdot (B \cdot C) = A \cdot B \cdot C$
- **Distributiva**: $A + (B \cdot C) = (A \cdot B) + (A \cdot C)$, $A \cdot (B + C) = (A + B) \cdot (A + C)$
- **De Morgan**: $\neg (A + B) = \neg A \cdot \neg B$, $\neg (A \cdot B) = \neg A + \neg B$

## Operatori universali

Dal **NAND** e dal **NOR** è possibile ricavarsi tutte le porte logiche, e sono quindi chiamati **operatori universali**.

Per esempio,
$$\neg A = \neg A + 0 = \neg (A \cdot \neg 0) = \neg (A \cdot 1)$$
che sarebbe il $\mathrm{nand}(A, 1)$.
