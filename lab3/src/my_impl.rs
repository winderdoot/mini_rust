#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Var {
    X,
    Y,
    Z
}

impl Var {
    pub fn to_string(&self) -> String {
        match self {
            Var::X => String::from("X"),
            Var::Y => String::from("Y"),
            Var::Z => String::from("Z")
        }
    }

}

#[derive(Clone, Debug)]
pub enum Const {
    Numeric(i64),
    Named(String)
}

impl Const {
    pub fn to_string(&self) -> String {
        match self {
            Const::Numeric(x) => x.to_string(),
            Const::Named(string) => string.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum E {
    Add(Box<E>, Box<E>),
    Neg(Box<E>),
    Mul(Box<E>, Box<E>),
    Inv(Box<E>),
    Const(Const),
    Func { name: String, arg: Box<E> },
    Var(Var)
}

impl E {
    pub fn add(arg1: Box<Self>, arg2: Box<Self>) -> Box<Self> {
        Box::new(Self::Add(arg1, arg2))
    }

    pub fn neg(arg: Box<Self>) -> Box<Self> {
        Box::new(Self::Neg(arg))
    }

    pub fn mul(arg1: Box<Self>, arg2: Box<Self>) -> Box<Self> {
        Box::new(Self::Mul(arg1, arg2))
    }

    pub fn inv(arg: Box<Self>) -> Box<Self> {
        Box::new(Self::Inv(arg))
    }

    pub fn constant(arg: Const) -> Box<Self> {
        Box::new(Self::Const(arg))
    }

    pub fn func(name: String, arg: Box<Self>) -> Box<Self> {
        Box::new(Self::Func {
            name,
            arg
        })
    }

    pub fn var(arg: Var) -> Box<Self> {
        Box::new(Self::Var(arg))
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Add(e, e1) => format!("({} + {})", e.to_string(), e1.to_string()),
            Self::Neg(e) => format!("-({})", e.to_string()),
            Self::Mul(e, e1) => format!("({} * {})", e.to_string(), e1.to_string()),
            Self::Inv(e) => format!("1/({})", e.to_string()),
            Self::Const(c) => c.to_string(),
            Self::Func { name, arg } => format!("{}({})", name, arg.to_string()),
            Self::Var(var) => var.to_string(),
        }
    }

    pub fn arg_count(&self) -> u32 {
        match self {
            Self::Add(..) | Self::Mul(..) => 2,
            Self::Neg(_) | Self::Inv(_) | Self::Func {..} => 1,
            Self::Const(_) | Self::Var(_) => 0,
        }
    }

    pub fn diff(&self, by: Var) -> Box<Self> {
        match self {
            Self::Add(e, e1) => Self::add(e.diff(by), e1.diff(by)),
            Self::Neg(e) => Self::neg(e.diff(by)),
            Self::Mul(l, r) => Self::add(
                Self::mul(l.diff(by), r.clone()), 
                Self::mul(l.clone(), r.diff(by))
            ),
            Self::Inv(e) => Self::mul(
                Self::neg(Self::inv(Self::mul(e.clone(), e.clone()))),
                e.diff(by)
            ),
            Self::Const(_) => Self::constant(Const::Numeric(0)),
            Self::Func { name, arg } => Self::mul(
                Self::func(format!("{}_{}", name, by.to_string()), arg.clone()),
                arg.diff(by)
            ),
            Self::Var(var) => if by == *var { 
                Self::constant(Const::Numeric(1)) 
            } else { 
                Self::constant(Const::Numeric(0)) 
            },
        }
    }

    pub fn unpack_inv_inv(self) -> Option<Box<Self>> {
        if let Self::Inv(inner) = self {
            if let Self::Inv(e) = *inner {
                Some(e)
            } else {
                None
            }
        } else {
            None
        }
    }

    // Bez sensu trochę bo klonujemy całe wyrażenie żeby sprawdzić czy przypadkiem nie da się go uprościć.
    // Lepiej byłoby chyba żeby unpack_inv_inv() zwracało zarówno skonsumowane wyrażenie (uproszczone jeśli się da)
    // oraz informację czy zostało uproszczone
    pub fn uninv(self: Box<Self>) -> Box<Self> {
        let mut other = self.clone().unpack_inv_inv();
        let mut result = self;
        while let Option::Some(e) = other {
            other = e.clone().unpack_inv_inv();
            result = e;
        }
        result
    }

    pub fn unpack_neg_neg(self) -> Option<Box<Self>> {
        if let Self::Neg(inner) = self {
            if let Self::Neg(e) = *inner {
                Some(e)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn unneg(self: Box<Self>) -> Box<Self> {
        let mut other = self.clone().unpack_neg_neg();
        let mut result = self;
        while let Option::Some(e) = other {
            other = e.clone().unpack_neg_neg();
            result = e;
        }
        result
    }

    pub fn substitute(self, name: &str, value: Box<Self>) -> Box<Self> {
        match self {
            Self::Add(e, e1) => Self::add(
                e.substitute(name, value.clone()),
                e1.substitute(name, value)
            ),
            Self::Neg(e) => Self::neg(
                e.substitute(name, value)
            ),
            Self::Mul(e, e1) => Self::mul(
                e.substitute(name, value.clone()),
                e1.substitute(name, value)
            ),
            Self::Inv(e) => Self::inv(
                e.substitute(name, value)
            ),
            Self::Const(Const::Named(const_name)) if const_name == name => {
                value
            },
            Self::Const(c) => {
                Self::constant(c)
            }
            Self::Func { name: func_name, arg } => Self::func(
                func_name,
                arg.substitute(name, value)
            ),
            Self::Var(var) => Self::var(var),
        }
    }

}



