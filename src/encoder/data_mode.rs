#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, PartialOrd)]
pub enum DataMode {
    None(u8),
    Numeric(u8),
    Alphanumeric(u8),
    Byte(u8),
}

impl DataMode {
    pub fn none() -> Self {
        DataMode::None(1)
    }

    pub fn numeric() -> Self {
        DataMode::Numeric(2)
    }

    pub fn alphanumeric() -> Self {
        DataMode::Alphanumeric(4)
    }

    pub fn byte() -> Self {
        DataMode::Byte(8)
    }

    pub fn value(&self) -> u8 {
        match self {
            DataMode::None(n) => *n,
            DataMode::Numeric(n) => *n,
            DataMode::Alphanumeric(n) => *n,
            DataMode::Byte(n) => *n,
        }
    }
}