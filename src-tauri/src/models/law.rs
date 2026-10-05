#[derive(Clone, Copy)]
pub enum Law {
    Identity,
    Domination,
    Idempotence,
    Complement,
    DoubleNegation,
    Commutativity,
    Associativity,
    Distributivity,
    Absorption,
    DeMorgan,
}

impl Law {
    pub fn label(self) -> &'static str {
        match self {
            Law::Identity => "Identidade",
            Law::Domination => "Dominação",
            Law::Idempotence => "Idempotência",
            Law::Complement => "Complemento",
            Law::DoubleNegation => "D. Negação",
            Law::Commutativity => "Comutativa",
            Law::Associativity => "Associativa",
            Law::Distributivity => "Distributiva",
            Law::Absorption => "Absorção",
            Law::DeMorgan => "De Morgan",
        }
    }
}
