# Makra

## Makra proceduralne
- funkcja rustowa zamieniająca strumień tokenów na inny strumień tokenów.
- Forma metaprogramowania
- Operuje na AST a nie na tokenach tak jak w C

## Materiały

- W rust booku jest mało
- Jest coś takiego jak A little book of rust macros
- W Rust language reference też coś jest, ale bardzo referencyjnie

## Praktyka

```rs
macro_rules! makro {
    ($x: ident) => {
        struct $x;
    }
}

```

**Można obejrzeć rozwinięte makra przez cargo expand**