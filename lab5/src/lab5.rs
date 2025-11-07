pub use std::collections::HashMap;
pub type Context = HashMap<&'static str, u64>;

pub trait Expr {
    fn exec_expr(&mut self, context: &Context) -> u64;
}

pub trait Stmt {
    fn exec_stmt(&mut self, context: &Context);
}


/* PRINT */
pub struct Print<T: Expr> {
    inner: T
}

pub fn print<T: Expr>(inner: T) -> Print<T> {
    Print { inner }
}

impl<T: Expr> Stmt for Print<T> {
    fn exec_stmt(&mut self, context: &Context) {
        println!("{}", self.inner.exec_expr(context));
    }
}

/* NOTHING */
pub struct Nothing {}

pub fn nothing() -> Nothing {
    Nothing {}
}

impl Stmt for Nothing {
    fn exec_stmt(&mut self, _context: &Context) { }
}

/* SEQ  */
pub struct Seq<T: Stmt, U: Stmt> {
    first: T,
    second: U
}

pub fn seq<T: Stmt, U: Stmt>(first: T, second: U) -> Seq<T, U> {
    Seq { first, second }
}

impl<T: Stmt, U: Stmt> Stmt for Seq<T, U> {
    fn exec_stmt(&mut self, context: &Context) {
        self.first.exec_stmt(context);
        self.second.exec_stmt(context);
    }
}

impl<T: Stmt> Seq<T, Nothing> {
    pub fn shorten_1(self) -> T {
        self.first
    }
} 

impl<T: Stmt> Seq<Nothing, T> {
    pub fn shorten_2(self) -> T {
        self.second
    }
}

impl Seq<Nothing, Nothing> {
    pub fn collapse(self) -> Nothing {
        Nothing {}
    }
}

impl Expr for u64 {
    fn exec_expr(&mut self, _context: &Context) -> u64 {
        *self
    }
}

pub struct When<T: Expr, U: Expr, V: Expr> {
    condition: T,
    if_true: U,
    if_false: V
}

pub fn when<T: Expr, U: Expr, V: Expr>(condition: T, if_true: U, if_false: V) -> When<T, U, V> {
    When { condition, if_true, if_false }
}

impl<T: Expr, U: Expr, V: Expr> Expr for When<T, U, V> {
    fn exec_expr(&mut self, context: &Context) -> u64 {
        match self.condition.exec_expr(context) {
            0 => self.if_false.exec_expr(context),
            1.. => self.if_true.exec_expr(context)
        }
    }
} 

pub struct Repeat<const N: u32, T: Stmt> {
    inner: T
}

pub fn repeat<const N: u32, T: Stmt>(inner: T) -> Repeat<N, T> {
    Repeat::<N, T>{ inner }
}

impl<const N: u32, T: Stmt> Stmt for Repeat<N, T> {
    fn exec_stmt(&mut self, context: &Context) {
        for _ in 1..=N {
            self.inner.exec_stmt(context);
        }
    }
}

pub struct Constant {
    name: &'static str
}

pub fn constant(name: &'static str) -> Constant {
    Constant { name }
}

impl Expr for Constant {
    fn exec_expr(&mut self, context: &Context) -> u64 {
        *context.get(self.name).unwrap()
    }
}

pub struct ReadFrom<'a> {
    value: &'a u64
}

pub fn read_from<'a>(value: &'a u64) -> ReadFrom<'a> {
    ReadFrom { value }
}

impl<'a> Expr for ReadFrom<'a> {
    fn exec_expr(&mut self, _context: &Context) -> u64 {
        *self.value
    }
}

pub struct SaveIn<'a, T: Expr> {
    inner: T,
    destination: &'a mut u64
}

pub fn save_in<'a, T: Expr>(destination: &'a mut u64, inner: T) -> SaveIn<'a, T> {
    SaveIn { inner, destination }
}

impl<'a, T: Expr> Expr for SaveIn<'a, T> {
    fn exec_expr(&mut self, context: &Context) -> u64 {
        *self.destination = self.inner.exec_expr(context);
        *self.destination
    }
}

pub struct Volatile<'a, T: Expr> {
    destination: &'a mut u64,
    name: &'static str,
    inner: T
}

pub fn volatile<'a, T: Expr>(destination: &'a mut u64, name: &'static str, inner: T) -> Volatile<'a, T> {
    Volatile { destination, name, inner }
}

impl<'a, T: Expr> Expr for Volatile<'a, T> {
    fn exec_expr(&mut self, context: &Context) -> u64 {
        let mut ctx = context.clone();
        // context.entry(self.name).and_modify(|val| *val = *self.destination).or_insert(*self.destination);
        ctx.insert(self.name, *self.destination);
        *self.destination = self.inner.exec_expr(&ctx);
        *self.destination
    }
}