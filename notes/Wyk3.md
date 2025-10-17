# Enumy i switch

```rs
enum Kolor {
    Czerwony(u32),
    Zielony {
        ile_moniz: f32,
        jakie_moniz: String
    },
    Niebieski
}

fn main() {
    let a: Kolor = Kolor::Czerwony(12);
    let b: Kolor = Kolor::Niebieski;
    match a {
        Kolor::Czerwony(x) => println!("Czerwony komuch weee! {}", x),
        Kolor::Zielony {ile_moniz, jakie_moniz} => println!("Moooniiizzz"),
        Kolor::Niebieski => println!("Niebieskie buuuu!!"), 
    }
}
```

**Enumy są typami algebraicznymi**
Trzeba rozumieć teorię kategorii i teorię typów matematyczną żeby pisać w ruscie.  
W ruście są sumy typów i iloczyny typów.  
- ```T x U``` to iloczyn, czyli np. struct w postaci ```(int, float, char)```
- ```T + U```to suma typów, czyli obiekt jest jednego typu lub jest drugiego typu ```union number { int num; float num }```

**Enum jest sumą wielu typów**, to znaczy że zmienna typu enum, może być dowolną ze struktur składającą się na enum.
- Bardzo przydatne.
- Np. adresy IP można w ten sposób napisać
- Jak chcemy metody dla enumów, to trzeba w enumie zrobić po prostu wartość typu struktury dla której zrobimy te metody


**Enum Option<T>**:
- generyczny template
- ma taką postać:
```rs
enum Option<T> {
    None,
    Some(T)
}
```
- Kompilator wymusza, żeby sprawdzić co się znajduje w opcji

```rs
enum Animal {
    Cat(f32),
    Dog {
        name: String
    },
    Giraffe {
        neck_len: f32
    }

}

fn f(a: Animal) {
    if let Animal::Cat(f) = a {
        println!("Mamy kotka!");
    }
    let Animal::Dog {name} = a else {
        return;
    };
}
```

## Match i wzorce

- poza ```if let``` mamy też ```while let```
- wzorce można nie tylko w matchu dać. Można też w ```let```
- jeśli nie pasuje wzorzec to zawsze można potem dać ```else```
- we wzorcach można dodawać ```mut``
- przypisując w let można zdereferencować jakby, że z referencji pobrać tylko inta
- w parametrach funkcji też są wzorce, ale muszą być nieodrzucalne
- @ 