use crate::error::InputError;
use crate::flatlander;
use crate::shadow;
use std::fmt;

const MIN_ANG: i32 = 10;
const MAX_ANG: i32 = 80;

const MIN_FLATLANDERS: u32 = 1;
const MAX_FLATLANDERS: u32 = 100_000;

/// Representa el mundo con sus habitantes y la luz.
pub struct World {
    light_ang: i32,
    flatlanders: Vec<flatlander::Flatlander>,
    shadows: Vec<shadow::Shadow>,
    capacity: u32,
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("World")
            .field("flatlanders", &self.flatlanders.len())
            .field("lightAng", &self.light_ang)
            .finish()
    }
}

impl World {
    /// Crea un nuevo `World`.
    ///
    /// # Argumentos
    ///
    /// * `ang` - El ángulo de la luz en grados.
    /// * `capacity` - El número máximo de `Flatlanders` que puede haber en el mundo.
    ///
    /// # Errores
    ///
    /// Devuelve `InputError::OutOfRange` si `ang` o `capacity` están fuera de los rangos permitidos.
    pub fn new(ang: i32, capacity: u32) -> Result<Self, InputError> {
        if !(MIN_ANG..=MAX_ANG).contains(&ang) {
            return Err(InputError::OutOfRange);
        }
        if !(MIN_FLATLANDERS..=MAX_FLATLANDERS).contains(&capacity) {
            return Err(InputError::OutOfRange);
        }

        Ok(Self {
            light_ang: ang,
            flatlanders: Vec::new(),
            shadows: Vec::new(),
            capacity,
        })
    }

    /// Añade un `Flatlander` al mundo.
    ///
    /// # Argumentos
    ///
    /// * `x` - La posición del nuevo `Flatlander`.
    /// * `hight` - La altura del nuevo `Flatlander`.
    ///
    /// # Errores
    ///
    /// Devuelve `InputError::OutOfRange` si el mundo ya está lleno o si los datos del `Flatlander` son inválidos.
    pub fn add_flatlander(&mut self, x: i32, hight: u32) -> Result<(), InputError> {
        if self.flatlanders.len() >= self.capacity as usize {
            return Err(InputError::OutOfRange);
        }
        match flatlander::Flatlander::new(x, hight) {
            Ok(flat) => {
                let light_ang_f64 = self.light_ang as f64;

                self.shadows
                    .push(shadow::Shadow::new(&flat, &light_ang_f64));

                self.flatlanders.push(flat);

                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    /// Devuelve el número actual de `Flatlanders` en el mundo.
    pub fn len_flatlanders(&self) -> u32 {
        self.flatlanders.len() as u32
    }

    /// Calcula la longitud total de todas las sombras proyectadas, fusionando las que se solapan.
    pub fn total_shadows_len(&mut self) -> f64 {
        if self.shadows.is_empty() {
            return 0.0;
        }

        self.shadows.sort_by(shadow::Shadow::compare);

        let mut merged_overlaped: Vec<shadow::Shadow> = Vec::new();
        let mut act_start = self.shadows[0].start;
        let mut act_end = self.shadows[0].end;

        for shadow in &self.shadows[1..] {
            if shadow.start <= act_end {
                act_end = act_end.max(shadow.end);
            } else {
                merged_overlaped.push(shadow::Shadow {
                    start: act_start,
                    end: act_end,
                });
                act_start = shadow.start;
                act_end = shadow.end;
            }
        }

        merged_overlaped.push(shadow::Shadow {
            start: act_start,
            end: act_end,
        });

        merged_overlaped.iter().map(|s| s.end - s.start).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_world_valid_params() {
        let ang = (MIN_ANG + MAX_ANG) / 2;
        let capacity = (MIN_FLATLANDERS + MAX_FLATLANDERS) / 2;
        let world = World::new(ang, capacity);
        assert!(
            matches!(world, Ok(ref w) if w.light_ang == ang && w.capacity == capacity && w.flatlanders.len() == 0)
        );
    }

    #[test]
    fn test_new_world_angle_too_low() {
        let ang = MIN_ANG - 1;
        let capacity = (MIN_FLATLANDERS + MAX_FLATLANDERS) / 2;
        let world = World::new(ang, capacity);
        assert!(matches!(world, Err(InputError::OutOfRange)));
    }

    #[test]
    fn test_new_world_angle_too_high() {
        let ang = MAX_ANG + 1;
        let capacity = (MIN_FLATLANDERS + MAX_FLATLANDERS) / 2;
        let world = World::new(ang, capacity);
        assert!(matches!(world, Err(InputError::OutOfRange)));
    }

    #[test]
    fn test_new_world_capacity_too_low() {
        let ang = (MIN_ANG + MAX_ANG) / 2;
        let capacity = MIN_FLATLANDERS - 1;
        let world = World::new(ang, capacity);
        assert!(matches!(world, Err(InputError::OutOfRange)));
    }

    #[test]
    fn test_new_world_capacity_too_high() {
        let ang = (MIN_ANG + MAX_ANG) / 2;
        let capacity = MAX_FLATLANDERS + 1;
        let world = World::new(ang, capacity);
        assert!(matches!(world, Err(InputError::OutOfRange)));
    }

    #[test]
    fn test_new_world_min_angle_and_capacity() {
        let ang = MIN_ANG;
        let capacity = MIN_FLATLANDERS;
        let world = World::new(ang, capacity);
        assert!(matches!(world, Ok(_)));
    }

    #[test]
    fn test_new_world_max_angle_and_capacity() {
        let ang = MAX_ANG;
        let capacity = MAX_FLATLANDERS;
        let world = World::new(ang, capacity);
        assert!(matches!(world, Ok(_)));
    }
}
