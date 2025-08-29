use crate::flatlander;
use std::{cmp::Ordering, f64};

/// Representa una sombra proyectada por un `Flatlander`.
#[derive(Debug)]
pub struct Shadow {
    pub start: f64,
    pub end: f64,
}

impl Shadow {
    /// Crea una nueva `Shadow` a partir de un `Flatlander` y un ángulo de luz.
    ///
    /// # Argumentos
    ///
    /// * `flatlander` - Una referencia al `Flatlander` que proyecta la sombra.
    /// * `deg_ang` - El ángulo de la luz en grados.
    pub fn new(flatlander: &flatlander::Flatlander, deg_ang: &f64) -> Self {
        let rad_ang = deg_ang.to_radians();
        let tan = rad_ang.tan();
        let x = flatlander.get_pos();

        let res = if tan != 0.0 {
            flatlander.get_hight() as f64 / tan
        } else {
            f64::INFINITY
        };

        let (start, end) = if res >= 0.0 {
            (x as f64, x as f64 + res)
        } else {
            (x as f64 + res, x as f64)
        };

        Self { start, end }
    }

    /// Compara dos sombras para ordenarlas.
    ///
    /// El orden se basa primero en `start` y luego en `end`.
    pub fn compare(a: &Self, b: &Self) -> Ordering {
        match a.start.total_cmp(&b.start) {
            Ordering::Less => Ordering::Less,       // a.start < b.start
            Ordering::Greater => Ordering::Greater, // a.start > b.start
            Ordering::Equal => a.end.total_cmp(&b.end),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELTA_ERROR: f64 = 1e-4;

    #[test]
    fn test_shadow_new_positive_angle() {
        let angle = 45.0; // en grados
        let flatlander = flatlander::Flatlander::new(0, 10).unwrap();

        let shadow = Shadow::new(&flatlander, &angle);

        // tan(45) = 1, entonces res = 10 / 1 = 10
        assert!((shadow.start - 0.0).abs() < DELTA_ERROR);
        assert!((shadow.end - 10.0).abs() < DELTA_ERROR);
    }

    #[test]
    fn test_shadow_new_negative_angle() {
        // -45 grados en radianes
        let angle = -45.0; // en grados
        let flatlander = flatlander::Flatlander::new(5, 10).unwrap();

        let shadow = Shadow::new(&flatlander, &angle);

        // tan(-45°) = -1, entonces res = 10 / -1 = -10
        assert!((shadow.start - (-5.0)).abs() < DELTA_ERROR);
        assert!((shadow.end - 5.0).abs() < DELTA_ERROR);
    }

    #[test]
    fn test_shadow_new_zero_angle() {
        // 0 grados en radianes
        let angle = 0.0;
        let flatlander = flatlander::Flatlander::new(2, 10).unwrap();

        let shadow = Shadow::new(&flatlander, &angle);

        // tan(0) = 0, entonces res = inf
        assert!((shadow.start - 2.0).abs() < DELTA_ERROR);
        assert!(shadow.end.is_infinite());
    }
}
