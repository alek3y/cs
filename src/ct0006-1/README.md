# Basi di Dati

## Progettazione concettuale

I tipi primitivi di dati sono:
- `int`
- `real`
- `bool`
- `date`
- `string`

I tipi non primitivi, invece, sono:
- `[A: T, B: S, ...]` (i.e. record), dove `A` e `B` sono etichette mentre `T` ed `S` sono tipi
- `(A; B; ...)` (i.e. enumerazione; e.g. `(M; F)`), dove `A` e `B` sono etichette
- `seq T` (i.e. sequenza; e.g. `seq int`), dove `T` è un tipo

La **cardinalità** di un'associazione fra `X` e `Y` descrive la **molteplicità** di `X -> Y` e di `Y -> X`:

| | `[1:1]` | `[1:N]` | `[N:1]` | `[N:N]` |
|:-:|:-:|:-:|:-:|:-:|
| Ognuno di `X` con al massimo | uno di `Y` | molti di `Y` | uno di `Y` | molti di `Y` |
| Ognuno di `Y` con al massimo | uno di `X` | uno di `X` | molti di `X` | molti di `X` |

Nello schema, le **associazioni** sono rappresentate come:
- `[A] <-----> [B]`, come `[1:1]`
- `[A] <---->> [B]`, come `[1:N]`
- `[A] <<|---> [B]`, come `[N:1]` dove ognuno di `A` è associato con **al minimo** uno di `B`

Per esempio, nel caso di:
```
[Studente] <--- (HaSostenuto) -|>> [Esame]
```
ogni studente può aver sostenuto più esami, mentre ogni esame è sostenuto da uno ed un solo studente.

Nel caso in cui l'**associazione contenga proprietà**, come
```
[Utento] <|-- (HaInPrestito: Data) -|>> [Libro]
```
la relazione viene trasformata in un'ulteriore entità:
```
[Utente] <--- (HaPreso) -|>> [Prestito: Data] <|- (Riguarda) --> [Libro]
```
