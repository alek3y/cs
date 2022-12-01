# CPU

La più semplice CPU realizzabile è quella a **ciclo singolo**, cioè che ogni istruzione richiede un **singolo ciclo di clock** per effettuare il _Fetch-Decode-Execute_.

L'accesso ai **registri** e alla **memoria** verrà effettuato da:
- `M[x]`, cioè la _word_ all'indirizzo `x`
- `R[y]`, cioè il registro `y`

## Istruzioni

Le istruzioni principali sono:
- _Memory-reference_: `lw`, `sw`
- _Arithmetic-logic_: `add`, `sub`, `and`, `or`, `slt`
- _Control-flow_: `beq`, `j`

Decodificando i vari [tipi di istruzioni](../03/README.md#formati-delle-istruzioni), otteniamo:
- `rd`, `rt`, `rs`, cioè i registri
- `Imm16`, che corrisponde al valore _immediato_ dell'istruzione

### Implementazione

Di conseguenza il pseudocodice delle operazioni che riuscirà a fare il processore è:
- `add`: `R[rd] = R[rs] + R[rt]; PC += 4;`
- `sub`: `R[rd] = R[rs] - R[rt]; PC += 4;`
- `lw`: `R[rt] = M[R[rs] + sign_ext(Imm16)]; PC += 4;`
- `sw`: `M[R[rs] + sign_ext(Imm16)] = R[rt]; PC += 4;`
- `beq`: `PC += 4; if (R[rs] == R[rt]) {PC += sign_ext(Imm16) << 2;}`

dove `sign_ext(Imm16)` corrisponde all'_estensione in CA2_ di `Imm16` (e.g. $-2_{10} = 1110_2 = 1111 1110_2$).
