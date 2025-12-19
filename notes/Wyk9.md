# Multithreading

## STD

**Na tym wykładzie jest o podstawowym api które mapuje na wątki os. Jest też biblioteka rayon**

```rs
fn main() {
    let t = std::thread::spawn(|| {
        println!("Wątek!");
    });

    t1.join(); // Zwraca result
}
```

- Funkcje robocze wątków mają typ statyczny. To znaczy że nie mogą korzystać z danych które nie są statyczne albo przeniesione na własność,
bo inaczej kompilator nie wie co się dzieje

```rs

fn main() {
    let x = 1;
    std::thread::scope(|s| {
        s.spawn(|| {
            println!("x: {}", x);
        });

        s.spawn(|| {
            println!("x: {}", x);
        });

        s.spawn(|| {
            println!("x: {}", x);
        });
    }); // Wszystkie wątki są automatycznie joinowane i kompilator wie o tym
    

}
```

**W ruscie mutex to jest skrzynka do której możemy zajrzeć w synchronizowany sposób**

```rs

fn main() {
    let x = std::sync::Mutex::new(vec![1, 2, 3]);
    std::thread::scope(|s| {
        s.spawn(|| {
            x.lock().unwrap().push(1);
            println!("x: {:?}", x);
        });

        s.spawn(|| {
            println!("x: {:?}", x);
        });

        s.spawn(|| {
            println!("x: {:?}", x);
        });
    }); // Wszystkie wątki są automatycznie joinowane i kompilator wie o tym
    

}
```

- Mamy za free robust mutexy (MutexGuard i mutex poisoning)
- Zeby odblokować mutexa wystarczy zrobić drop na mutex guardzie.
- Nie da się niepoprawnie użyć mutexa
- Chyba można zrobić deadlock dropując mutex drugi raz
- Mutex działa podobnie jak refcell, ale nie wolno używać refcella zamiast mutexa

### Traity synchronizacyjne

- **Sync** - czy referencję danego typu można przenosić do wątku
- **Send** - czy dany typ można przenosić do wątku
- Te traity są implementowane dla struktur automatycznie i są zaszyte gdzieś magicznie w kompilatorze
- Np. Rc nie implementuje żadnego z nich i nie można go współdzielić

### Arc

**Atomically reference counted**
- Wielowątkowa wersja rc, obiekt wewnętrzny jest na stercie, można arca klonować nie kopiując wewnętrznej danej
- Wewnętrzna dana jest 

### Bariera

- Stara i jara

### Channel - (mq kolejki z posixa)

```std::sync::mpsc::channel```
- **Multiple Producer Single Consumer**
- są też inne channele

### Condvar
- jest i ma śmieszne api

### RwLock

## Biblioteka Rayon

```rs
use rayon::prelude::*;
let mut vec = vec![1, 2, 3, 4, 5];

/* Rayon jest broken, za free zrównolegla nam iteratory.
 * Zawiera ze sobą cały runtime z threadpoolem. (Green thready) */ 
vec.par_iter_mut().for_each(|x| *x += 1)
println!("{vec: {:?}", vec)
```

Warto użyć rayona w projekcie (gierka Grand Strategy) bo ma dużo dobrych sztuczek i zdejmuje z nas jarzmo synchronizowania rzeczy

## Biblioteka crossbeam

Bardzo przydatna, ma wciul pobrań.
Ma też wciul bardzo specyficznych i przydatnych struktur synchronizacyjnych i struktur danych.  
Jest to tego typu biblioteka, że std libka z niej potem kopiuje

- ShardedLock
- Waitgroup
- Parker

## Zmienne atomowe (stdlib)

Uważać z nimi bo reordering, memory observability are a bitch.
Są za to bardzo szybkie.