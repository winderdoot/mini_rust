# Moduły i biblioteki

Dzieci modułów widzą co jest w rodzicach domyślnie,
ale rodzice nie widzą co się dzieje w dzieciach.

self, super, crate

Skomplikowane to, w praktyce jakoś się nauczy.

## Testy

```rs
#[cfg(test)]
mod tests {
    #[test]
    pub fn tescior1() {
        todo!()
    }
}
```

```rs
[#should_panic]
```
informuje kompilator że test powinien panikować

Jak chcemy przekazać flagi jakieś do samego testu a nie do ```cargo test``` to piszemy:
```cargo test -- --flags```

Konwencja jest taka że unit testy są w tym samym pliku co funkcjonalność  
Testy integracyjne są w katalogu tests, nie trzeba już pisać ```[#cfg(test)]```

## Dokumentacja

```cargo doc --open``` - otwiera nam stronkę z automatycznie wygenerowaną dokumentacją

Komentarze w dokumentacji są kompilowane i testowane 😭😭😭😭😭
Oraz np. jak mamy unsafe funkcje to kompilator będzie się pruł że w dokumentacji nie jest napisane jak używać tej funkcji bezpiecznie

## Workspace

Nie wiem o co z tym chodzi, ale 

```toml
[workspace]
resolver = 2
members = ...
```

## Crates

crates.io - stronka z krejtami gdzie ludzie publikują swoje biblioteki
cargo.lock - plik który wymusza że wersje bibliotek muszą być dokładnie takie jak u nas

