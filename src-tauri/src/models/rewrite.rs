use crate::models::{expression, law, operation};

pub struct Rewrite {
    pub e: expression::Expr,
    pub law: law::Law,
    pub explanation: &'static str,
}

impl Rewrite {
    pub fn rewrite_once(e: &expression::Expr) -> Option<Rewrite> {
        Self::rewrite_filtered(e, None, None)
    }

    /// Searches for a selected law without forcing the automatic engine's priority.
    pub fn rewrite_using(e: &expression::Expr, selected: law::Law) -> Option<Rewrite> {
        Self::rewrite_filtered(e, Some(selected), None)
    }

    /// Checks the actual transition, accepting a law in either direction and at any subtree.
    pub fn explains_step(
        before: &expression::Expr,
        after: &expression::Expr,
        selected: law::Law,
    ) -> bool {
        if before == after {
            return false;
        }
        let inside = match (before, after) {
            (expression::Expr::Not(a), expression::Expr::Not(b)) => {
                Self::explains_step(a, b, selected)
            }
            (expression::Expr::Bin(a, op, b), expression::Expr::Bin(c, next_op, d))
                if op == next_op =>
            {
                (a == c && Self::explains_step(b, d, selected))
                    || (b == d && Self::explains_step(a, c, selected))
            }
            _ => false,
        };
        inside
            || Self::rewrite_filtered(before, Some(selected), Some(after)).is_some()
            || Self::rewrite_filtered(after, Some(selected), Some(before)).is_some()
    }

