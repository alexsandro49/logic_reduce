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
                    return Some(Rewrite {
                        e: expression::not(r.e),
                        ..r
                    });
                }
                if let Some(v) = expression::constant(a) {
                    return Some(Rewrite {
                        e: expression::Expr::Const(!v),
                        law: law::Law::Inverse,
                        explanation: "¬0 = 1 e ¬1 = 0",
                    });
                }
                if let expression::Expr::Not(x) = a.as_ref() {
                    return Some(Rewrite {
                        e: (**x).clone(),
                        law: law::Law::Inverse,
                        explanation: "¬(¬A) = A",
                    });
                }
                if let Some((x, y)) = expression::children(a) {
                    if let expression::Expr::Bin(_, op, _) = a.as_ref() {
                        if *op == operation::Op::And || *op == operation::Op::Or {
                            return Some(Rewrite {
                                e: if *op == operation::Op::And {
                                    expression::bin(
                                        expression::not(x.clone()),
                                        operation::Op::Or,
                                        expression::not(y.clone()),
                                    )
                                } else {
                                    expression::bin(
                                        expression::not(x.clone()),
                                        operation::Op::And,
                                        expression::not(y.clone()),
                                    )
                                },
                                law: law::Law::Demorgans,
                                explanation: "regra de De Morgan",
                            });
                        }
                    }
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
                                    law: law::Law::Idempotent,
                                    explanation: "A · A = A",
                                });
                            }
                            if expression::neg_of(ts[i], ts[j]) {
                                return Some(Rewrite {
                                    e: expression::Expr::Const(!and),
                                    law: law::Law::Inverse,
                                    explanation: if and { "A · ¬A = 0" } else { "A + ¬A = 1" },
                                });
                            }
                        }
                    }
                    if (and && (av == Some(false) || bv == Some(false)))
                        || (!and && (av == Some(true) || bv == Some(true)))
                    {
                        return Some(Rewrite {
                            e: expression::Expr::Const(!and),
                            law: law::Law::Null,
                            explanation: if and { "A · 0 = 0" } else { "A + 1 = 1" },
                        });
                    }
                    if (and && av == Some(true)) || (!and && av == Some(false)) {
                        return Some(Rewrite {
                            e: (**b).clone(),
                            law: law::Law::Identity,
                            explanation: "regra de identidade",
                        });
                    }
                    if (and && bv == Some(true)) || (!and && bv == Some(false)) {
                        return Some(Rewrite {
                            e: (**a).clone(),
                            law: law::Law::Identity,
                            explanation: "regra de identidade",
                        });
                    }
                    if expression::same(a, b) {
                        return Some(Rewrite {
                            e: (**a).clone(),
                            law: law::Law::Idempotent,
                            explanation: "A · A = A",
                        });
                    }
                    if expression::neg_of(a, b) {
                        return Some(Rewrite {
                            e: expression::Expr::Const(!and),
                            law: law::Law::Inverse,
                            explanation: "complemento",
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
                    if let Some((compound, simple)) = nested {
                        if let Some((x, y)) = expression::children(compound) {
                            if expression::same(simple, x) || expression::same(simple, y) {
                                return Some(Rewrite {
                                    e: simple.clone(),
                                    law: law::Law::Absorption,
                                    explanation: "lei da absorção",
                                });
                            }
                        }
                    }
                    if operation::op_of(a) == operation::op_of(b)
                        && operation::op_of(a)
                            == Some(if and {
                                operation::Op::Or
                            } else {
                                operation::Op::And
                            })
                    {
                        if let (Some((x, y)), Some((u, v))) =
                            (expression::children(a), expression::children(b))
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
                                        expression::bin(
                                            common.clone(),
                                            operation::Op::Or,
                                            expression::bin(
                                                l.clone(),
                                                operation::Op::And,
                                                r.clone(),
                                            ),
                                        )
                                    } else {
                                        expression::bin(
                                            common.clone(),
                                            operation::Op::And,
                                            expression::bin(
                                                l.clone(),
                                                operation::Op::Or,
                                                r.clone(),
                                            ),
                                        )
                                    },
                                    law: law::Law::InverseDistributive,
                                    explanation: "distributividade inversa",
                                });
                            }
                        }
                    }
                }
                if *op == operation::Op::Xor || *op == operation::Op::Xnor {
                    if expression::same(a, b) {
                        return Some(Rewrite {
                            e: expression::Expr::Const(*op == operation::Op::Xnor),
                            law: law::Law::Xor,
                            explanation: "A ⊕ A = 0 (ou XNOR = 1)",
                        });
                    }
                    if av == Some(false) {
                        return Some(Rewrite {
                            e: (**b).clone(),
                            law: law::Law::Xor,
                            explanation: "A ⊕ 0 = A",
                        });
                    }
                    if bv == Some(false) {
                        return Some(Rewrite {
                            e: (**a).clone(),
                            law: law::Law::Xor,
                            explanation: "A ⊕ 0 = A",
                        });
                    }
                    if av == Some(true) {
                        return Some(Rewrite {
                            e: expression::not((**b).clone()),
                            law: law::Law::Xor,
                            explanation: "A ⊕ 1 = ¬A",
                        });
                    }
                    if bv == Some(true) {
                        return Some(Rewrite {
                            e: expression::not((**a).clone()),
                            law: law::Law::Xor,
                            explanation: "A ⊕ 1 = ¬A",
                        });
                    }
                }
            }
            _ => {}
        }
        None
    }
}
