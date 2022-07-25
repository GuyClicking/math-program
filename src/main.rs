// TODO!!!!! macro stuff

use std::ops::*;

type Num = isize;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Expr {
    Const(Num),
    X,
    Sum(Vec<Expr>),
    Prod(Vec<Expr>),
    Neg(Box<Expr>),
    // Reciprocal
    Recip(Box<Expr>),
}

impl Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Expr::Sum(match self {
            Expr::Sum(v) => {
                let mut v = v.clone();
                v.push(rhs);
                // TODO simplify equation
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
                let mut v = v.clone();
                v.push(rhs);
                // TODO simplify expression
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

    fn div(self, rhs: Self) -> Self {
        self * Expr::Recip(Box::new(rhs))
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

impl Expr {
    /// Write an expression as a latex math equation.
    pub fn to_latex(&self) -> String {
        match self {
            Expr::Const(n) => n.to_string(),
            Expr::X => "x".to_string(),
            Expr::Neg(e) => format!("-{}", e.to_latex()),
            Expr::Recip(e) => format!("\\frac{{1}}{{{}}}", e.to_latex()),
            Expr::Sum(v) => {
                let mut str = v[0].to_latex();
                for e in v.iter().skip(1) {
                    if let Expr::Neg(e) = e {
                        str += "-";
                        str += &e.to_latex();
                    } else {
                        str += "+";
                        str += &e.to_latex();
                    }
                }
                str
            }
            Expr::Prod(v) => {
                let mut str = v[0].to_latex();
                for e in v.iter().skip(1) {
                    if let Expr::Sum(_) = e {
                        str += "(";
                        str += &e.to_latex();
                        str += ")";
                    } else {
                        str += &e.to_latex();
                    }
                }
                str
            }
        }
    }
}

fn main() {
    let mut e = Expr::X;
    e /= Expr::X - Expr::X;
    e *= Expr::X + Expr::X;
    println!("{}", e.to_latex());
    println!("{:?}", e);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn latex() {
        let mut e = Expr::X;
        e += Expr::Const(5) * Expr::X;
        e /= Expr::X;

        assert_eq!(e.to_latex(), "x+5x\\frac{1}{x}");
    }
}
