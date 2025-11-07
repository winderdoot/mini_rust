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
    fn exec_stmt(&mut self, context: &Context) { }
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
    fn exec_expr(&mut self, context: &Context) -> u64 {
        *self
    }
}

pub struct When<T: Expr, U: Expr, V: Expr> {
    condition: T,
    if_true: U,
    if_false: V
}

impl<T: Expr, U: Expr, V: Expr> Expr for When<T, U, V> {
    fn exec_expr(&mut self, context: &Context) -> u64 {
        match self.condition.exec_expr(context) {
            0 => self.if_false.exec_expr(context),
            1.. => self.if_true.exec_expr(context)
        }
    }
} 

