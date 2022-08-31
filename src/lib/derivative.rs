use super::Expr;

impl Expr {
    /// Find the derivative of an expression.
    pub fn derivative(self) -> Self {
        match self {
            // The derivative of a constant is 0
            Expr::Const(_) => Expr::Const(0),
            Expr::Prod(v) if v.is_empty() => Expr::Const(0),
            Expr::Pow(_, b) if matches!(*b, Expr::Const(0)) => Expr::Const(0),
            // Simplifications
            Expr::Prod(mut v) if v.len() == 1 => v.pop().unwrap().derivative(),
            Expr::Pow(a, b) if matches!(*b, Expr::Const(1)) => a.derivative(),
            // The derivative of x is 1
            Expr::X => Expr::Const(1),
            // The derivative of a sum of expressions is the sum of the expressions' derivatives
            // Maybe it is better to use an itermut to skip the collection but the borrow checker
            // was being annoying
            Expr::Sum(v) => Expr::Sum(v.into_iter().map(|x| x.derivative()).collect()),
            // The derivative of a negative expression is negative the derivative of the expression
            // when made positiv
            Expr::Neg(e) => -e.derivative(),
            // Product rule (ab)' = a'b + ab'
            Expr::Prod(mut v) => {
                // This should never panic because we have already checked the vector length
                let a = v.pop().unwrap();
                let b = Expr::Prod(v);

                // This a.clone() is ugly, especially when b exists without the clone
                a.clone() * b.clone().derivative() + b * a.derivative()
            }
            // Power rule (x^a)' = ax^(a-1)
            Expr::Pow(a, b) if matches!(*b, Expr::Const(_)) => {
                let dec = *b.clone() - Expr::Const(1);
                // Chain rule
                *b * a.clone().pow(dec) * a.derivative()
            }
            // a^b = e^(lna * b) so then the derivative is just a^b * (lna * b)'
            Expr::Pow(ref a, ref b) => (a.clone().ln() * *b.clone()).derivative() * self,

            // A bunch of rules + chain rule added in
            Expr::Ln(x) => (1 / *x.clone()) * x.derivative(),
            Expr::Sin(x) => Expr::Cos(x.clone()) * x.derivative(),
            Expr::Cos(x) => -Expr::Sin(x.clone()) * x.derivative(),
            Expr::Arcsin(x) => {
                (1 - x.clone().pow(Expr::Const(2))).pow(Expr::Const(1) / 2) * x.derivative()
            }
            Expr::Arccos(x) => {
                -(1 - x.clone().pow(Expr::Const(2))).pow(Expr::Const(1) / 2) * x.derivative()
            }
            Expr::Arctan(x) => 1 / (1 + x.clone().pow(Expr::Const(2))) * x.derivative(),
        }
    }
}
