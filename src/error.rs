use std::fmt;

#[derive(Debug)]
pub enum InputError {
    /// - `Io`: Error de entrada/salida.
    Io,
    /// - `OutOfRange`: Un valor numérico está fuera del rango permitido.
    OutOfRange,
    /// - `MissingValue`: Falta un valor esperado en la entrada.
    MissingValue,
    /// - `InvalidValue`: Un valor no se pudo interpretar como un número válido.
    InvalidValue,
    /// - `MissingLine`: Falta una línea esperada en la entrada.
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
