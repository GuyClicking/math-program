//! This library defines the `Expr` type, where mathematical expressions can be created. It then
//! implements operations that can be applied to these expressions.

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

mod derivative;
mod latex;
mod operations;
mod simplify;

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
    /// The negative value of the expression. This may be removed and replaced with multiplying by
    /// -1
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
