# Pipeline

Ogni ciclo della CPU [a singolo ciclo](../../ct0615-1/07/README.md#circuito-a-singolo-ciclo) può essere suddiviso in _unità funzionali_, cioè delle operazioni fondamentali che possono essere parallelizzate per ottenere la **CPU pipeline**:
- **IF** (_Instruction Fetch_): **lettura** dell'istruzione e **incremento** PC
- **ID** (_Instruction Decode_): **scomposizione** istruzione (i 32 bit) e **lettura** registri
- **EX** (_Execute_): **esecuzione** ALU (per `add`/`lw`/`sw`) e **calcolo** indirizzo per `beq`
- **MEM** (_Memory Access_): **lettura**/**scrittura** in memoria (per `lw`/`sw`)
- **WB** (_Write Back_): **scrittura** sul registro (per `add`/`lw`)

L'implementazione avviene eseguendo ogni _unità funzionale_ **parallelamente** alle istruzioni successive.

![Diagramma dei tempi ridotti dalla struttura pipeline](assets/01.png)
