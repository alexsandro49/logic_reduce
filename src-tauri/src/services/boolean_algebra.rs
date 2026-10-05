use crate::models::{expression, law, notation, operation, rewrite};

pub fn simplify(
    mut e: expression::Expr,
) -> (
    expression::Expr,
    Vec<(expression::Expr, expression::Expr, law::Law, &'static str)>,
) {
    let mut s = Vec::new();
    for _ in 0..1000 {
        if let Some(r) = rewrite::Rewrite::rewrite_once(&e) {
            s.push((e, r.e.clone(), r.law, r.explanation));
            e = r.e
        } else {
            return (e, s);
        }
    }
    panic!("simplificação não convergiu")
}

pub fn format(
    e: &expression::Expr,
    parent: Option<operation::Op>,
    right: bool,
    n: notation::Notation,
) -> String {
    match e {
        expression::Expr::Var(s) => s.clone(),
        expression::Expr::Const(v) => (if *v { "1" } else { "0" }).into(),
        expression::Expr::Not(a) => {
            let mut v = format(a, Some(operation::Op::And), false, n);
            if operation::op_of(a).is_some() {
                v = format!("({})", v);
            }
            if matches!(n, notation::Notation::Latex) {
                format!("\\overline{{{}}}", v)
            } else {
                format!(
                    "{}{}",
                    if matches!(n, notation::Notation::Logic) {
                        "¬"
                    } else if matches!(n, notation::Notation::ProgBools) {
                        "!"
                    } else {
                        "~"
                    },
                    v
                )
            }
        }
        expression::Expr::Bin(a, o, b) => {
            let l = format(a, Some(*o), false, n);
            let r = format(b, Some(*o), true, n);
            let mut ls = l;
            let mut rs = r;
            if operation::prec(operation::op_of(a)) < operation::prec(Some(*o))
                || (*o == operation::Op::Or && operation::op_of(a) == Some(operation::Op::And))
            {
                ls = format!("({})", ls);
            }
            if operation::prec(operation::op_of(b)) < operation::prec(Some(*o))
                || (*o == operation::Op::Or && operation::op_of(b) == Some(operation::Op::And))
                || (right && operation::op_of(b) == Some(*o) && *o != operation::Op::And)
            {
                rs = format!("({})", rs);
            }
            let _ = parent;
            format!("{}{}{}", ls, operation::symbol(*o, n), rs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{format, simplify};
    use crate::models::{notation::Notation, parser::Parser};

    #[test]
    fn simplifies_absorption_across_associated_terms() {
        let expression = Parser::parse("(A & ~C) + (B + ~C)").unwrap();
        let (simplified, steps) = simplify(expression);

        assert_eq!(format(&simplified, None, false, Notation::Default), "B + ~C");
        assert!(steps.iter().any(|(_, _, law, _)| matches!(law, crate::models::law::Law::Absorption)));
    }

    #[test]
    fn applies_commutativity_in_a_stable_direction() {
        let expression = Parser::parse("B + A").unwrap();
        let (simplified, steps) = simplify(expression);

        assert_eq!(format(&simplified, None, false, Notation::Default), "A + B");
        assert!(steps.iter().any(|(_, _, law, _)| matches!(law, crate::models::law::Law::Commutativity)));
    }

    #[test]
    fn normalizes_association_to_the_left() {
        let expression = Parser::parse("A + (B + C)").unwrap();
        let (_, steps) = simplify(expression);

        assert!(steps.iter().any(|(_, _, law, _)| matches!(law, crate::models::law::Law::Associativity)));
    }
}
