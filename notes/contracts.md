# Konktrakty

## Co to jest

Kontrakty dotyczą funkcji i typów. Ich celem jest pozwolić programistom rusta w formalny sposób zdefiniwać warunki
korzystania z ich funckji i typów, tak aby działały poprawnie. Obecnie, takie informacje zawiera się w dokumentacji i ewentualnie sprawdza assertami w runtime. Jednak żadna z tych rzeczy nie może być łatwo przetestowana ani nie podlega statycznej analizie.

## Przykład

```rs
#![feature(contracts)]
extern crate core;
use core::contracts;

impl<T: ?Sized> NonNull<T> {
    #[contracts::requires(!ptr.is_null())]
    pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
        // SAFETY: the caller must guarantee that `ptr` is non-null.
        unsafe {
            NonNull { pointer: ptr as _ }
        }
    }

    #[contracts::ensures(|new_ptr| !new_ptr.is_null())]
    pub const fn as_ptr(self) -> *mut T {
        self.pointer as *mut T
    }
}

```

## Drugi przykład

```rs
#![feature(rustc_contracts)]

#[derive(Clone, Debug, PartialEq, Eq)]
#[rustc_contracts::invariant(for safety: self.start <= self.end)]
pub(crate) struct IndexRange {
    start: usize,
    end: usize,
}

impl IndexRange {
    #[rustc_contracts::requires(for safety: start <= end)]
    pub const unsafe fn new_unchecked(start: usize, end: usize) -> Self {
        IndexRange { start, end }
    }
    
    pub const fn len(&self) -> usize {
        // (verification assumes invariant to prove underflow impossible here.)
        unsafe { unchecked_sub(self.end, self.start) }
    }
    
    #[rustc_contracts::requires(for safety: self.len() > 0)]
    #[rustc_contracts::ensures(
        for safety: |output| output == old(self.start),
        for correctness: |_output| self.len() == old(self.len()) - 1)]
    unsafe fn next_unchecked(&mut self) -> usize {
        let value = self.start;
        // verification tools are expected to perform reasoning like so:
        // operational safety: self.len() > 0 ==> self.start < self.end
        //     ==> value < self.end ==> value + 1 cannot overflow. QED.
        // invariant maintained: value < self.end ==> value + 1 <= self.end. QED.
        self.start = unsafe { unchecked_add(value, 1) }
        value
    }
}
```


## Co kontrakty wprowadziłyby do rusta

Kontrakty mają spory potencjał na zoptymalizowanie języka:
- pozwalają na wczesne sprawdzanie warunków, dzięki czemu są w stanie na etapie kompilacji wykryć niektóre rodzaje błędów (w przeciwieństwie do obecnej alternatywy jaką jest makro ```assert!```)
- mogą pozwalać kompilatorowi na optymalizacje, które byłyby niebezpieczne lub wręcz niemożliwe bez kontraktu
	- przykładem może być eliminacja bound checku w ```arr[i]``` gdy kontrakt zapewnia ```0 < i < arr.len()``` 
	- innym przykładem może być funkcja inkrementująca zmienną - jeśli zawrzemy kontrakt ograniczający od góry wartość tej zmiennej, to kompilator może być w stanie usunąć niepotrzebny overflow check
- pozwalają zapisać założenia na których opiera się unsafe funkcja za pomocą formalnego języka, a nie za pomocą komentarzy i dokumentacji, których i tak nie widać używając tej funkcji
