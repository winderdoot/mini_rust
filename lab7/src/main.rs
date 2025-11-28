use std::{borrow::Cow, cell::{Cell, LazyCell, OnceCell}, collections::VecDeque, ops::{Deref, DerefMut}, path::{PathBuf}, rc::Rc};

fn main() {
    let hans = AustroHungarianGreeter { counter: Cell::new(0) };
    for _ in 0..10 {
        println!("{}", hans.greet());
    }

    let heap = HeapOrStack::Heap(Box::new(3));
    let stack = HeapOrStack::Stack(5);
    let vals = [heap, stack];
    vals.iter().for_each(|v| println!("{}", **v));

    let vecdeq = VecDeque::from(vec![0, 2, 4, 3, 5]);
    let rotated = canon_head(&vecdeq);
    match rotated {
        Some(vd) => println!("vecdeq: {:#?}", vd),
        None => println!("Vecdeq empty"),
    }

    let f1 = SharedFile::new(PathBuf::from("Cargo.toml"));
    let f2 = f1.clone();
    let f3 = f2.clone();
    let fds = [f1.clone(), f1, f2.clone(), f3, f2];
    fds.iter().for_each(|f| println!("SharedFile:\n{}", f.get()));

    let c1 = CachedFile::new();
    println!("Cached:\n{}", c1.get(&PathBuf::from("data.txt")));


}   


// 1
struct AustroHungarianGreeter {
    counter: Cell<usize>
}

impl AustroHungarianGreeter {

    fn greet(&self) -> &'static str {
        let greetings: [&'static str; 3] = ["Es lebe der Kaiser!", "Möge uns der Kaiser schützen!", "Éljen Ferenc József császár!"];
        let ret = greetings[self.counter.get() % 3];
        self.counter.update(|c| c + 1);
        ret
    }
}

// 2
impl Drop for AustroHungarianGreeter {
    fn drop(&mut self) {
        println!("Ich habe {} mal gegrüßt", self.counter.get());
    }
}

// 3
pub enum HeapOrStack<T> {
    Stack(T),
    Heap(Box<T>)
}

impl<T> Deref for HeapOrStack<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            HeapOrStack::Stack(inner) => inner,
            HeapOrStack::Heap(inner) => inner,
        }
    }
}

impl<T> DerefMut for HeapOrStack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            HeapOrStack::Stack(inner) => inner,
            HeapOrStack::Heap(inner) => inner,
        }
    }
}

// 4

pub fn canon_head<'a>(xs: &'a VecDeque<i32>) -> Option<Cow<'a, VecDeque<i32>>> {
    let mut i = 0;
    let mut cow = Cow::Borrowed(xs);

    while i < xs.len() {
        match cow.front() {
            Some(v) => {
                if v % 2 == 1 {
                    return Some(cow);
                }
                cow.to_mut().rotate_left(1);
                i += 1;
            },
            None => return Some(cow),
        }
    }

    None
}

// 5

pub struct CachedFile {
    cache: OnceCell<String>
}

impl CachedFile {
    pub fn new() -> Self {
        Self { cache: OnceCell::new() }
    }

    fn read_content(path: &PathBuf) -> String {
        std::fs::read_to_string(path)
            .unwrap_or_else(|err| err.to_string())
    }

    pub fn get(&self, path: &PathBuf) -> &str {
        self.cache
            .get_or_init(|| Self::read_content(path))
    }

    pub fn try_get(&self) -> Option<&str> {
        self.cache
            .get()
            .map(|s| s.as_str())
    }
}


// 6

#[derive(Clone)]
pub struct SharedFile {
    file: Rc<LazyCell<String, Box<dyn FnOnce() -> String>>>
}

impl SharedFile {
    fn read_content(path: PathBuf) -> String {
        std::fs::read_to_string(path)
            .unwrap_or_else(|err| err.to_string())
    }

    pub fn new(path: PathBuf) -> Self {
        Self { file: Rc::new(LazyCell::new(Box::new(move || Self::read_content(path)))) }
    }

    pub fn get(&self) -> &str {
        &self.file
    }
}





