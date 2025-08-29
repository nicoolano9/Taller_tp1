use crate::error::InputError;

const MIN_X: i32 = 0;
const MAX_X: i32 = 3 * 100_000;
const MIN_H: u32 = 1;
const MAX_H: u32 = 1000;

/// Representa a un habitante de del mundo plano con una posición y altura.
#[derive(Debug)]
pub struct Flatlander {
    x: i32,
    hight: u32,
}

impl Flatlander {
    /// Crea un nuevo `Flatlander`.
    ///
    /// # Argumentos
    ///
    /// * `x` - La posición en el eje x.
    /// * `hight` - La altura del Flatlander.
    ///
    /// # Errores
    ///
    /// Devuelve `InputError::OutOfRange` si `x` o `hight` están fuera de los rangos permitidos.
    pub fn new(x: i32, hight: u32) -> Result<Self, InputError> {
        if !(MIN_X..=MAX_X).contains(&x) {
            return Err(InputError::OutOfRange);
        }

        if !(MIN_H..=MAX_H).contains(&hight) {
            return Err(InputError::OutOfRange);
        }

        Ok(Self { x, hight })
    }

    /// Devuelve la posición del `Flatlander`.
    pub fn get_pos(&self) -> i32 {
        self.x
    }

    /// Devuelve la altura del `Flatlander`.
    pub fn get_hight(&self) -> u32 {
        self.hight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid_flatlander() {
        let result = Flatlander::new(MIN_X, MIN_H);
        assert!(matches!(result, Ok(ref f) if f.x == MIN_X && f.hight == MIN_H));

        let result = Flatlander::new(MAX_X, MAX_H);
        assert!(matches!(result, Ok(ref f) if f.x == MAX_X && f.hight == MAX_H));
    }

    #[test]
    fn test_new_invalid_x_below_min() {
        let result = Flatlander::new(MIN_X - 1, 10);
        assert!(matches!(result, Err(InputError::OutOfRange)));
    }

    #[test]
    fn test_new_invalid_x_above_max() {
        let result = Flatlander::new(MAX_X + 1, 10);
        assert!(matches!(result, Err(InputError::OutOfRange)));
    }

    #[test]
    fn test_new_invalid_hight_below_min() {
        let result = Flatlander::new(10, MIN_H - 1);
        assert!(matches!(result, Err(InputError::OutOfRange)));
    }

    #[test]
    fn test_new_invalid_hight_above_max() {
        let result = Flatlander::new(10, MAX_H + 1);
        assert!(matches!(result, Err(InputError::OutOfRange)));
    }

    #[test]
    fn test_new_invalid_both_out_of_range() {
        let result = Flatlander::new(MIN_X - 1, MAX_H + 1);
        assert!(matches!(result, Err(InputError::OutOfRange)));
    }
}
