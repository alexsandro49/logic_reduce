#[derive(Clone, Copy)]
pub enum Law {
    Identity,
    Null,
    Idempotent,
    Inverse,
    Absorption,
    InverseDistributive,
    Xor,
    Demorgans,
}

impl Law {
    #[allow(unused)]
    pub fn name(self) -> &'static str {
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
