use super::{Expr, Num};
use std::ops::*;

impl Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Expr::Sum(match self {
            Expr::Sum(v) => {
                let mut v = v;
                v.push(rhs);
                v
            }
            _ => {
                vec![self, rhs]
            }
        })
    }
}

impl Mul for Expr {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Expr::Prod(match self {
            Expr::Prod(v) => {
                let mut v = v;
                v.push(rhs);
                v
            }
            _ => {
                vec![self, rhs]
            }
        })
    }
}

impl Neg for Expr {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Expr::Neg(e) => *e,
            _ => Expr::Neg(Box::new(self)),
        }
    }
}

impl Sub for Expr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl Div for Expr {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self {
        self * rhs.recip()
    }
}

macro_rules! assigning_operator {
    ($trait_name:ty, $func_name:ident, $token:tt) => {
        impl $trait_name for Expr {
            fn $func_name(&mut self, rhs: Self) {
                *self = self.clone() $token rhs;
            }
        }
    }
}

assigning_operator!(AddAssign, add_assign, +);
assigning_operator!(MulAssign, mul_assign, *);
assigning_operator!(SubAssign, sub_assign, -);
assigning_operator!(DivAssign, div_assign, /);

macro_rules! apply_num {
    ($trait_name:ty, $assign_trait:ty, $func_name:ident, $assign_func:ident, $token:tt) => {
        impl $trait_name for Expr {
            type Output = Self;
            fn $func_name(self, rhs: Num) -> Self::Output {
                self.clone() $token Expr::Const(rhs)
            }
        }
        impl $assign_trait for Expr {
            fn $assign_func(&mut self, rhs: Num) {
                *self = self.clone() $token rhs;
            }
        }
    }
}

apply_num!(Add<Num>, AddAssign<Num>, add, add_assign, +);
apply_num!(Mul<Num>, MulAssign<Num>, mul, mul_assign, *);
apply_num!(Sub<Num>, SubAssign<Num>, sub, sub_assign, -);
apply_num!(Div<Num>, DivAssign<Num>, div, div_assign, /);

macro_rules! apply_to_num {
    ($trait_name:ty, $func_name:ident, $token:tt) => {
        impl $trait_name for Num {
            type Output = Expr;
            fn $func_name(self, rhs: Expr) -> Self::Output {
                Expr::Const(self) $token rhs
            }
        }
    }
}

apply_to_num!(Add<Expr>, add, +);
apply_to_num!(Mul<Expr>, mul, *);
apply_to_num!(Sub<Expr>, sub, -);
apply_to_num!(Div<Expr>, div, /);
