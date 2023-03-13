# Multiple issue

Le alternative per aumentare il parallelismo della _pipeline_ (e quindi diminuire il CPI) sono:
- **Aumentare la profondità** della _pipeline_ aumentando il numero di _unità fondamentali_
- **Replicare i componenti** per poter processare più istruzioni alla volta

L'ultima alternativa è detta **multiple issue** e può essere **static** o **dynamic**.

## Static multiple issue

Il **compilatore** raggruppa le istruzioni in **issue packet** da inserire contemporaneamente nella _pipeline_.

Anche se evita alla _CPU_ di effettuare _branch prediction_ e affida l'ottimizzazione del codice al compilatore (dandogli molto più tempo di quanto se ne possa permettere la _pipeline_) ha comunque più svantaggi:
- Il compilatore non può _speculare i branch_ senza eseguire il codice
- Il bytecode diventerà dipendente dalla profondità della _pipeline_ oltre che dall'ISA

## Dynamic multiple issue

Le CPU che adottano questo metodo, chiamate **superscalari**, scelgono dinamicamente **quante** e **quali** istruzioni immettere nella _pipeline_ **ad ogni ciclo** di clock.

Ogni gruppo di istruzioni può essere inserito in due modi:
- **In-order**: viene solamente scelto il numero di istruzioni consecutive da inviare
- **Out-of-order**: vengono scelte quante e quali istruzioni inviare (facendo attenzione alle dipendenze)
