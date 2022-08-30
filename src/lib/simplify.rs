use super::Expr;

impl Expr {
    /// Apply all simplification techniques to an expression (INCOMPLETE!)
    ///
    /// List of applied simplifications:
    /// [`Expr::simplify_terms`]
    /// [`Expr::simplify_singleton`]
    /// [`Expr::simplify_zero_pow`]
    /// [`Expr::simplify_one_pow`]
    /// [`Expr::simplify_negative_consts`]
    /// [`Expr::simplify_double_negative`]
    /// [`Expr::simplify_distribute_negative_in_sum`]
    /// [`Expr::simplify_times_zero`]
    /// [`Expr::simplify_plus_zero`]
    pub fn simplify(&mut self) {
        // Simplify all subterms before simplifying the current term
        self.simplify_terms();
        match self {
            Expr::Const(_) => (),
            Expr::X => (),
            Expr::Sum(_) => {
                self.simplify_singleton();
                self.simplify_plus_zero();
            }
            Expr::Prod(_) => {
                self.simplify_singleton();
                self.simplify_times_zero();
            }
            Expr::Neg(_) => {
                self.simplify_negative_consts();
                self.simplify_double_negative();
                self.simplify_distribute_negative_in_sum();
            }
            Expr::Pow(_, _) => {
                self.simplify_zero_pow();
                self.simplify_one_pow();
            }
            Expr::Ln(_) => (),
            Expr::Sin(_) => (),
            Expr::Cos(_) => (),
            Expr::Arcsin(_) => (),
            Expr::Arccos(_) => (),
            Expr::Arctan(_) => (),
        };
    }

    /// This function simplifies all of the terms in an expression. For example, it may simplify
    /// all terms in a sum.
    pub fn simplify_terms(&mut self) {
        match self {
            Expr::Const(_) => (),
            Expr::X => (),
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
            Expr::Neg(x) => {
                x.simplify();
            }
            Expr::Pow(a, b) => {
                a.simplify();
                b.simplify();
            }
            Expr::Ln(x) => {
                x.simplify();
            }
            Expr::Sin(x) => {
                x.simplify();
            }
            Expr::Cos(x) => {
                x.simplify();
            }
            Expr::Arcsin(x) => {
                x.simplify();
            }
            Expr::Arccos(x) => {
                x.simplify();
            }
            Expr::Arctan(x) => {
                x.simplify();
            }
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
    /// e.g. `x^1 = x`
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

    /// This function turns a negative of a negative of an expression into just the expression
    pub fn simplify_double_negative(&mut self) {
        if let Expr::Neg(x) = self {
            if let Expr::Neg(x) = x.as_mut() {
                // use unsafe std::ptr::read() ? faster but unsafe
                let a = std::mem::replace(x, Box::new(Expr::X));
                *self = *a;
                // In case it is a quadruple+ negative
                self.simplify_double_negative();
            }
        }
    }

    /// This function distributes the negative sign in a sum to all of it's elements
    pub fn simplify_distribute_negative_in_sum(&mut self) {
        if let Expr::Neg(x) = self {
            if let Expr::Sum(v) = x.as_mut() {
                // I hope this is fine
                let v = v
                    .iter_mut()
                    .map(|x| -std::mem::replace(x, Expr::X))
                    .collect();
                *self = Expr::Sum(v);
                // Simplify the new stuff
                // This may not be a good idea
                self.simplify_terms();
            }
        }
    }

    /// This function turns expressions multiplied by zero into just zero
    pub fn simplify_times_zero(&mut self) {
        if let Expr::Prod(v) = self {
            if v.contains(&Expr::Const(0)) {
                *self = Expr::Const(0);
            }
        }
    }

    /// This function removes zeros from sums
    pub fn simplify_plus_zero(&mut self) {
        if let Expr::Sum(_) = self {
            todo!()
        }
    }

    /// This function adds constants in a sum together
    pub fn simplify_add_consts(&mut self) {
        if let Expr::Sum(_) = self {
            todo!();
        }
    }

    /// This function multiplies constants in a sum together
    pub fn simplify_multiply_consts(&mut self) {
        if let Expr::Prod(_) = self {
            todo!();
        }
    }
}
