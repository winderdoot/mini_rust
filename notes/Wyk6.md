# Iteratory i programowanie funckyjne

Nie ma stanu, nie ma zmiennych. Wszystko jest wyrażeniem i zwraca jakąś wartość. Nie ma iteracji i wszystko się robi rekurencją.

## Funkcje anonimowe (closure)

```rs
let f = |x: u32| {
    let y = x + 10;
    y * 10
};

let g = |x: u32| x * x;

let mut vec = Vec::<u32>::new();
let f = move |x: u32| {
    vec.push(x);
    x * 10
}

/* Dzięki move możemy zwracać teraz taką funkcję, bo vec został do niej przeniesiony.
 * To znaczy, że vec jest tak jakby pole struktury f. 
 * Jest taki crate closure, który pozwala definiować które zmienne są captured przez
 * referencję, mutowalną referencję i prez przeniesienie. Domyślnie nie ma tego w języku niestety. 
 */

```

```rs
fn funkcja(f: fn(u32) -> u32) {
    println!("{}", f(10));
}

// Pure funkcje są akceptowane
fn funckja1(f: impl Fn(u32) -> u32) {
    println!("{}", f(10));
    println!("{}", f(20));
}

// Funkcje które się mutują są przyjmowane, ale musi dać się je wywołać wiele razy
fn funckja1(f: impl FnMut(u32) -> u32) {
    println!("{}", f(10));
    println!("{}", f(20));
}

// Funkcja konsumuje się przy pierwszym wywołaniu
fn funkcja2(f: impl FnOnce(u32) -> u32) {

}

fn f(x: 32) -> u32 {
    x * x
}

fn main() {
    let y = 100;
    funkcja(|x| x * y); // Nie działa
    /* Pełny typ funkcji anonimowej zawiera informację o przechwytywanych typach i sposobie w jaki są przechwytywane */
    funckja1(|x| x * y); // To jest ok
}
```

```rs
fn           operator()()          // Nie może nic przechowywać, używa tylko własnych argumentów
Fn           operator()(&self)
FnMut        operator()(&mut self)
FnOnce       operator()(self)
```

```rs
// Te dwa zapisy są równoważne
fn funkcja<T: Display>(cos: T) {
    println!("{}", cos);
}

fn funkcja1(cos: impl Display) {
    println!("{}", cos);
}
```

## Iteratory (jak w linq yeee)

Jest coś takiego jak associated types. Gdy się implementuje Iterator trait dla struktury, to trzeba określić raz jakiego typu jest item zwracany przez iterator. Jest to lepsze czasami od parametryzowania typem, bo chcemy tylko dla jednego typu to zrobić.

```rs

fn main() {
    let mut vec = vec![1, 2, 1, 3, 7, 8, 5, 6, 9, 0, 2, 3];

    let sum = vec.iter().any(|&x| x > 10);
    let sum = vec.iter().all(|x| *x > 10);

    /* trait Iterator ma fajne funkcyjnki jak w pajtnie 🐍🐍.
     * map, filter, take, zip, enumerate, find, chain */
}
```

- ```collect()``` to jest taki ```IEnumerable.ToArray()```

**Jak przeiterować po elementach gdy mamy wektor wektorów**
- ```iter().flatten()```

- ```flat_map()``` -> mapuję kolekcję funkcją wejściową, ale jak funkcja zwraca inną kolekcję, to flat_map nam to rozpłaszcza
- ```crate itertools``` dostarcza rozszerzenie iteratora, ze wszystkimi cool ahhh funkcyjkami 🍸🍸😎😎

**kompilator ma informacje o wszystkich typach i i kolejnych operacjach jakie są na iteratorach wykonywane**:
- dzięki temu dobrze te operacje optymalizuje, dlatego warto ich używać

*typy takie jak bool, option, result itd też mają takie śmieszne funkcje
