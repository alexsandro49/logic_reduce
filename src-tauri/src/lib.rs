// use std::env;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expr {
    Var(String),
    Const(bool),
    Not(Box<Expr>),
    Bin(Box<Expr>, Op, Box<Expr>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    And,
    Or,
    Xor,
    Xnor,
}

fn not(e: Expr) -> Expr {
    Expr::Not(Box::new(e))
}
fn bin(a: Expr, op: Op, b: Expr) -> Expr {
    Expr::Bin(Box::new(a), op, Box::new(b))
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}
impl Parser {
    fn new(s: &str) -> Self {
        Self {
            chars: s.chars().collect(),
            pos: 0,
        }
    }
    fn parse(s: &str) -> Result<Expr, String> {
        if s.trim().is_empty() {
            return Err("expressão vazia".into());
        }
        let mut p = Self::new(s);
        let e = p.parse_or()?;
        p.spaces();
        if p.pos != p.chars.len() {
            return Err(p.error(&format!("símbolo inesperado '{}'", p.chars[p.pos])));
        }
        Ok(e)
    }
    fn spaces(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }
    fn take(&mut self, c: char) -> bool {
        self.spaces();
        if self.pos < self.chars.len() && self.chars[self.pos] == c {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn error(&self, s: &str) -> String {
        format!("{} na posição {}", s, self.pos)
    }
    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut r = self.parse_xor()?;
        while self.take('+') || self.take('|') {
            r = bin(r, Op::Or, self.parse_xor()?);
        }
        Ok(r)
    }
    fn parse_xor(&mut self) -> Result<Expr, String> {
        let mut r = self.parse_and()?;
        while self.take('^') {
            r = bin(r, Op::Xor, self.parse_and()?);
        }
        Ok(r)
    }
    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut r = self.parse_unary()?;
        loop {
            if self.take('.') || self.take('*') || self.take('&') {
                r = bin(r, Op::And, self.parse_unary()?);
            } else if self.starts_unary() {
                r = bin(r, Op::And, self.parse_unary()?);
            } else {
                return Ok(r);
            }
        }
    }
    fn starts_unary(&mut self) -> bool {
        self.spaces();
        self.pos < self.chars.len()
            && (matches!(self.chars[self.pos], '(' | '~' | '!' | '0' | '1' | '_')
                || self.chars[self.pos].is_alphabetic())
    }
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.take('~') || self.take('!') {
            return Ok(not(self.parse_unary()?));
        }
        if self.take('(') {
            let mut r = self.parse_or()?;
            if !self.take(')') {
                return Err(self.error("era esperado ')'"));
            }
            while self.take('\'') {
                r = not(r);
            }
            return Ok(r);
        }
        self.spaces();
        if self.pos >= self.chars.len() {
            return Err(self.error("era esperado um operando"));
        }
        let c = self.chars[self.pos];
        let mut r = if c == '0' || c == '1' {
            self.pos += 1;
            Expr::Const(c == '1')
        } else if c.is_alphabetic() || c == '_' {
            let start = self.pos;
            self.pos += 1;
            while self.pos < self.chars.len()
                && (self.chars[self.pos].is_alphanumeric() || self.chars[self.pos] == '_')
            {
                self.pos += 1;
            }
            Expr::Var(self.chars[start..self.pos].iter().collect())
        } else {
            return Err(self.error("era esperado um operando"));
        };
        while self.take('\'') {
            r = not(r);
        }
        Ok(r)
    }
}

