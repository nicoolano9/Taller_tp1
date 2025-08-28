use std::fmt;

#[derive(Debug)]
pub enum InputError {
    Io,
    OutOfRange,
    MissingValue,
    InvalidValue,
    MissingLine,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io => write!(f, "Error: \"IO\""),
            Self::OutOfRange => write!(f, "Error: \"Fuera de rango\""),
            Self::MissingValue => write!(f, "Error: \"Valor faltante\""),
            Self::InvalidValue => write!(f, "Error: \"Numero invalido\""),
            Self::MissingLine => write!(f, "Error: \"Linea faltante\""),
        }
    }
}