    fn rewrite_filtered(
        e: &expression::Expr,
        selected: Option<law::Law>,
        expected: Option<&expression::Expr>,
    ) -> Option<Rewrite> {
        macro_rules! propose {
            ($candidate:expr) => {{
                let candidate = $candidate;
                if expected.is_none_or(|target| &candidate.e == target) {
                    return Some(candidate);
                }
            }};
        }
        let allows = |candidate| selected.is_none_or(|law| law == candidate);
        match e {
            expression::Expr::Not(a) => {
                if expected.is_none()
                    && let Some(r) = Self::rewrite_filtered(a, selected, None)
                {
                    propose!(Rewrite {
                        e: expression::not(r.e),
                        ..r
                    });
                }
                if let Some(v) = expression::constant(a)
                    && allows(law::Law::Complement)
                {
                    propose!(Rewrite {
                        e: expression::Expr::Const(!v),
                        law: law::Law::Complement,
                        explanation: law::Law::Complement.label(),
                    });
                }
                if let expression::Expr::Not(x) = a.as_ref()
                    && allows(law::Law::DoubleNegation)
                {
                    propose!(Rewrite {
                        e: (**x).clone(),
                        law: law::Law::DoubleNegation,
                        explanation: law::Law::DoubleNegation.label(),
                    });
                }
                if let expression::Expr::Bin(x, op, y) = a.as_ref()
                    && (*op == operation::Op::And || *op == operation::Op::Or)
                    && allows(law::Law::DeMorgan)
                {
                    propose!(Rewrite {
                        e: if *op == operation::Op::And {
                            expression::bin(
                                expression::not((**x).clone()),
                                operation::Op::Or,
                                expression::not((**y).clone()),
                            )
                        } else {
                            expression::bin(
                                expression::not((**x).clone()),
                                operation::Op::And,
                                expression::not((**y).clone()),
                            )
                        },
                        law: law::Law::DeMorgan,
                        explanation: law::Law::DeMorgan.label(),
                    });
                }
            }
            expression::Expr::Bin(a, op, b) => {
                if expected.is_none()
                    && let Some(r) = Self::rewrite_filtered(a, selected, None)
                {
                    propose!(Rewrite {
                        e: expression::bin(r.e, *op, (**b).clone()),
                        ..r
                    });
                }
                if expected.is_none()
                    && let Some(r) = Self::rewrite_filtered(b, selected, None)
                {
                    propose!(Rewrite {
                        e: expression::bin((**a).clone(), *op, r.e),
                        ..r
                    });
                }
                let av = expression::constant(a);
                let bv = expression::constant(b);
                if *op == operation::Op::And || *op == operation::Op::Or {
                    let and = *op == operation::Op::And;
                    let mut ts = Vec::new();
                    expression::terms(e, *op, &mut ts);
                    for i in 0..ts.len() {
                        for j in i + 1..ts.len() {
                            if expression::same(ts[i], ts[j]) && allows(law::Law::Idempotence) {
                                let mut q = ts.clone();
                                q.remove(j);
                                propose!(Rewrite {
                                    e: expression::rebuild(&q, *op),
                                    law: law::Law::Idempotence,
                                    explanation: law::Law::Idempotence.label(),
                                });
                            }
                            if expression::neg_of(ts[i], ts[j]) && allows(law::Law::Complement) {
                                propose!(Rewrite {
                                    e: expression::Expr::Const(!and),
                                    law: law::Law::Complement,
                                    explanation: law::Law::Complement.label(),
                                });
                            }
                        }
                    }

                    let opposite = if and {
                        operation::Op::Or
                    } else {
                        operation::Op::And
                    };
                    for (compound_index, compound) in ts.iter().enumerate() {
                        if operation::op_of(compound) != Some(opposite) {
                            continue;
                        }

                        let mut compound_terms = Vec::new();
                        expression::terms(compound, opposite, &mut compound_terms);
                        let is_absorbed = compound_terms.iter().any(|compound_term| {
                            ts.iter().enumerate().any(|(simple_index, simple)| {
                                simple_index != compound_index
                                    && expression::same(compound_term, simple)
                            })
                        });

                        if is_absorbed && allows(law::Law::Absorption) {
                            let mut remaining = ts.clone();
                            remaining.remove(compound_index);
                            propose!(Rewrite {
                                e: expression::rebuild(&remaining, *op),
                                law: law::Law::Absorption,
                                explanation: law::Law::Absorption.label(),
                            });
                        }
                    }
                    if ((and && (av == Some(false) || bv == Some(false)))
                        || (!and && (av == Some(true) || bv == Some(true))))
                        && allows(law::Law::Domination)
                    {
                        propose!(Rewrite {
                            e: expression::Expr::Const(!and),
                            law: law::Law::Domination,
                            explanation: law::Law::Domination.label(),
                        });
                    }
                    if ((and && av == Some(true)) || (!and && av == Some(false)))
                        && allows(law::Law::Identity)
                    {
                        propose!(Rewrite {
                            e: (**b).clone(),
                            law: law::Law::Identity,
                            explanation: law::Law::Identity.label(),
                        });
                    }
                    if ((and && bv == Some(true)) || (!and && bv == Some(false)))
                        && allows(law::Law::Identity)
                    {
                        propose!(Rewrite {
                            e: (**a).clone(),
                            law: law::Law::Identity,
                            explanation: law::Law::Identity.label(),
                        });
                    }
                    if expression::same(a, b) && allows(law::Law::Idempotence) {
                        propose!(Rewrite {
                            e: (**a).clone(),
                            law: law::Law::Idempotence,
                            explanation: law::Law::Idempotence.label(),
                        });
                    }
                    if expression::neg_of(a, b) && allows(law::Law::Complement) {
                        propose!(Rewrite {
                            e: expression::Expr::Const(!and),
                            law: law::Law::Complement,
                            explanation: law::Law::Complement.label(),
                        });
                    }
                    let nested = if operation::op_of(a)
                        == Some(if and {
                            operation::Op::Or
                        } else {
                            operation::Op::And
                        }) {
                        Some((a.as_ref(), b.as_ref()))
                    } else if operation::op_of(b)
                        == Some(if and {
                            operation::Op::Or
                        } else {
                            operation::Op::And
                        })
                    {
                        Some((b.as_ref(), a.as_ref()))
                    } else {
                        None
                    };
                    if let Some((compound, simple)) = nested
                        && let Some((x, y)) = expression::children(compound)
                        && (expression::same(simple, x) || expression::same(simple, y))
                        && allows(law::Law::Absorption)
                    {
                        propose!(Rewrite {
                            e: simple.clone(),
                            law: law::Law::Absorption,
                            explanation: law::Law::Absorption.label(),
                        });
                    }
                    if operation::op_of(a) == operation::op_of(b)
                        && allows(law::Law::Distributivity)
                        && operation::op_of(a)
                            == Some(if and {
                                operation::Op::Or
                            } else {
                                operation::Op::And
                            })
                        && let (Some((x, y)), Some((u, v))) =
                            (expression::children(a), expression::children(b))
                    {
                        for (common, l, other, r) in
                            [(x, y, u, v), (x, y, v, u), (y, x, u, v), (y, x, v, u)]
                        {
                            if !expression::same(common, other) {
                                continue;
                            }
                            propose!(Rewrite {
                                e: if and {
                                    expression::bin(
                                        common.clone(),
                                        operation::Op::Or,
                                        expression::bin(l.clone(), operation::Op::And, r.clone()),
                                    )
                                } else {
                                    expression::bin(
                                        common.clone(),
                                        operation::Op::And,
                                        expression::bin(l.clone(), operation::Op::Or, r.clone()),
                                    )
                                },
                                law: law::Law::Distributivity,
                                explanation: law::Law::Distributivity.label(),
                            });
                        }
                    }

                    // In manual work, distributing is valid as well as factoring.
                    if selected == Some(law::Law::Distributivity) {
                        for (compound, simple, compound_left) in [
                            (a.as_ref(), b.as_ref(), true),
                            (b.as_ref(), a.as_ref(), false),
                        ] {
                            if operation::op_of(compound) != Some(opposite) {
                                continue;
                            }
                            if let Some((x, y)) = expression::children(compound) {
                                let product = |term: &expression::Expr| {
                                    if compound_left {
                                        expression::bin(term.clone(), *op, simple.clone())
                                    } else {
                                        expression::bin(simple.clone(), *op, term.clone())
                                    }
                                };
                                propose!(Rewrite {
                                    e: expression::bin(product(x), opposite, product(y)),
                                    law: law::Law::Distributivity,
                                    explanation: law::Law::Distributivity.label(),
                                });
                            }
                        }
                    }
                }

                if let expression::Expr::Bin(middle, right_op, right) = b.as_ref()
                    && *right_op == *op
                    && allows(law::Law::Associativity)
                {
                    propose!(Rewrite {
                        e: expression::bin(
                            expression::bin((**a).clone(), *op, (**middle).clone()),
                            *op,
                            (**right).clone(),
                        ),
                        law: law::Law::Associativity,
                        explanation: law::Law::Associativity.label(),
                    });
                }

                // Manual hints accept regrouping in either direction.
                if selected == Some(law::Law::Associativity)
                    && let expression::Expr::Bin(left, left_op, middle) = a.as_ref()
                    && *left_op == *op
                {
                    propose!(Rewrite {
                        e: expression::bin(
                            (**left).clone(),
                            *op,
                            expression::bin((**middle).clone(), *op, (**b).clone()),
                        ),
                        law: law::Law::Associativity,
                        explanation: law::Law::Associativity.label(),
                    });
                }

                if allows(law::Law::Commutativity)
                    && ((selected == Some(law::Law::Commutativity) && a != b)
                        || (operation::op_of(a) != Some(*op)
                            && operation::op_of(b) != Some(*op)
                            && expression::ordering_key(a) > expression::ordering_key(b)))
                {
                    propose!(Rewrite {
                        e: expression::bin((**b).clone(), *op, (**a).clone()),
                        law: law::Law::Commutativity,
                        explanation: law::Law::Commutativity.label(),
                    });
                }
            }
            _ => {}
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::parser::Parser;
    use std::collections::HashMap;

    #[test]
    fn validates_the_changed_subtree_and_each_possible_root_match() {
        for (before, after, selected) in [
            ("(A + A) + (B + B)", "(A + A) + B", law::Law::Idempotence),
            ("A + A + B + B", "A + A + B", law::Law::Idempotence),
            ("A + B", "B + A", law::Law::Commutativity),
            ("A", "A & 1", law::Law::Identity),
            ("(A + B) & C", "(A & C) + (B & C)", law::Law::Distributivity),
            ("(A & B) + (B & A)", "B & (A + A)", law::Law::Distributivity),
        ] {
            let before = Parser::parse(before).unwrap();
            let after = Parser::parse(after).unwrap();
            assert!(
                Rewrite::explains_step(&before, &after, selected),
                "{before:?} -> {after:?}"
            );
        }
        let expression = Parser::parse("A + B").unwrap();
        assert!(!Rewrite::explains_step(
            &expression,
            &expression,
            law::Law::Commutativity
        ));
    }

    #[test]
    fn selected_laws_preserve_the_expression_for_every_input() {
        for (source, selected) in [
            ("(B + A) & 1", law::Law::Identity),
            ("A & 0", law::Law::Domination),
            ("A + A", law::Law::Idempotence),
            ("A + ~A", law::Law::Complement),
            ("~~A", law::Law::DoubleNegation),
            ("A + (B + C)", law::Law::Commutativity),
            ("(A + B) + C", law::Law::Associativity),
            ("A + (B + C)", law::Law::Associativity),
            ("A & (B + C)", law::Law::Distributivity),
            ("(A & B) + C", law::Law::Distributivity),
            ("(A & B) + (A & C)", law::Law::Distributivity),
            ("A + (A & B)", law::Law::Absorption),
            ("~(B + ~A)", law::Law::DeMorgan),
        ] {
            let before = Parser::parse(source).unwrap();
            let rewrite = Rewrite::rewrite_using(&before, selected).unwrap();
            assert_eq!(rewrite.law, selected);
            for bits in 0..8 {
                let inputs = HashMap::from([
                    ("A".into(), bits & 1 != 0),
                    ("B".into(), bits & 2 != 0),
                    ("C".into(), bits & 4 != 0),
                ]);
                assert_eq!(
                    before.evaluate(&inputs).unwrap(),
                    rewrite.e.evaluate(&inputs).unwrap(),
                    "{source}: {selected:?}, {inputs:?}"
                );
            }
        }
    }
}
