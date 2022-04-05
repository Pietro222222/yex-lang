use std::{
    cmp::Ordering,
    mem,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub, Deref},
};

pub mod fun;
pub mod instance;
pub mod list;
pub mod symbol;
pub mod table;
pub mod yextype;
pub mod str;
pub mod mutable;
use crate::{error::InterpretResult, gc::GcRef, raise};

use fun::Fn;
use instance::Instance;
use list::List;
use symbol::Symbol;
use yextype::YexType;
use mutable::Mutable;

use self::{table::Table};

pub fn nil() -> Value {
    Value::Nil
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

/// Immediate values that can be consumed
#[derive(Debug, PartialEq)]
pub enum Value {
    /// float-precision numbers
    Num(f64),
    /// Strings
    Str(GcRef<String>),
    /// erlang-like atoms
    Sym(Symbol),
    /// Booleans
    Bool(bool),
    /// Fnctions
    Fn(GcRef<Fn>),
    /// Tables
    Table(Table),
    /// Yex lists
    List(List),
    /// Yex user-defined types
    Type(GcRef<YexType>),
    ///Mutable Value
    Mutable(GcRef<Mutable>),
    /// Yex instances
    Instance(GcRef<Instance>),
    /// null
    Nil,
}

impl Clone for Value {
    fn clone(&self) -> Self {
        use Value::*;

        match self {
            List(xs) => List(xs.clone()),
            Str(str) => Str(GcRef::clone(str)),
            Fn(f) => Fn(GcRef::clone(f)),
            Bool(b) => Bool(*b),
            Num(n) => Num(*n),
            Sym(s) => Sym(*s),
            Type(t) => Type(t.clone()),
            Instance(i) => Instance(i.clone()),
            Table(t) => Table(t.clone()),
            Mutable(t) => Mutable(GcRef::clone(t)),
            Nil => Nil,
        }
    }
}

impl Value {
    /// checks if the constant is `nil`
    pub fn is_nil(&self) -> bool {
        self == &Self::Nil
    }

    /// Returns the size of `self`
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        match self {
            Value::List(xs) => xs.len(),
            Value::Num(_) => mem::size_of::<f64>(),
            Value::Sym(_) => mem::size_of::<Symbol>(),
            Value::Str(s) => s.len(),
            Value::Fn(f) => mem::size_of_val(&f),
            Value::Bool(_) => mem::size_of::<bool>(),
            Value::Type(t) => mem::size_of_val(&t),
            Value::Instance(i) => mem::size_of_val(&i),
            Value::Table(t) => mem::size_of_val(&t),
            Value::Mutable(_) => mem::size_of::<Mutable>(),
            Value::Nil => 4,
        }
    }

    /// Compares the left and the right value
    pub fn ord_cmp(&self, rhs: &Self) -> InterpretResult<Ordering> {
        let (left, right) = match (self, rhs) {
            (Self::Num(left), Self::Num(right)) => (left, right),
            (left, right) => return crate::raise!("Can't compare `{}` and `{}`", left, right),
        };

        match left.partial_cmp(right) {
            Some(ord) => Ok(ord),
            None => raise!("Error applying cmp"),
        }
    }

    /// Convert the constant to a boolean
    pub fn to_bool(&self) -> bool {
        use Value::*;

        match self {
            Bool(b) => *b,
            Sym(_) => true,
            Str(s) if s.is_empty() => false,
            Str(_) => true,
            Num(n) if *n == 0.0 => false,
            Num(_) => true,
            Nil => false,
            List(xs) => !xs.is_empty(),
            Fn(_) => true,
            Value::Type(_) => true,
            Table(_) => true,
            Value::Instance(_) => true,
            Mutable(_) => true, //who cares?
        }
    }

    /// returns the type of the value
    pub fn type_of(&self) -> GcRef<YexType> {
        use Value::*;

        match self {
            Type(t) => return t.clone(),
            Instance(i) => return i.ty.clone(),
            _ => {}
        };

        let ty = match self {
            List(_) => YexType::list(),
            Fn(_) => YexType::fun(),
            Num(_) => YexType::num(),
            Str(_) => YexType::str(),
            Bool(_) => YexType::bool(),
            Nil => YexType::nil(),
            Sym(_) => YexType::sym(),
            Table(_) => YexType::table(),
            Mutable(_) => YexType::mutable(),
            Type(_) | Instance(_) => unreachable!(),
        };

        GcRef::new(ty)
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Nil
    }
}

type ConstantErr = InterpretResult<Value>;

impl From<Value> for bool {
    fn from(o: Value) -> Self {
        o.to_bool()
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        let tk = match self {
            Fn(f) => format!("<fun({})>", f.arity),
            Nil => "nil".to_string(),
            List(xs) => format!("{}", *xs),
            Str(s) => "\"".to_owned() + s + "\"",
            Sym(s) => format!("{}", s),
            Num(n) => n.to_string(),
            Type(t) => format!("<type({})>", t.name),
            Instance(i) => format!("<instance({})>", i.ty.name),
            Table(t) => format!("{t}"),
            Mutable(t) => format!("Mutable<{}>", t.get()),
            Bool(b) => b.to_string(),
        };
        write!(f, "{}", tk)
    }
}

