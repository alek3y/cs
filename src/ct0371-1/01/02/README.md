# Calcolo della complessità

Di un'algoritmo si possono trovare le _complessità_ $T_{\text{best}}(n)$ e $T_{\text{avg}}(n)$, ma quella di interesse è $T_{\text{worst}}(n)$.

Tra i **costrutti**, le complessità sono di:
- **Sequenza**:

	```c
	Block1  // O(f(n))
	Block2  // O(g(n))
	```

	$$T(n) = O(f(n) + g(n))$$

- **If else**:

	```c
	if Cond then  // O(f(n))
	  Block1      // O(g(n))
	else
	  Block2      // O(h(n))
	```

	$$T(n) = O(f(n) + \max(g(n), h(n)))$$

- **Cicli for**:

	```c
	for i = 1 to k
	  for j = 1 to m
	    Block  // O(f(n))
	```

	$$T(n) = O(k \cdot m \cdot f(n))$$

- **Ciclo while**:

	```c
	while Cond do  // O(f(n))
	  Block        // O(g(n))
	```

	$$T(n) = O(N(n) \cdot f(n) + N(n) \cdot g(n))$$
	dove $N(n)$ è il **massimo** numero di iterazioni per una certa $n$.

Per esempio, l'algoritmo
```c
MyAlgorithm(int n) -> int
  int a, i, j
  if n > 1 then                 // 1
    a = 0                       // 1
    for i = 1 to n              // n
      for j = 1 to n            // n
        a = (a + i) * j + a/2   // 1
    for i = 1 to 16             // 16
      a = a + MyAlgorithm(n/4)  // T(n/4)
    return a                    // 1
  else
    return 0                    // 1
```
avrà una complessità $T(n) = n^2 + 16T(\frac{n}{4}) = \Theta(n^2 \log n)$.
