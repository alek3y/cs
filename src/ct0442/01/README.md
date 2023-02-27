# Fondamenti

```c++
extern int a;        // Dichiarazione di un nome (definito solo in fase di linking)
int b;               // Definizione con locazione di memoria
b = 1;               // Assegnazione
int c = 3;           // Inizializzazione

double d = 3.14;
int e = d;           // Narrowing

void sum(int, int);  // Parametri formali
sum(2, 3);           // Parametri effettivi
```

## Tipo lvalue e rvalue

```c++
int *a, b;

a = 0x1;       // a è lvalue, 0x1 è rvalue
*(a + 1) = 2;  // *(a + 1) è lvalue, (a + 1) è rvalue, 2 è rvalue
a = &b;        // a è lvalue, b è lvalue, &b è rvalue

char s[10];
s = 0x1;       // Errore, perchè s equivale a &s[0] che è rvalue
*s = 'a';      // *s è lvalue, s è rvalue, 'a' è rvalue
a = &s;        // Errore, perchè s è rvalue e & necessita di un lvalue
```

Di norma, se l'operatore richiede un _rvalue_ (e.g. `*a`), è possibile usare un _lvalue_ (e.g. `a`), perchè è convertito.
