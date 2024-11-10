# Supervised learning

L'_apprendimento supervisionato_ avviene istruendo un modello **conoscendo gli output** voluti a priori.

Tra le due principali forme di apprendimento, ci sono:
- **Classificazione**: per predire l'appartenenza a delle categorie
- **Analisi di regressione**: per stimare una funzione numerica sui dati

Il **dataset** viene suddiviso in **training** e **test** per poter **valutare** successivamente il modello, per esempio:
```python
from sklearn.model_selection import train_test_split
from sklearn.metrics import accuracy_score

X_train, X_test, y_train, y_test = train_test_split(
  X, y, test_size=0.25, random_state=42
)
model.fit(X_train, y_train)
y_pred = model.predict(X_test)
accuracy = accuracy_score(y_true=y_test, y_pred=y_pred)
```

Oltre al campionamento casuale del _dataset_ è possibile effettuarne uno **stratified**, ovvero mantenendo le frequenze relative di ogni label del _dataset_, per evitare distorsioni durante l'apprendimento.
