#[derive(Clone, Copy)]
pub enum Notation {
    Default,
    Logic,
    Mathematical,
    ProgBools,
    ProgBits,
    AltLogic,
    Latex,
}

#[allow(dead_code)]
pub fn parse_notation(s: &str) -> Result<Notation, String> {
    match s.to_ascii_lowercase().as_str() {
        "default" => Ok(Notation::Default),
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
