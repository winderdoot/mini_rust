# OOP i Dynamic Dispatch

```rs
struct A;
struct B;

impl Funny for A;
impl Funny for B;

/* Rodzina funkcji */
fn funkcja(x: &impl Funny) {

}

/* Dokładnie jedna funkcja przyjmująca referencję na dowolny obiekt Funny
 * którego typ jest ustalany w runtime */
funkcja_dyn(x: &dyn Funny) {

}

/* Uwaga: jeżeli mamy dynamic dispatch to trzeba pakować obiekty w boxy. Bo w przeciwnym wypadku,
 * nie znamy wielkości obiektu który implementuje dany trait. Ich rozmiar musi być znany w czasie
 * kompilacji dlatego się je boxuje. */
fn main() {
    let a = A;
    let b = B;
    let v: Vec<Box<dyn Funny>> = vec![Box::new(a), Box::new(b)]

}

```

- Trait Sized jest implementowany automatycznie, chyba że się nie da dla danego structa (bo np. ma gołego stringa bez referencji w środku)
- Kiedy używamy typów generycznych to zawsze implicite ma on constraint size
- Można to obejść robiąc ?Sized w constrainach. Wtedy można przekazywać typy które nie implementują Sized, ale można też takie co implementują. Ten syntax odnosi się tylko to traitu Sized.

----

- Nie z każdego obiektu można robić obiekty dynamiczne. Jest coś takiego jak dyn safety. Są pewne ograniczenia na to jakie traity mogą być dyn.
- W szczególności trait nie może używać Self. Nie mogą mieć też metod generycznych

- Jest coś takiego jak Any trait ale nie jest on zbyt potężny