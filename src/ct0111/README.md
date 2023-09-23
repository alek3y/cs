# Probabilità e statistica

La probabilità si basa sul [calcolo combinatorio](../ct0434/10/README.md), cioè sul come **contare raggruppamenti** di oggetti.

Di base, il **principio fondamentale generalizzato del calcolo combinatorio** permette di contare le **configurazioni** di $r$ oggetti, ognuno dei quali può assumere $m_i$ stati:
$$
\prod_{i = 1}^r m_i = m_1 \cdot m_2 \cdot ... \cdot m_r
$$
dove $m_1$ sono gli stati del primo oggetto, $m_2$ del secondo, etc.

In particolare, considerata un'urna contente delle palline **distinguibili**, si possono pescare:
- $r$ palline su $n$ **reinserendole**, cioè $\underbrace{n \cdot n \cdot ... \cdot n}_r = n^r = D'_{n,r}$[^1]
- $r$ palline dalle rimanenti su $n$ **senza reinserirle**, cioè $\underbrace{n \cdot (n-1) \cdot ... \cdot (n-r+1)}_r = \frac{n!}{(n-r)!} = D_{n,r}$[^2]
- **tutte** le $n$ palline, cioè $\underbrace{n \cdot (n-1) \cdot ... \cdot 1}_n = n! = P_n$[^3]
- $r$ palline su $n$ **senza reinserirle** il cui **ordine non distingue** le configurazioni, cioè $\frac{D_{n,r}}{P_n} = C_{n,r}$[^4]

Per esempio, date le lettere $\mathrm{PEPPER}$ si vogliono trovare il numero di anagrammi dove l'ordine delle $\mathrm{P}$ e delle $\mathrm{E}$ non differenzia le configurazioni (e.g. $\mathrm{P_1E_1P_2P_3E_2R} = \mathrm{P_2E_2P_3P_1E_1R}$):
$$
\frac{D_{6,6}}{P_3 \cdot P_2} = \frac{6!}{3! \cdot 2!} = \frac{720}{12} = 60
$$
infatti ogni disposizione contiene $3! \cdot 2!$ configurazioni uguali, scambiando le $\mathrm{P}$ e le $\mathrm{E}$.

[^1]: [Disposizioni con ripetizioni](../ct0434/10/README.md#multiinsieme)

[^2]: [Disposizioni senza ripetizioni](../ct0434/10/README.md#disposizioni)

[^3]: [Permutazioni](../ct0434/10/README.md#permutazioni)

[^4]: [Combinazioni](../ct0434/10/README.md#combinazioni)
