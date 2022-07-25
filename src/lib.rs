//! This library defines the `Expr` type, where mathematical expressions can be created. It then
//! implements operations that can be applied to these expressions.

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![allow(dead_code)]

use std::ops::*;

type Num = isize;

/// An expression type! All mathematical expressions should be able to be expressed with this type.
/// This type is essentially an AST (abstract syntax tree).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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
        if self == term {
            return true;
        }
        match self {
            Expr::Prod(a) => match term {
                Expr::Prod(b) => {
                    // If both terms, with the coefficients removed, are equal (after sorting) then
                    // they are like terms!
                    let mut a = a.clone();
                    let mut b = b.clone();
                    a.retain(|x| !matches!(x, Expr::Const(_)));
                    a.sort();
                    b.retain(|x| !matches!(x, Expr::Const(_)));
                    b.sort();

                    a == b
                }
                _ => todo!(),
            },
            Expr::Sum(a) => match term {
                Expr::Sum(b) => {
                    // If they are both sums and they have the same elements but in different orders, return
                    // true
                    // Sort first so that they have the same orders
                    let mut a = a.clone();
                    let mut b = b.clone();
                    a.sort();
                    b.sort();

                    a == b
                }
                _ => false,
            },
            _ => false,
        }
    }
    fn add_like_term(&mut self, term: &Expr) {
        match self {
            Expr::Const(a) => {
                // If both terms are like terms and one is constant the other one must also be
                // constant, so there doesn't need to be an else contrition here.
                if let Expr::Const(b) = term {
                    *self = Expr::Const(*a + *b);
                }
            }
            Expr::Prod(a) => match term {
                Expr::Prod(b) => {
                    // Surely theres a better way than the into_iter filter closure next thing
                    let c1 = a.iter_mut().find(|x| matches!(x, Expr::Const(_)));
                    let c2 = b.iter().find(|x| matches!(x, Expr::Const(_)));
                    if let Some(Expr::Const(c1)) = c1 {
                        if let Some(Expr::Const(c2)) = c2 {
                            *c1 += c2;
                        }
                    }
                }
                _ => {
                    if a.contains(term) {
                        // Ths case would just be like 3x + x for example
                        // There is no coefficient product for the 2nd one
                        // so add 1 to 3
                        let c1 = a.iter_mut().find(|x| matches!(x, Expr::Const(_)));
                        if let Some(Expr::Const(c1)) = c1 {
                            *c1 += 1;
                        }
                    }
                }
            },
            _ => {
                if self == term {
                    // This is just e + e = 2e
                    *self = Expr::Prod(vec![Expr::Const(2), self.clone()]);
                }
            }
        }
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
    /// [`Expr::simplify_singleton`]
    /// [`Expr::simplify_sums_in_sums`]
    /// [`Expr::simplify_apply_sums`]
    /// [`Expr::simplify_mult_consts`]
    pub fn simplify(&mut self) {
        match self {
            Expr::Sum(v) => {
                // Simplify all of the terms in the sum first, then simplify the whole sum
                for e in v.iter_mut() {
                    e.simplify();
                }

                if self.simplify_singleton() {
                    return;
                }

                self.simplify_sums_in_sums();

                // Turn addition into multiplication or something
                // x + 2x = 3x for example
                self.simplify_apply_sums();
            }
            Expr::Prod(v) => {
                // Simplify all of the terms
                for e in v.iter_mut() {
                    e.simplify();
                }

                if self.simplify_singleton() {
                    return;
                }

                // Cancel out fractions!

                // Multiply constants
                self.simplify_mult_consts();
                // Also simplify fraction of constants
                todo!();
            }
            _ => unimplemented!(),
        };
    }

    /// This function turns sums or products of singular terms into just their terms.
    /// Returns true if the mutated term is no longer a multivalued operation (Sum or Prod)
    pub fn simplify_singleton(&mut self) -> bool {
        match self {
            Expr::Sum(v) => {
                if v.is_empty() {
                    *self = Expr::Const(0);
                    return true;
                } else if v.len() == 1 {
                    // I feel like I shouldn't use an unwrap but len == 1
                    *self = v.first().unwrap().clone();
                    return true;
                }
                false
            }
            Expr::Prod(v) => {
                if v.is_empty() {
                    *self = Expr::Const(0);
                    return true;
                } else if v.len() == 1 {
                    *self = v.first().unwrap().clone();
                    return true;
                }
                false
            }
            _ => false,
        }
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
                        v[j].add_like_term(&e);
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

    /// This function multiplies constants in a product.
    /// e.g. `5 * x * 6 = 30 * x`
    pub fn simplify_mult_consts(&mut self) {
        if let Expr::Prod(v) = self {
            let mut i = 1;
            while i < v.len() {
                if let Expr::Const(b) = v[i] {
                    let mut j = 0;
                    while j < i {
                        if let Expr::Const(a) = &mut v[j] {
                            *a += b;
                        }
                        j += 1;
                    }
                    i += 1;
                } else {
                    i += 1;
                    continue;
                }
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
