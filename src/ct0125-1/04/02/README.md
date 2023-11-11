# Segmentazione

Come la [paginazione](../01/README.md), ma ogni **entry** contiene anche la **lunghezza** del segmento e dei **bit di protezione**.
Inoltre la **condivisione** porterà a meno _overhead_, dato che in genere ci sono meno _entry_ da condividere.

## Protezione

Una tecnica per **proteggere** i segmenti in memoria è quella di associare al processo e ai segmenti una **chiave di protezione** modificabili solo dal _kernel_ che, se combacia, ne permetterà l'accesso.

L'alternativa più comune è l'uso dei **bit di protezione**, che specificano quali tra le operazioni il processo ha il permesso di effettuare, come i bit _read_, _write_ e _execute_.

Questi danno un maggior controllo sulla memoria, infatti durante un riferimento avvenire le **eccezioni**:
- **Missing segment fault**, se il segmento non è caricato in memoria
- **Overflow segment fault**, se il _page offset_ supera la lunghezza del segmento
- **Segment protection exception**, se l'operazione effettuata non è consentita

## Ibrido con paginazione

Per sfruttare i vantaggi della **paginazione** e della **segmentazione**, si possono usare segmenti che occupano una o più pagine, in questo modo è possibile caricare solamente **parte del segmento** in memoria.

In questo caso si dovranno salvare sia il _page number_ che il _segment number_ assieme al _page offset_.

Usando il _segment number_ si trova nella _segment table_ l'indirizzo della _page table_, che verrà poi indicizzata con il _page number_ restituendo infine il _page frame_.

Più processi **condividono** memoria se una delle righe della loro _segment table_ punta alla stessa _page table_.
