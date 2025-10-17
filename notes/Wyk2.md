# Obiekty, Pożyczanie i Semantyka Przeniesienia

- primitives nie podlegają temu, są kopiowane po ludzki
- Structy domyślnie są przenoszone do funkcji
- Można zwrócić z funkcji obiekt żeby był przeniesiony przy przypisaniu
- Są referencje mutowalne i niemutowalne

| Ref. Rust | Ptr. C      | Ref. C++    | Co              |
|-----------|-------------|-------------|-----------------|
| &a        | &a          | a           | Stała           |
| &mut a    | &a          | a           | Zmienna         |
| &i32      | const int * | const int & | Stał typ        |
| &mut i32  | int *       | int &       | Zmien typ       |
| NIE       | TAK         | NIE         | NULLABLE        |
| NIE       | TAK         | TAK         | Inv reference   |
| TAK       | TAK         | NIE         | Czy to obiekt   |
| *a        | *a          | a           | Jak się odwołać |

----
Jak widać, referencje w ruscie są jak wskaźniki z C, ale zabezpieczone.  
Są też w ruscie wskaźniki, które działają dokładnie jak wskaźniki w C.  

Pożyczanie:
Do obiektu mogą istnieć:
- Nieskończona liczba niemutowalnych referencji
- ALBO
- Tylko jedna mutowalna referencja

---

## Typ Slice (typ plasterkowy)

- Typ referencyjny wskazujący na jakiś fragment bufora (np. stringa)
- Tylko czyta pamięć i nie usuwa jej po zniknięciu plasterka
- Dla stringa typ plasterkowy nazywa się ```str```
- Można przekazać &String do funkcji oczekuącej &str, typy mogą być automatycznie na slice konwertowane
- Biorąc slice ze stringa podajemy liczbę bajtów niestety, nie znaków (Stringi są utf-8 w ruscie)

## Struktury

- Tworząc strukturę danego typu można w to miejsce przenieść istniejący obiekt
- Składnia ```..struct_object``` żeby przenieść resztę pól struktury
- Są struktury krotkowe
- Metody implementujemy poza definicją struktury