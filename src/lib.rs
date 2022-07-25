//! This library defines the `Expr` type, where mathematical expressions can be created. It then
//! implements operations that can be applied to these expressions.

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![allow(dead_code)]

use std::ops::*;

type Num = isize;

/// An expression type! All mathematical expressions should be able to be expressed with this type.
/// This type is essentially an AST (abstract syntax tree).
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    /// A constant value (e.g. 1, 6, 15)
    Const(Num),
    /// Simply an X variable.
    /// This might be changed to an id based variable or something (because you will often want
    /// more variables than just x in expressions).
    X,
    /// The sum of each expression in the vector.
    Sum(Vec<Expr>),
    /// The product of each expression in the vector.
    Prod(Vec<Expr>),
    /// The negative value of the expression.
    Neg(Box<Expr>),
    /// The reciprocal of the value (i.e. 1 divided by the expression).
    Recip(Box<Expr>),
}

impl Expr {
    fn like_terms_with(&self, term: &Expr) -> bool {
        todo!()
    }
}

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
                    if matches!(e, Expr::Sum(_)) || matches!(e, Expr::Const(_)) {
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

impl Expr {
    // TODO list all simplifications that this function applies
    // TODO separate all of the simplifications into other functions
    /// Apply all simplification techniques to an expression (INCOMPLETE!)
    ///
    /// List of applied simplifications:
    /// [`Expr::simplify_sums_in_sums`]
    pub fn simplify(&mut self) {
        match self {
            Expr::Sum(v) => {
                // Simplify all of the terms in the sum first, then simplify the whole sum
                for e in v.iter_mut() {
                    e.simplify();
                }

                self.simplify_sums_in_sums();

                // Turn addition into multiplication or something
                // x + 2x = 3x for example
                todo!();
            }
            Expr::Prod(v) => {
                // Simplify all of the terms
                for e in v.iter_mut() {
                    e.simplify();
                }

                // Cancel out fractions!

                // Multiply constants
                // Also simplify fraction of constants
            }
            _ => unimplemented!(),
        };
    }

    /// This function simplifies [`Expr::Sum`] expressions which have [`Expr::Sum`] values in their
    /// own vector. For example, you could have `3 + (5 - 2)`, which would then be simplified to
    /// `3 + 5 - 2`.
    pub fn simplify_sums_in_sums(&mut self) {
        if let Expr::Sum(v) = self {
            // If there is an Expr::Sum in v, grab the vector in this internal Sum and append it to
            // v. Then remove the Expr::Sum value.
            //
            // I'm sure there's a better way to implement this!
            let mut i = 0;
            while i < v.len() {
                if let Expr::Sum(e) = &v[i] {
                    let mut e = e.clone();
                    v.append(&mut e);
                    v.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }

    /// This function simplifies [`Expr::Sum`] expressions which contain like terms which can be
    /// added together. For example, you could have `x + 2 + 2x + 4`, which would simplify to
    /// `3x + 6`. (INCOMPLETE!!)
    pub fn simplify_apply_sums(&mut self) {
        if let Expr::Sum(v) = self {
            let mut i = 1;
            while i < v.len() {
                let mut j = 0;
                while j < i {
                    if v[i].like_terms_with(&v[j]) {
                        let e = v[i].clone();
                        // Cant do this because it doesnt add the coefficients balbhalbalh
                        v[j] += e;
                        v[j].simplify();
                        v.remove(i);
                        i -= 1;
                        break;
                    }
                    j += 1;
                }
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn latex() {
        let mut e = Expr::X;
        e += Expr::X * Expr::Const(5);
        e /= Expr::X;

        assert_eq!(e.to_latex(), "x+x(5)\\frac{1}{x}");
    }
}
