use crate::models::operation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Var(String),
    Const(bool),
    Not(Box<Expr>),
    Bin(Box<Expr>, operation::Op, Box<Expr>),
}

pub fn not(e: Expr) -> Expr {
    Expr::Not(Box::new(e))
}

pub fn bin(a: Expr, op: operation::Op, b: Expr) -> Expr {
    Expr::Bin(Box::new(a), op, Box::new(b))
}

pub fn children(e: &Expr) -> Option<(&Expr, &Expr)> {
    if let Expr::Bin(a, _, b) = e {
        Some((a, b))
    } else {
        None
    }
}
pub fn constant(e: &Expr) -> Option<bool> {
    if let Expr::Const(v) = e {
        Some(*v)
    } else {
        None
    }
}

pub fn same(a: &Expr, b: &Expr) -> bool {
    a == b
}

pub fn neg_of(a: &Expr, b: &Expr) -> bool {
    matches!(a,Expr::Not(x) if x.as_ref()==b) || matches!(b,Expr::Not(x) if x.as_ref()==a)
}

pub fn ordering_key(e: &Expr) -> String {
    match e {
        Expr::Var(name) => format!("0:{name}"),
        Expr::Const(value) => format!("1:{}", u8::from(*value)),
        Expr::Not(inner) => format!("2:{}", ordering_key(inner)),
        Expr::Bin(left, op, right) => format!(
            "3:{op:?}:{}:{}",
            ordering_key(left),
            ordering_key(right)
        ),
    }
}

pub fn terms<'a>(e: &'a Expr, op: operation::Op, out: &mut Vec<&'a Expr>) {
    if let Expr::Bin(a, o, b) = e
        && *o == op
    {
        terms(a, op, out);
        terms(b, op, out);
        return;
    }
    out.push(e)
}

pub fn rebuild(xs: &[&Expr], op: operation::Op) -> Expr {
    xs.iter()
        .skip(1)
        .fold((*xs[0]).clone(), |a, b| bin(a, op, (*b).clone()))
}
