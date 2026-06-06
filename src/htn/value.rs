use std::{cmp::Ordering, ops::Add, ops::Div, ops::Mul, ops::Sub};

#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
}

macro_rules! impl_arith {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for Value {
            type Output = Value;
            fn $method(self, rhs: Self) -> Self::Output {
                use Value::*;
                match (self, rhs) {
                    (Int(a),   Int(b))   => Int(a $op b),
                    (Float(a), Float(b)) => Float(a $op b),
                    (Int(a),   Float(b)) => Float(a as f32 $op b),
                    (Float(a), Int(b))   => Float(a $op b as f32),
                    (left, _) => left,
                }
            }
        }
    };
}

impl_arith!(Add, add, +);
impl_arith!(Sub, sub, -);
impl_arith!(Mul, mul, *);
impl_arith!(Div, div, /);

macro_rules! impl_ptr_arith {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for &Value {
            type Output = Value;
            fn $method(self, rhs: Self) -> Self::Output {
                use Value::*;
                match (self, rhs) {
                    (Int(a),   Int(b))   => Int(a $op b),
                    (Float(a), Float(b)) => Float(a $op b),
                    (Int(a),   Float(b)) => Float(*a as f32 $op b),
                    (Float(a), Int(b))   => Float(a $op *b as f32),
                    (left, _) => left.clone(),
                }
            }
        }
    };
}

impl_ptr_arith!(Add, add, +);
impl_ptr_arith!(Sub, sub, -);
impl_ptr_arith!(Mul, mul, *);
impl_ptr_arith!(Div, div, /);

macro_rules! impl_from {
    ($variant:ident, $ty:ty) => {
        impl From<$ty> for Value {
            fn from(v: $ty) -> Self {
                Value::$variant(v)
            }
        }
    };

    ($variant:ident, $ty:ty, |$v:ident| $body:expr) => {
        impl From<$ty> for Value {
            fn from($v: $ty) -> Self {
                Value::$variant($body)
            }
        }
    };
}

impl_from!(Int, i32);
impl_from!(Float, f32);
impl_from!(Bool, bool);
impl_from!(String, String);
impl_from!(String, &str, |v| v.to_string());

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => a.partial_cmp(b),
            (Self::Int(a), Self::Int(b)) => a.partial_cmp(b),
            (Self::Float(a), Self::Float(b)) => a.partial_cmp(b),
            (Self::String(a), Self::String(b)) => a.partial_cmp(b),
            (Self::Int(a), Self::Float(b)) => (*a as f32).partial_cmp(b),
            (Self::Float(a), Self::Int(b)) => a.partial_cmp(&(*b as f32)),
            _ => None,
        }
    }
}
