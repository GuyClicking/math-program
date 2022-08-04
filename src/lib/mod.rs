//! This library defines the `Expr` type, where mathematical expressions can be created. It then
//! implements operations that can be applied to these expressions.

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

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
    pub fn recip(&self) -> Self {
        match self {
            Expr::Pow(a, b) => Expr::Pow(a.clone(), Box::new(-*b.clone())),
            _ => Expr::Pow(Box::new(self.clone()), Box::new(Expr::Const(-1))),
        }
    }

    /// Get the ln of an expression
    pub fn ln(&self) -> Self {
        Expr::Ln(Box::new(self.clone()))
    }
}

impl Expr {
    /// Write an expression as a latex math equation.
    // TODO negative indecies as fractions
    pub fn to_latex(&self) -> String {
        match self {
            Expr::Const(n) => n.to_string(),
            Expr::X => "x".to_string(),
            Expr::Neg(e) => format!("-{}", e.to_latex()),
            // Expr::Recip(e) => format!("\\frac{{1}}{{{}}}", e.to_latex()),
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
                let mut str = if v[0] == Expr::Const(1) {
                    "".to_string()
                } else {
                    v[0].to_latex()
                };
                for e in v.iter().skip(1) {
                    if matches!(e, Expr::Sum(_)) || matches!(e, Expr::Const(_)) {
                        if let Expr::Const(e) = e {
                            if *e == 1 {
                                continue;
                            }
                        }
                        str += "(";
                        str += &e.to_latex();
                        str += ")";
                    } else {
                        str += &e.to_latex();
                    }
                }
                str
            }
            Expr::Pow(a, b) => {
                format!("{}^{{{}}}", &a.to_latex(), &b.to_latex())
            }
            Expr::Ln(x) => {
                format!("ln{}", &x.to_latex())
            }
            Expr::Sin(x) => {
                format!("sin{}", &x.to_latex())
            }
            Expr::Cos(x) => {
                format!("cos{}", &x.to_latex())
            }
            Expr::Arcsin(x) => {
                format!("arcsin{}", &x.to_latex())
            }
            Expr::Arccos(x) => {
                format!("arccos{}", &x.to_latex())
            }
            Expr::Arctan(x) => {
                format!("arctan{}", &x.to_latex())
            }
        }
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

impl Expr {
    /// Find the derivative of an expression.
    pub fn derivative(&self) -> Self {
        match self {
            // The derivative of a constant is 0
            Expr::Const(_) => Expr::Const(0),
            // The derivative of x is 1
            Expr::X => Expr::Const(1),
            // The derivative of a sum of expressions is the sum of the expressions' derivatives
            Expr::Sum(v) => Expr::Sum(v.iter().map(|x| x.derivative()).collect()),
            // The derivative of a negative expression is the negative of the derivative of the
            // expression
            Expr::Neg(e) => Expr::Neg(Box::new(e.derivative())),
            // Product rule (ab)' = a'b + ab'
            Expr::Prod(v) => {
                let a = &v[0];
                let b = Expr::Prod(v[1..].to_vec());

                a.clone() * b.derivative() + b * a.derivative()
            }
            // x^0 = 1 so the derivative is 0
            Expr::Pow(_, b) if matches!(**b, Expr::Const(0)) => Expr::Const(0),
            // x^1 = x so the derivative is the derivative of x
            Expr::Pow(a, b) if matches!(**b, Expr::Const(1)) => a.clone().derivative(),
            // Power rule (x^a)' = ax^(a-1)
            Expr::Pow(a, b) if matches!(**b, Expr::Const(_)) => {
                let dec = *b.clone() - Expr::Const(1);
                // Chain rule
                *b.clone() * Expr::Pow(a.clone(), Box::new(dec)) * b.clone().derivative()
            }
            // a^b = e^(lna * b) so then the derivative is just a^b * (lna * b)'
            Expr::Pow(a, b) => self.clone() * (a.clone().ln() * *b.clone()).derivative(),
            // derivative of lnx is 1/x
            Expr::Ln(x) => x.clone().derivative() * Expr::Pow(x.clone(), Box::new(Expr::Const(-1))),
            Expr::Sin(x) => x.clone().derivative() * Expr::Cos(x.clone()),
            Expr::Cos(x) => x.clone().derivative() * -Expr::Sin(x.clone()),
            Expr::Arcsin(x) => {
                x.clone().derivative()
                    * Expr::Pow(
                        Box::new(-(*x.clone() * *x.clone() + 1)),
                        Box::new(Expr::Const(1) / 2),
                    )
            }
            Expr::Arccos(x) => {
                x.clone().derivative()
                    * -Expr::Pow(
                        Box::new(-(*x.clone() * *x.clone() + 1)),
                        Box::new(Expr::Const(1) / 2),
                    )
            }
            Expr::Arctan(x) => {
                x.clone().derivative()
                    * Expr::Pow(
                        Box::new((*x.clone() * *x.clone()) + 1),
                        Box::new(Expr::Const(-1)),
                    )
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

        //assert_eq!(e.to_latex(), "(x+x(5))x^{-1}");
    }

    #[test]
    fn simplification() {
        // Singleton test
        let mut e = Expr::Sum(vec![Expr::X]);
        e.simplify_singleton();
        assert_eq!(e, Expr::X);
    }
}
