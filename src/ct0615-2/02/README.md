# Cache

Oltre alla [SRAM](../../ct0615-1/05/README.md#sram) (usata per la **cache**), e [DRAM](../../ct0615-1/05/README.md#dram) (usata per la **RAM**), che include il tipo _DDR SDRAM_ per trasferire sia sul _rising-_ che _falling-edge_, esiste anche la **flash NAND** (usata per le **SSD**), o _EEPROM_ (_Electrically Erasable Programmable ROM_), che con il **wear leveling** ne distribuiscono la scrittura per evitare il rapido consumo.

Più la memoria è veloce più costa, di conseguenza va sfruttato un principio di **gerarchia**, mettendo quelle più veloci con quantità minore più **vicino alla CPU**.

Per fare ciò si sfruttano i **principi di località**:
- **Località temporale**, per cui si tende ad accedere allo stesso dato più volte in brevi periodi di tempo
- **Località spaziale**, per cui si tende ad accedere ad altri dati che sono vicini a quello corrente

Attraverso la **gerarchia** si hanno degli **hit**, per cui il dato è stato **trovato**, e dei **miss**, per cui **va cercato** nella cache successiva, sulla memoria più vicina (e.g. _L1_).

Quindi, l'**hit time** è il tempo di accesso alla cache più vicina, mentre **miss penalty** è il tempo richiesto per caricare un dato da una memoria più distante alla cache e per passarlo alla CPU.

## Mapping diretto

Per generare l'indirizzo sulla cache esiste il **direct-mapping**, che usa parte dei bit dell'indirizzo originale.

Ogni riga della cache conterrà quindi:
- Un bit **valid**, per identificare se la riga è vuota oppure no
- Un **tag**, per identificare i bit alti dell'indirizzo della _RAM_
- Un **block** (i.e. multiple celle di memoria), per memorizzare i valori

Per trovare l'indirizzo $i_C$ su una cache $h \times w$ ($h$ righe da $w$ byte, cioè $h \cdot w$ bytes) con $w, h \in \{2^n : n \in \mathbb{N}\}$, va manipolato l'indirizzo di memoria $i_M$ per ottenere tre parti consecutive:
- **Offset**: identifica quale dei $w$ sotto-blocchi contiene il dato di $i_M$
$$
o = i_M\, \&\, (w-1)
$$

- **Indice**: identifica l'indirizzo sulla cache dell'intero blocco (per tutti i $w$ sotto-blocchi)
$$
\begin{split}
i_C &= (i_M / w) \bmod h = \\
&= (i_M / w)\, \&\, (h-1) = \\
&= (i_M \gg \log_2(w))\, \&\, (h-1)
\end{split}
$$

- **Tag**: contiene i rimanenti bit alti di $i_M$ per evitare [collisioni](https://it.wikipedia.org/wiki/Principio_dei_cassetti) degli $i_M$ che producono gli stessi $i_C$
$$
t = i_M / (h \cdot w) = i_M \gg \log_2(h \cdot w)
$$

Per esempio, avendo $i_M = 10101101_2$ su una cache alta $8$ e larga $2$:
$$
\begin{split}
o &= 10101101_2\, \&\, (2-1) = 1_2 \\
i_C &= (10101101_2 / 2)\, \&\, (8-1) = 1010110_2\, \&\, 111_2 = 110_2 \\
t &= 10101101_2 / (8 \cdot 2) = 1010_2
\end{split}
$$
