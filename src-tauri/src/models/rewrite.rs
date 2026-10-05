use crate::models::{expression, law, operation};

pub struct Rewrite {
    pub e: expression::Expr,
    pub law: law::Law,
    pub explanation: &'static str,
}

impl Rewrite {
    pub fn rewrite_once(e: &expression::Expr) -> Option<Rewrite> {
        match e {
            expression::Expr::Not(a) => {
                if let Some(r) = Self::rewrite_once(a) {
                    return Some(Rewrite { e: expression::not(r.e), ..r });
                }
                if let Some(v) = expression::constant(a) {
                    return Some(Rewrite {
                        e: expression::Expr::Const(!v),
                        law: law::Law::Complement,
                        explanation: law::Law::Complement.label(),
                    });
                }
                if let expression::Expr::Not(x) = a.as_ref() {
                    return Some(Rewrite {
                        e: (**x).clone(),
                        law: law::Law::DoubleNegation,
                        explanation: law::Law::DoubleNegation.label(),
                    });
                }
                if let expression::Expr::Bin(x, op, y) = a.as_ref()
                    && (*op == operation::Op::And || *op == operation::Op::Or)
                {
                    return Some(Rewrite {
                        e: if *op == operation::Op::And {
                            expression::bin(expression::not((**x).clone()), operation::Op::Or, expression::not((**y).clone()))
                        } else {
                            expression::bin(expression::not((**x).clone()), operation::Op::And, expression::not((**y).clone()))
                        },
                        law: law::Law::DeMorgan,
                        explanation: law::Law::DeMorgan.label(),
                    });
                }
            }
            expression::Expr::Bin(a, op, b) => {
                if let Some(r) = Self::rewrite_once(a) {
                    return Some(Rewrite {
                        e: expression::bin(r.e, *op, (**b).clone()),
                        ..r
                    });
                }
                if let Some(r) = Self::rewrite_once(b) {
                    return Some(Rewrite {
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
                            if expression::same(ts[i], ts[j]) {
                                let mut q = ts.clone();
                                q.remove(j);
                                return Some(Rewrite {
                                    e: expression::rebuild(&q, *op),
                                    law: law::Law::Idempotence,
                                    explanation: law::Law::Idempotence.label(),
                                });
                            }
                            if expression::neg_of(ts[i], ts[j]) {
                                return Some(Rewrite {
                                    e: expression::Expr::Const(!and),
                                    law: law::Law::Complement,
                                    explanation: law::Law::Complement.label(),
                                });
                            }
                        }
                    }

                    let opposite = if and { operation::Op::Or } else { operation::Op::And };
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

                        if is_absorbed {
                            let mut remaining = ts.clone();
                            remaining.remove(compound_index);
                            return Some(Rewrite {
                                e: expression::rebuild(&remaining, *op),
                                law: law::Law::Absorption,
                                explanation: law::Law::Absorption.label(),
                            });
                        }
                    }
                    if (and && (av == Some(false) || bv == Some(false)))
                        || (!and && (av == Some(true) || bv == Some(true)))
                    {
                        return Some(Rewrite {
                            e: expression::Expr::Const(!and),
                            law: law::Law::Domination,
                            explanation: law::Law::Domination.label(),
                        });
                    }
                    if (and && av == Some(true)) || (!and && av == Some(false)) {
                        return Some(Rewrite {
                            e: (**b).clone(),
                            law: law::Law::Identity,
                            explanation: law::Law::Identity.label(),
                        });
                    }
                    if (and && bv == Some(true)) || (!and && bv == Some(false)) {
                        return Some(Rewrite {
                            e: (**a).clone(),
                            law: law::Law::Identity,
                            explanation: law::Law::Identity.label(),
                        });
                    }
                    if expression::same(a, b) {
                        return Some(Rewrite {
                            e: (**a).clone(),
                            law: law::Law::Idempotence,
                            explanation: law::Law::Idempotence.label(),
                        });
                    }
                    if expression::neg_of(a, b) {
                        return Some(Rewrite {
                            e: expression::Expr::Const(!and),
                            law: law::Law::Complement,
                            explanation: law::Law::Complement.label(),
                        });
                    }
                    let nested = if operation::op_of(a) == Some(if and { operation::Op::Or } else { operation::Op::And }) {
                        Some((a.as_ref(), b.as_ref()))
                    } else if operation::op_of(b) == Some(if and { operation::Op::Or } else { operation::Op::And }) {
                        Some((b.as_ref(), a.as_ref()))
                    } else {
                        None
                    };
                    if let Some((compound, simple)) = nested
                        && let Some((x, y)) = expression::children(compound)
                        && (expression::same(simple, x) || expression::same(simple, y))
                    {
                        return Some(Rewrite {
                            e: simple.clone(),
                            law: law::Law::Absorption,
                            explanation: law::Law::Absorption.label(),
                        });
                    }
                    if operation::op_of(a) == operation::op_of(b)
                        && operation::op_of(a) == Some(if and { operation::Op::Or } else { operation::Op::And })
                        && let (Some((x, y)), Some((u, v))) = (expression::children(a), expression::children(b))
                    {
                        let c = if expression::same(x, u) {
                            Some((x, y, v))
                        } else if expression::same(x, v) {
                            Some((x, y, u))
                        } else if expression::same(y, u) {
                            Some((y, x, v))
                        } else if expression::same(y, v) {
                            Some((y, x, u))
                        } else {
                            None
                        };
                        if let Some((common, l, r)) = c {
                            return Some(Rewrite {
                                e: if and {
                                    expression::bin(common.clone(), operation::Op::Or, expression::bin(l.clone(), operation::Op::And, r.clone()))
                                } else {
                                    expression::bin(common.clone(), operation::Op::And, expression::bin(l.clone(), operation::Op::Or, r.clone()))
                                },
                                law: law::Law::Distributivity,
                                explanation: law::Law::Distributivity.label(),
                            });
                        }
                    }
                }

                if let expression::Expr::Bin(middle, right_op, right) = b.as_ref()
                    && *right_op == *op
                {
                    return Some(Rewrite {
                        e: expression::bin(
                            expression::bin((**a).clone(), *op, (**middle).clone()),
                            *op,
                            (**right).clone(),
                        ),
                        law: law::Law::Associativity,
                        explanation: law::Law::Associativity.label(),
                    });
                }
                
                if operation::op_of(a) != Some(*op)
                    && operation::op_of(b) != Some(*op)
                    && expression::ordering_key(a) > expression::ordering_key(b)
                {
                    return Some(Rewrite {
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
