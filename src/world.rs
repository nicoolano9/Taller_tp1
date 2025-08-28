use std::fmt;
use crate::flatlander;
use crate::error::InputError;

const MIN_ANG: i32 = 10;
const MAX_ANG: i32 = 80;

const MIN_FLATLANDERS: u32 = 1;
const MAX_FLATLANDERS: u32 = 100_000;

pub struct World {
    light_ang: i32,
    flatlanders: Vec<flatlander::Flatlander>,
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
    pub fn new(ang: i32, capacity: u32) -> Result<Self, InputError> {
        if ang < MIN_ANG || ang > MAX_ANG {
            return Err(InputError::OutOfRange);
        }
        if capacity < MIN_FLATLANDERS || capacity > MAX_FLATLANDERS {
            return Err(InputError::OutOfRange);
        }

        Ok(Self {
            light_ang: ang,
            flatlanders: Vec::new(),
            capacity
        })
    }

    pub fn add_flatlander(&mut self, x: i32, hight: u32) -> Result<(), InputError> {
        if self.flatlanders.len() >= self.capacity as usize {
            return Err(InputError::OutOfRange);
        }
        match flatlander::Flatlander::new(x, hight) {
            Ok(flat) => {
                self.flatlanders.push(flat);
                Ok(())
            },
            Err(err) => Err(err)
        }
    }

    pub fn len_flatlanders(&self) -> u32 {
        self.flatlanders.len() as u32
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
        assert!(matches!(world, Ok(ref w) if w.light_ang == ang && w.capacity == capacity && w.flatlanders.len() == 0));
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
