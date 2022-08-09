use super::Expr;

impl Expr {
    /// Write an expression as a latex math equation.
    // TODO negative indecies as fractions
    // basically just redo this whole function
    pub fn to_latex(&self) -> String {
        match self {
            Expr::Const(n) => n.to_string(),
            Expr::X => "x".to_string(),
            Expr::Neg(e) => format!("-({})", e.to_latex()),
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
                } else if matches!(v[0], Expr::Sum(_))
                    || matches!(v[0], Expr::Const(_))
                    || matches!(v[0], Expr::Neg(_))
                {
                    "(".to_owned() + &v[0].to_latex() + ")"
                } else {
                    v[0].to_latex()
                };
                for e in v.iter().skip(1) {
                    if matches!(e, Expr::Sum(_))
                        || matches!(e, Expr::Const(_))
                        || matches!(e, Expr::Neg(_))
                    {
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
                format!("({})^{{{}}}", &a.to_latex(), &b.to_latex())
            }
            Expr::Ln(x) => {
                format!("ln({})", &x.to_latex())
            }
            Expr::Sin(x) => {
                format!("sin({})", &x.to_latex())
            }
            Expr::Cos(x) => {
                format!("cos({})", &x.to_latex())
            }
            Expr::Arcsin(x) => {
                format!("arcsin({})", &x.to_latex())
            }
            Expr::Arccos(x) => {
                format!("arccos({})", &x.to_latex())
            }
            Expr::Arctan(x) => {
                format!("arctan({})", &x.to_latex())
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

        println!("{:?}", e);
        assert_eq!(e.to_latex(), "(x+x(5))x^{-1}");
    }
}
