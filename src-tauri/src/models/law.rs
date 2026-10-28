#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pub const ALL: [Self; 10] = [
        Self::Identity,
        Self::Domination,
        Self::Idempotence,
        Self::Complement,
        Self::DoubleNegation,
        Self::Commutativity,
        Self::Associativity,
        Self::Distributivity,
        Self::Absorption,
        Self::DeMorgan,
    ];

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "identity" => Some(Self::Identity),
            "null" => Some(Self::Domination),
            "idempotent" => Some(Self::Idempotence),
            "complement" => Some(Self::Complement),
            "double_negative" => Some(Self::DoubleNegation),
            "commutative" => Some(Self::Commutativity),
            "associative" => Some(Self::Associativity),
            "distributive" => Some(Self::Distributivity),
            "absorption" => Some(Self::Absorption),
            "de_morgan" => Some(Self::DeMorgan),
            _ => None,
        }
    }

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
