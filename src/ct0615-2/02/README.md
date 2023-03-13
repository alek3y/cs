# Cache

Oltre alla [SRAM](../../ct0615-1/05/README.md#sram) (usata per la **cache**), e [DRAM](../../ct0615-1/05/README.md#dram) (usata per la **RAM**), che include il tipo _DDR SDRAM_ per trasferire sia sul _rising-_ che _falling-edge_, esiste anche la **flash NAND** (usata per le **SSD**), o _EEPROM_ (_Electrically Erasable Programmable ROM_), che con il **wear leveling** ne distribuiscono la scrittura per evitare il rapido consumo.

Più la memoria è veloce più costa, di conseguenza va sfruttato un principio di **gerarchia**, mettendo quelle più veloci con quantità minore più **vicino alla CPU**.