#[derive(Clone, Copy)]
enum Notation {
    Default,
    Logic,
    Mathematical,
    ProgBools,
    ProgBits,
    AltLogic,
    Latex,
}
#[allow(unused)]
fn parse_notation(s: &str) -> Result<Notation, String> {
    match s.to_ascii_lowercase().as_str() {
        "logic" => Ok(Notation::Logic),
        "mathematical" => Ok(Notation::Mathematical),
        "progbools" => Ok(Notation::ProgBools),
        "progbits" => Ok(Notation::ProgBits),
        "altlogic" | "altlog" => Ok(Notation::AltLogic),
        "latex" => Ok(Notation::Latex),
        _ => Err(format!(
            "notação inválida: {}. Use logic, altlog, mathematical, latex, progbools ou progbits",
            s
        )),
    }
}
fn prec(op: Option<Op>) -> i32 {
    match op {
        None => 100,
        Some(Op::Or) => 1,
        Some(Op::Xor | Op::Xnor) => 2,
        Some(Op::And) => 3,
    }
}
fn op_of(e: &Expr) -> Option<Op> {
    if let Expr::Bin(_, o, _) = e {
        Some(*o)
    } else {
        None
    }
}
fn symbol(op: Op, n: Notation) -> &'static str {
    match n {
        Notation::Default => match op {
            Op::And => " & ",
            Op::Or => " + ",
            Op::Xor => " ^ ",
            Op::Xnor => " # ",
        },
        Notation::Logic => match op {
            Op::And => " ∧ ",
            Op::Or => " ∨ ",
            Op::Xor => " ⊻ ",
            Op::Xnor => " ≡ ",
        },
        Notation::Mathematical => match op {
            Op::And => " ⋅ ",
            Op::Or => " + ",
            Op::Xor => " ⊕ ",
            Op::Xnor => " ⊙ ",
        },
        Notation::ProgBools => match op {
            Op::And => " && ",
            Op::Or => " || ",
            Op::Xor => " ^ ",
            Op::Xnor => " == ",
        },
        Notation::ProgBits => match op {
            Op::And => " & ",
            Op::Or => " | ",
            Op::Xor => " ^ ",
            Op::Xnor => " ^~ ",
        },
        Notation::AltLogic => match op {
            Op::And => " ∧ ",
            Op::Or => " ∨ ",
            Op::Xor => " ≢ ",
            Op::Xnor => " ≡ ",
        },
        Notation::Latex => match op {
            Op::And => " \\cdot ",
            Op::Or => " + ",
            Op::Xor => " \\oplus ",
            Op::Xnor => " \\odot ",
        },
    }
}
fn format(e: &Expr, parent: Option<Op>, right: bool, n: Notation) -> String {
    match e {
        Expr::Var(s) => s.clone(),
        Expr::Const(v) => (if *v { "1" } else { "0" }).into(),
        Expr::Not(a) => {
            let mut v = format(a, Some(Op::And), false, n);
            if op_of(a).is_some() {
                v = format!("({})", v);
            }
            if matches!(n, Notation::Latex) {
                format!("\\overline{{{}}}", v)
            } else {
                format!(
                    "{}{}",
                    if matches!(n, Notation::Logic) {
                        "¬"
                    } else if matches!(n, Notation::ProgBools) {
                        "!"
                    } else {
                        "~"
                    },
                    v
                )
            }
        }
        Expr::Bin(a, o, b) => {
            let l = format(a, Some(*o), false, n);
            let r = format(b, Some(*o), true, n);
            let mut ls = l;
            let mut rs = r;
            if prec(op_of(a)) < prec(Some(*o)) || (*o == Op::Or && op_of(a) == Some(Op::And)) {
                ls = format!("({})", ls);
            }
            if prec(op_of(b)) < prec(Some(*o))
                || (*o == Op::Or && op_of(b) == Some(Op::And))
                || (right && op_of(b) == Some(*o) && *o != Op::And)
            {
                rs = format!("({})", rs);
            }
            let _ = parent;
            format!("{}{}{}", ls, symbol(*o, n), rs)
        }
    }
}

