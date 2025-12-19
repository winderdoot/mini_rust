# Zaawansowane traity

## Generyczne traity i generyczne implementacje

```rs
trait<T> MyTrait {
    fn metoda(&self) -> T;
    fn wiele_metod<U>(&self) -> U; // To jest rodzina funkcji dla jednego konkretnego MyTrait::<T>
}
// Zauważmy że taki trait nie jest dyn safe, bo zwraca T

// Mamy teraz rodzinę traitów której parametryzowany trait można zaimplementować dla dowolnego typu
struct MyStruct {
    x: u32
};

impl MyTrait<u32> for MyStruct {
    fn metoda(&self) -> u32 {
        self.x
    }
}
fn funkcja<U>() -> Box<dyn Trait<U>> {
    todo!()
}

trait Trait2 {
    type T; // To jest associated type
    // Można tylko raz zdecydować dla jakiego associated typu implementujemy trait

    fn medota(&self) -> Self::T;
}

impl Trait2 for MyStruct {
    type T = String

    fn metoda(&self) -> Self::T {
        String::from("poopoo")
    }
}
```

- Parametry generyczne mogą mieć wartości domyślne
- Jak chcemy kilka constraintów to robimy ```T + U```
- Nie da się parametryzować negatywnie, tzn. że dajemy constraint ```T + !U```

## Przeładowanie operatorów

Robione za pomocą traitów

- np. ```std::ops::Add``` - ma dwie wersje dodawanie dwa razy Associated typu,
oraz wersję gdzie dodajemy associated typ i 

- Specjalny syntax:
    - ```<i32 as MyWeirdTrait>::ambiguous_function()```

### Wymaganie traitóœ

```rs
impl<T> MyFunnyTrait: Display + OtherTrait<T> {
    // W ten sposób można ograniczać to dla czego wolno zaimplementować nasz trait
    // Jest to efektywnie kompozycja interfejsów jak w C#
}
```

- Obce traity możemy implementować tylko dla własnych typów
- N. nie można zrobić dodawania dla wektorów
- Ale możemy stworzyć własny typ który oplata obcy typ i wtedy będziemy mogli dla niego zaimplementować
dodawanie (obcy trait)

### Typ nigdy (! wykrzykniczek)

- Typ ```never``` (wykrzykniczek) może być konwertowany na dowolny inny typ
- rzeczy takie jak exit, return, break, continue kiedy są wyrażeniami to zwracają ten typ

### Lajftajmy ponownie

Nie tylko referencje mogą mieć lifetime'y??

```rs
fn wrap<'a, T: X + 'a>(a: T) -> Box<dyn X + 'a> {
    Box::new(a)
}

fn get_lambda<'a>(x: &'a u32) -> Box<dyn Fn(u32) -> u32 + 'a> {
    if x.is_multiple_of(2) {
        Box::new(|y| y + *x)
    }
    else {
        Box::new(|y| y - *x)
    }
}

```

### Markery

Jak chcemy mieć strukturę parametryzowaną typem generycznym z jakiegoś powodu, ale 
nie chcemy aby miała dane tego typu to można użyć ```std::marker::PhantomData<T>``` żeby kompilator się nie pruł.
Tak samo z lifetajmami można zrobić. Wtedy kompilator będzie wymuszał lifetime dla obiektów tej struktury