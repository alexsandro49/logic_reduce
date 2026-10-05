use crate::models::{expression, notation};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    And,
    Or,
    Xor,
}

pub fn prec(op: Option<Op>) -> i32 {
    match op {
        None => 100,
        Some(Op::Or) => 1,
        Some(Op::Xor) => 2,
        Some(Op::And) => 3,
    }
}
pub fn op_of(e: &expression::Expr) -> Option<Op> {
    if let expression::Expr::Bin(_, o, _) = e {
        Some(*o)
    } else {
        None
    }
}
pub fn symbol(op: Op, n: notation::Notation) -> &'static str {
    match n {
        notation::Notation::Default => match op {
            Op::And => " & ",
            Op::Or => " + ",
            Op::Xor => " ^ ",
        },
        notation::Notation::Logic => match op {
            Op::And => " ∧ ",
            Op::Or => " ∨ ",
            Op::Xor => " ⊻ ",
        },
        notation::Notation::Mathematical => match op {
            Op::And => " ⋅ ",
            Op::Or => " + ",
            Op::Xor => " ⊕ ",
        },
        notation::Notation::ProgBools => match op {
            Op::And => " && ",
            Op::Or => " || ",
            Op::Xor => " ^ ",
        },
        notation::Notation::ProgBits => match op {
            Op::And => " & ",
            Op::Or => " | ",
            Op::Xor => " ^ ",
        },
        notation::Notation::AltLogic => match op {
            Op::And => " ∧ ",
            Op::Or => " ∨ ",
            Op::Xor => " ≢ ",
        },
        notation::Notation::Latex => match op {
            Op::And => " \\cdot ",
            Op::Or => " + ",
            Op::Xor => " \\oplus ",
        },
    }
}
