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

struct RGB {
    r: u8,
    g: u8,
    b: u8
}
enum Color {
    RGB(RGB),
    HSV(u8, u8, u8),
    HLS(u8, u8, u8),
    XYZ { n: u32, b: f32, c: String }
}

impl RGB {
    fn f(&self) {
        
    }
    fn g(self: Box<Self>) {

    }
}

fn color_f(color: Color) {
    match color {
        Color::RGB(rgb @ RGB { r, g, b}) => {
            rgb.f();
            println!("{r}{g}{b}");
        },
        Color::HSV(h, _, _) | Color::HLS(h, _, _) => todo!(),
        Color::XYZ { n, .. } => todo!()
    }
}

fn example() {
    let mut x = Box::new(10); // Teraz mamy obiekt na stercie (jak new w C#)
    // x += 10; Nielegalne
    *x += 10; // Trait Deref
    /* Box się przydaje, np. żeby przyjąć obiekt który implementuje dany trait, ale nie wiadomo jaki obiekt.
     * Dzięki boxowi, ma on stały rozmiar (jak pointer)
     * Box to jest w zasadzie unique_ptr z C++
     */
    let a = Box::new(RGB {
        r: 1,
        g: 2,
        b: 3
    });

    a.f(); // Automatyczna dereferencja

    a.g();
}