#[derive(Clone, Copy)]
enum Law {
    Identity,
    Null,
    Idempotent,
    Inverse,
    Absorption,
    InverseDistributive,
    Xor,
    Demorgans,
}
#[allow(dead_code)]
impl Law {
    fn name(self) -> &'static str {
        match self {
            Law::Identity => "IDENTITY",
            Law::Null => "NULL",
            Law::Idempotent => "IDEMPOTENT",
            Law::Inverse => "INVERSE",
            Law::Absorption => "ABSORPTION",
            Law::InverseDistributive => "INVERSE_DISTRIBUTIVE",
            Law::Xor => "XOR",
            Law::Demorgans => "DEMORGANS",
        }
    }
}
struct Rewrite {
    e: Expr,
    law: Law,
    explanation: &'static str,
}
fn children(e: &Expr) -> Option<(&Expr, &Expr)> {
    if let Expr::Bin(a, _, b) = e {
        Some((a, b))
    } else {
        None
    }
}
fn constant(e: &Expr) -> Option<bool> {
    if let Expr::Const(v) = e {
        Some(*v)
    } else {
        None
    }
}
fn same(a: &Expr, b: &Expr) -> bool {
    a == b
}
fn neg_of(a: &Expr, b: &Expr) -> bool {
    matches!(a,Expr::Not(x) if x.as_ref()==b) || matches!(b,Expr::Not(x) if x.as_ref()==a)
}
fn terms<'a>(e: &'a Expr, op: Op, out: &mut Vec<&'a Expr>) {
    if let Expr::Bin(a, o, b) = e {
        if *o == op {
            terms(a, op, out);
            terms(b, op, out);
            return;
        }
    }
    out.push(e)
}
fn rebuild(xs: &[&Expr], op: Op) -> Expr {
    xs.iter()
        .skip(1)
        .fold((*xs[0]).clone(), |a, b| bin(a, op, (*b).clone()))
}
fn rewrite_once(e: &Expr) -> Option<Rewrite> {
    match e {
        Expr::Not(a) => {
            if let Some(r) = rewrite_once(a) {
                return Some(Rewrite { e: not(r.e), ..r });
            }
            if let Some(v) = constant(a) {
                return Some(Rewrite {
                    e: Expr::Const(!v),
                    law: Law::Inverse,
                    explanation: "¬0 = 1 e ¬1 = 0",
                });
            }
            if let Expr::Not(x) = a.as_ref() {
                return Some(Rewrite {
                    e: (**x).clone(),
                    law: Law::Inverse,
                    explanation: "¬(¬A) = A",
                });
            }
            if let Some((x, y)) = children(a) {
                if let Expr::Bin(_, op, _) = a.as_ref() {
                    if *op == Op::And || *op == Op::Or {
                        return Some(Rewrite {
                            e: if *op == Op::And {
                                bin(not(x.clone()), Op::Or, not(y.clone()))
                            } else {
                                bin(not(x.clone()), Op::And, not(y.clone()))
                            },
                            law: Law::Demorgans,
                            explanation: "regra de De Morgan",
                        });
                    }
                }
            }
        }
        Expr::Bin(a, op, b) => {
            if let Some(r) = rewrite_once(a) {
                return Some(Rewrite {
                    e: bin(r.e, *op, (**b).clone()),
                    ..r
                });
            }
            if let Some(r) = rewrite_once(b) {
                return Some(Rewrite {
                    e: bin((**a).clone(), *op, r.e),
                    ..r
                });
            }
            let av = constant(a);
            let bv = constant(b);
            if *op == Op::And || *op == Op::Or {
                let and = *op == Op::And;
                let mut ts = Vec::new();
                terms(e, *op, &mut ts);
                for i in 0..ts.len() {
                    for j in i + 1..ts.len() {
                        if same(ts[i], ts[j]) {
                            let mut q = ts.clone();
                            q.remove(j);
                            return Some(Rewrite {
                                e: rebuild(&q, *op),
                                law: Law::Idempotent,
                                explanation: "A · A = A",
                            });
                        }
                        if neg_of(ts[i], ts[j]) {
                            return Some(Rewrite {
                                e: Expr::Const(!and),
                                law: Law::Inverse,
                                explanation: if and { "A · ¬A = 0" } else { "A + ¬A = 1" },
                            });
                        }
                    }
                }
                if (and && (av == Some(false) || bv == Some(false)))
                    || (!and && (av == Some(true) || bv == Some(true)))
                {
                    return Some(Rewrite {
                        e: Expr::Const(!and),
                        law: Law::Null,
                        explanation: if and { "A · 0 = 0" } else { "A + 1 = 1" },
                    });
                }
                if (and && av == Some(true)) || (!and && av == Some(false)) {
                    return Some(Rewrite {
                        e: (**b).clone(),
                        law: Law::Identity,
                        explanation: "regra de identidade",
                    });
                }
                if (and && bv == Some(true)) || (!and && bv == Some(false)) {
                    return Some(Rewrite {
                        e: (**a).clone(),
                        law: Law::Identity,
                        explanation: "regra de identidade",
                    });
                }
                if same(a, b) {
                    return Some(Rewrite {
                        e: (**a).clone(),
                        law: Law::Idempotent,
                        explanation: "A · A = A",
                    });
                }
                if neg_of(a, b) {
                    return Some(Rewrite {
                        e: Expr::Const(!and),
                        law: Law::Inverse,
                        explanation: "complemento",
                    });
                }
                let nested = if op_of(a) == Some(if and { Op::Or } else { Op::And }) {
                    Some((a.as_ref(), b.as_ref()))
                } else if op_of(b) == Some(if and { Op::Or } else { Op::And }) {
                    Some((b.as_ref(), a.as_ref()))
                } else {
                    None
                };
                if let Some((compound, simple)) = nested {
                    if let Some((x, y)) = children(compound) {
                        if same(simple, x) || same(simple, y) {
                            return Some(Rewrite {
                                e: simple.clone(),
                                law: Law::Absorption,
                                explanation: "lei da absorção",
                            });
                        }
                    }
                }
                if op_of(a) == op_of(b) && op_of(a) == Some(if and { Op::Or } else { Op::And }) {
                    if let (Some((x, y)), Some((u, v))) = (children(a), children(b)) {
                        let c = if same(x, u) {
                            Some((x, y, v))
                        } else if same(x, v) {
                            Some((x, y, u))
                        } else if same(y, u) {
                            Some((y, x, v))
                        } else if same(y, v) {
                            Some((y, x, u))
                        } else {
                            None
                        };
                        if let Some((common, l, r)) = c {
                            return Some(Rewrite {
                                e: if and {
                                    bin(common.clone(), Op::Or, bin(l.clone(), Op::And, r.clone()))
                                } else {
                                    bin(common.clone(), Op::And, bin(l.clone(), Op::Or, r.clone()))
                                },
                                law: Law::InverseDistributive,
                                explanation: "distributividade inversa",
                            });
                        }
                    }
                }
            }
            if *op == Op::Xor || *op == Op::Xnor {
                if same(a, b) {
                    return Some(Rewrite {
                        e: Expr::Const(*op == Op::Xnor),
                        law: Law::Xor,
                        explanation: "A ⊕ A = 0 (ou XNOR = 1)",
                    });
                }
                if av == Some(false) {
                    return Some(Rewrite {
                        e: (**b).clone(),
                        law: Law::Xor,
                        explanation: "A ⊕ 0 = A",
                    });
                }
                if bv == Some(false) {
                    return Some(Rewrite {
                        e: (**a).clone(),
                        law: Law::Xor,
                        explanation: "A ⊕ 0 = A",
                    });
                }
                if av == Some(true) {
                    return Some(Rewrite {
                        e: not((**b).clone()),
                        law: Law::Xor,
                        explanation: "A ⊕ 1 = ¬A",
                    });
                }
                if bv == Some(true) {
                    return Some(Rewrite {
                        e: not((**a).clone()),
                        law: Law::Xor,
                        explanation: "A ⊕ 1 = ¬A",
                    });
                }
            }
        }
        _ => {}
    }
    None
}
fn simplify(mut e: Expr) -> (Expr, Vec<(Expr, Expr, Law, &'static str)>) {
    let mut s = Vec::new();
    for _ in 0..1000 {
        if let Some(r) = rewrite_once(&e) {
            s.push((e, r.e.clone(), r.law, r.explanation));
            e = r.e
        } else {
            return (e, s);
        }
    }
    panic!("simplificação não convergiu")
}

#[tauri::command]
fn simplify_helper(source: &str) -> String{
    match Parser::parse(&source){
        Ok(parsed) => {
            let (result, _) = simplify(parsed.clone());
            return format(&result, None, false, Notation::Default);
        }
        Err(e) => {
            return format!("Erro: {}", e);
        }
    }
    
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![simplify_helper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
