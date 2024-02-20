# Tabelle hash

Le **tabelle hash** sono [dizionari](../01/README.md) che sfruttano un array `T` di $m < |U|$ celle **indicizzato dalle chiavi** tramite:
$$
h\colon U \to \{0, ..., m-1\}
$$
cioè una **funzione hash**, dove $U$ è l'**universo** delle chiavi e $K \subseteq U$ l'insieme di quelle **memorizzate** in `T`.

Nel **caso medio**, l'implementazione permette di eseguire le **operazioni** `search`, `insert` e `delete` in $\Theta(1)$.

Quando $h(k) = k$ la tabella viene detta **ad indirizzamento diretto**, che porta la complessità a $\Theta(1)$ anche nel **caso peggiore**, ma richiede che $m = |U| \gg |K|$ portando quindi ad un grande consumo di spazio.

## Collisioni

Una **collisione** si verifica quando due chiavi hanno lo **stesso hash**, che avviene [sicuramente](https://it.wikipedia.org/wiki/Principio_dei_cassetti) se $|K| > m$.
Per esempio, se $h(k) = k \bmod 3$ allora $h(1) = h(4) = h(7) = 1$.

Questo problema si può evitare tramite **concatenamento** o **indirizzamento aperto**.
