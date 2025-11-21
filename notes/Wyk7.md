# Cow, Cell, Deref, Rc

## Box (unique_ptr)

- Trait Deref/DerefMut pozwala nam używać operatora gwiazdka
- Deref ma associated type Target, który jest typem zwracanym przez dereferencję

## Rc (Rerefernce counted - shared_ptr)

- ```Rc::new(), Rc::clone()```

Po co? Weź pan spróbuj zrobić listę wiązaną na samych lifetime'ach. No nie da się.
Box lub Rc pozwalają sensownie to robić.

- Ograniczenie: Rc traktuje swoje dane jako niemutowalne 🤡🤡, żeby nie łamać zasady że nie ma kilku mutowalnych referencji

## Cell
- przechowuje miejse w pamięci, którego nie możemy stricte modyfikować, ale możemy podmieniać to co tam jest
- odpowiednik ```mutable``` w c++

**Jeszcze kilka wonky obiektów i będziemy w stanie pisać normalnie kod**

## RefCell
- można otrzymywać mutowalny dostęp do tego co jest pod pointerem
- mega sus 🤨🤨
- w kompilacji nie ma sprawdzanej poprawności pożyczania mutowalnego i niemutowalnego
- te same błędy są rzucane w rantajmie

## Memory leaks

**Bezpieczny rust nie gwarantuje że nie będzie w programie wycieków pamięci**
- Można zrobić cykl w grafie Rc i wtedy mamy problemito

## Weak (weak_ptr)

- Wskazuje na coś, ale nie przedłuża okresu życia obiektu
- Pozwala radzic sobie z cyklami w odniesieniach Rc
- Weak nie potrafi odczytać pamięci pod ponterem, trzeba
na nim zrobić upgrade() które zwraca opcję z Rc

## LazyCell

- przy pierwszym dostępie jest wołana funkcja inicjalizująca (i tylko wtedy)

## OnceCell

- Przy każdym dostępie jest wołana funkcja inicjalizująca

## Cow

LazyLock not taking closure





