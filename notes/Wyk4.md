# Biblioteka standardowa i obsługa błędów

## C++ ssie

## Vector

- metoda ```push```
- indeksowanie ```my_vec[]``` robi panic kiedy błędny access
- indeksowanie ```my_vec.get()``` (lub ```my_vec.get_mut()```) zwraca ```Option```
- kiedy w pętli for coś przekażemy, to zostaje to przeniesione. Jeśli tego nie chcemy, 
trzeba zrobić iterator lub (np. mutowalną) referencję
- ```Vec<i32>``` jako typ, ale ```Vec::<i32>::new()``` aby stworzyć nowy.

## String
- metoda ```chars()``` żeby dostać iterator po znakch (nie bajtach)
- stringi są w utf-8, więc jak normalnie indeksujemy (po bajtach) możemy trafić w środek znaku i kicha

## Obsługa błędów

Typ algebraiczny Result:
```rs
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

Funkcje które mogą mieć błąd zwracają result i definiują sobie często jakiś enum z rodzajem błędu.
Na resulcie możemy zrobić:
- match expression
- ```unwrap()``` żeby spanikować kiedy jest Err
- ```expect(&str)``` żeby spanikować i dać wiadomość kiedy jest Err
- magiczny operator pytajnik pytajniczek ```?```:
```rs
let a = do_thing()?;
/* jest równoważne */
let a = do_thing();
match a {
    Ok(_),
    e @ Err(_) => return e;
}
```
- W haskelu ```do``` jest podobno podobne do wielu pytajniczków z rumsta
- if let albo coś podobnego
- różne funkcyjne funkcycjki jakieś unwrap_or itp

## Fajne rzeczy
* Moduł **mem** zawiera różne przydatne funkcje do podmiany, przypisań itp. które normalnie byłyby niewykonalne przez borrow checker
    - std::mem::replace
    - mem::drop
    - mem::take
    - mem::discriminant
* Moduł **net** - TCP i networking
* Moduł **time** do odliczania czasu
* Moduł **path**
* Moduł **fs**
* Typ OsString - stringi tak jak są
* hint::black_box - można przekazać coś do tej funkcji i wtedy kompilator nigdy tego nie wyoptymalizuje
* ```NonZero<T>``` - gwarantuje że wartość nie jest zerem
* Jeśli chcemy używać overflowa to można użyc wrapping funkcji
