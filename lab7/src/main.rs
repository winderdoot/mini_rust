use std::{borrow::Cow, cell::{Cell, LazyCell, OnceCell, RefCell}, collections::VecDeque, ops::{Deref, DerefMut}, path::PathBuf, rc::{Rc, Weak}};

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


    let n = 10;
    let mut node = Vertex::cycle(n);

    for _ in 0..n {
        println!("node: {}", node.borrow().data);
        let next = node.borrow().all_neighbours().first().unwrap().upgrade().unwrap();
        node = next;
    }

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

/* Żeby się clippy nie pruł  */
impl Default for CachedFile {
    fn default() -> Self {
        Self::new()
    }
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

// 7

pub struct Vertex {
    pub out_edges_owned: Vec<Rc<RefCell<Vertex>>>,
    pub out_edges: Vec<Weak<RefCell<Vertex>>>,
    pub data: i32,

}

impl Default for Vertex {
    fn default() -> Self {
        Self::new()
    }
}

impl Vertex {
    pub fn new() -> Self {
        Self {
            data: 0,
            out_edges: Vec::new(),
            out_edges_owned: Vec::new()
        }
    }

    pub fn create_neighbour(&mut self) -> Rc<RefCell<Vertex>> {
        let neighbour = Rc::new(RefCell::new(Vertex::new()));
        self.out_edges_owned.push(neighbour.clone());
        neighbour
    }

    pub fn link_to(&mut self, other: Rc<RefCell<Vertex>>) {
        self.out_edges.push(Rc::downgrade(&other))
    }

    pub fn all_neighbours(&self) -> Vec<Weak<RefCell<Vertex>>> {
        self.out_edges
            .iter()
            .chain(
                self.out_edges_owned
                    .iter()
                    .map(Rc::downgrade)
                    .collect::<Vec<Weak<RefCell<Vertex>>>>()
                    .iter()
            )
            .cloned()
            .collect::<Vec<Weak<RefCell<Vertex>>>>()
    }

    pub fn cycle(n: usize) -> Rc<RefCell<Vertex>> {
        let first = Rc::new(RefCell::new(Self::new()));
        let mut last = first.clone();
        let mut neighbour;

        for i in 1..n {
            neighbour = last.borrow_mut().create_neighbour();
            neighbour.borrow_mut().data = i as i32;
            last = neighbour;
        }
        last.borrow_mut().link_to(first.clone());

        first
    }
}



