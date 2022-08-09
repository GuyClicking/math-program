//! This library defines the `Expr` type, where mathematical expressions can be created. It then
//! implements operations that can be applied to these expressions.

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

mod derivative;
mod latex;
mod operations;

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
    /// One expression to the power of another (a^b)
    Pow(Box<Expr>, Box<Expr>),
    /// Ln of an expression
    Ln(Box<Expr>),
    /// Sin of an expression
    Sin(Box<Expr>),
    /// Cos of an expression
    Cos(Box<Expr>),
    /// Arcsin of an expression
    Arcsin(Box<Expr>),
    /// Arccos of an expression
    Arccos(Box<Expr>),
    /// Arctan of an expression
    Arctan(Box<Expr>),
}

impl Expr {
    /// Get the reciprocal of an expression (i.e. 1/x)
    pub fn recip(self) -> Self {
        match self {
            Expr::Pow(a, b) => a.pow(-*b),
            _ => self.pow(Expr::Const(-1)),
        }
    }

    /// Raise an expression to a power
    pub fn pow(self, b: Expr) -> Self {
        Expr::Pow(Box::new(self), Box::new(b))
    }

    /// Get the ln of an expression
    pub fn ln(self) -> Self {
        Expr::Ln(Box::new(self))
    }

    /// Get the sin of an expression
    pub fn sin(self) -> Self {
        Expr::Sin(Box::new(self))
    }

    /// Get the cos of an expression
    pub fn cos(self) -> Self {
        Expr::Cos(Box::new(self))
    }
}

impl Expr {
    /// Apply all simplification techniques to an expression (INCOMPLETE!)
    ///
    /// List of applied simplifications:
    /// [`Expr::simplify_terms`]
    /// [`Expr::simplify_singleton`]
    /// [`Expr::simplify_zero_pow`]
    /// [`Expr::simplify_one_pow`]
    /// [`Expr::simplify_negative_consts`]
    pub fn simplify(&mut self) {
        match self {
            Expr::Sum(_) => {
                // Simplify all of the terms in the sum first, then simplify the whole sum
                self.simplify_terms();

                self.simplify_singleton();
            }
            Expr::Prod(_) => {
                // Simplify all of the terms
                self.simplify_terms();

                self.simplify_singleton();
            }
            Expr::Pow(_, _) => {
                self.simplify_terms();

                self.simplify_zero_pow();
                self.simplify_one_pow();
            }
            Expr::Neg(_) => {
                self.simplify_negative_consts();
            }
            // Dont do this!
            _ => (),
        };
        match self {
            Expr::Sum(v) => {
                v.sort();
            }
            Expr::Prod(v) => {
                v.sort();
            }
            _ => (),
        };
    }

    /// This function simplifies all of the terms in an expression. For example, it may simplify
    /// all terms in a sum.
    pub fn simplify_terms(&mut self) {
        match self {
            Expr::Sum(v) => {
                for e in v.iter_mut() {
                    e.simplify();
                }
            }
            Expr::Prod(v) => {
                for e in v.iter_mut() {
                    e.simplify();
                }
            }
            Expr::Pow(a, b) => {
                a.simplify();
                b.simplify();
            }
            Expr::Neg(x) => {
                x.simplify();
            }
            _ => (),
        }
    }

    /// This function turns sums or products with a singular term into just their term.
    pub fn simplify_singleton(&mut self) {
        match self {
            Expr::Sum(v) => {
                if v.is_empty() {
                    *self = Expr::Const(0);
                } else if v.len() == 1 {
                    // I feel like I shouldn't use an unwrap but len == 1
                    *self = v.first().unwrap().clone();
                }
            }
            Expr::Prod(v) => {
                if v.is_empty() {
                    *self = Expr::Const(0);
                } else if v.len() == 1 {
                    *self = v.first().unwrap().clone();
                }
            }
            _ => (),
        }
    }

    /// This function turns expressions to the power of 0 to 1
    /// e.g. `x^0 = 1`
    pub fn simplify_zero_pow(&mut self) {
        if let Expr::Pow(_, b) = self {
            if **b == Expr::Const(0) {
                *self = Expr::Const(1);
            }
        }
    }

    /// This function turns expressions to the power of 1 to x
    /// e.g. `x^x = x`
    pub fn simplify_one_pow(&mut self) {
        if let Expr::Pow(a, b) = self {
            if **b == Expr::Const(1) {
                *self = *a.clone();
            }
        }
    }

    /// This function turns expressions of the form `Neg(Const(x))` into Const(-x).
    pub fn simplify_negative_consts(&mut self) {
        if let Expr::Neg(x) = self {
            if let Expr::Const(c) = **x {
                *self = Expr::Const(-c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simplification() {
        // Singleton test
        let mut e = Expr::Sum(vec![Expr::X]);
        e.simplify_singleton();
        assert_eq!(e, Expr::X);
    }
}