impl Add for Value {
    type Output = ConstantErr;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Num(x), Self::Num(y)) => Ok(Self::Num(x + y)),
            (Self::Str(x), Self::Str(y)) => Ok(Self::Str(GcRef::new(x.to_string() + &y))),
            (s, r) => raise!("Can't apply `+` operator between {} and {}", s, r),
        }
    }
}

impl Sub for Value {
    type Output = ConstantErr;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, &rhs) {
            (Self::Num(x), Self::Num(y)) => Ok(Self::Num(x - y)),
            (s, r) => raise!("Can't apply `-` operator between {} and {}", s, r),
        }
    }
}

impl Mul for Value {
    type Output = ConstantErr;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, &rhs) {
            (Self::Num(x), Self::Num(y)) => Ok(Self::Num(x * y)),
            (s, r) => raise!("Can't apply `*` operator between {} and {}", s, r),
        }
    }
}

impl Div for Value {
    type Output = ConstantErr;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, &rhs) {
            (Self::Num(x), Self::Num(y)) => Ok(Self::Num(x / y)),
            (s, r) => raise!("Can't apply `/` operator between {} and {}", s, r),
        }
    }
}

impl Neg for Value {
    type Output = ConstantErr;

    fn neg(self) -> Self::Output {
        match self {
            Self::Num(n) => Ok(Self::Num(-n)),
            s => raise!("Can't apply unary `-` operator on {}", s),
        }
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        Self::Bool(!self.to_bool())
    }
}

impl BitXor for Value {
    type Output = ConstantErr;

    fn bitxor(self, rhs: Self) -> Self::Output {
        use Value::*;

        match (self, rhs) {
            (Num(x), Num(y)) => Ok(Num(((x.round() as i64) ^ (y.round() as i64)) as f64)),
            (x, y) => raise!("Can't apply bitwise `^` between {} and {}", x, y),
        }
    }
}

impl BitAnd for Value {
    type Output = ConstantErr;

    fn bitand(self, rhs: Self) -> Self::Output {
        use Value::*;

        match (self, rhs) {
            (Num(x), Num(y)) => Ok(Num(((x.round() as i64) & (y.round() as i64)) as f64)),
            (x, y) => raise!("Can't apply bitwise `&` between {} and {}", x, y),
        }
    }
}

impl BitOr for Value {
    type Output = ConstantErr;

    fn bitor(self, rhs: Self) -> Self::Output {
        use Value::*;

        match (self, rhs) {
            (Num(x), Num(y)) => Ok(Num(((x.round() as i64) | (y.round() as i64)) as f64)),
            (x, y) => raise!("Can't apply bitwise `|` between {} and {}", x, y),
        }
    }
}

impl Shr for Value {
    type Output = ConstantErr;

    fn shr(self, rhs: Self) -> Self::Output {
        use Value::*;

        match (self, rhs) {
            (Num(x), Num(y)) => Ok(Num(((x.round() as i64) >> (y.round() as i64)) as f64)),
            (x, y) => raise!("Can't apply bitwise `>>` between {} and {}", x, y),
        }
    }
}

impl Shl for Value {
    type Output = ConstantErr;

    fn shl(self, rhs: Self) -> Self::Output {
        use Value::*;

        match (self, rhs) {
            (Num(x), Num(y)) => Ok(Num(((x.round() as i64) << (y.round() as i64)) as f64)),
            (x, y) => raise!("Can't apply bitwise `<<` between {} and {}", x, y),
        }
    }
}

impl Rem for Value {
    type Output = ConstantErr;

    fn rem(self, rhs: Self) -> Self::Output {
        use Value::*;

        match (self, rhs) {
            (Num(x), Num(y)) => Ok(Num(x % y)),
            (x, y) => raise!("Can't apply `%` between {} and {}", x, y),
        }
    }
}

pub trait TryGet<T> {
    fn get(&self) -> InterpretResult<T>;
}
macro_rules! impl_get {
    ($to:ty: $pattern:tt) => {
        impl TryGet<$to> for Value {
            fn get(&self) -> InterpretResult<$to> {
                match self {
                    Self::$pattern(x) => Ok(x.clone()),
                    e => crate::raise!("expected {}, found {}", stringify!($to), e),
                }
            }
        }
    };
    ($to:ty: $pattern:pat => $parse_expr:expr) => {
        impl TryGet<$to> for Value {
            fn get(&self) -> InterpretResult<$to> {
                use Value::*;
                match self {
                    $pattern => Ok($parse_expr),
                    e => crate::raise!("expected {}, found {}", stringify!($to), e),
                }
            }
        }
    };
}
impl_get!(String: Str(s) => s.to_string());
impl_get!(f64: Num);
impl_get!(bool: Bool);
impl_get!(GcRef<YexType>: Type);
impl_get!(GcRef<Fn>: Fn);
impl_get!(GcRef<Instance>: Instance);
impl_get!(Table: Table);
impl_get!(Symbol: Sym);
impl_get!(List: List);
impl_get!(Mutable: Mutable(x) => x.deref().to_owned());
