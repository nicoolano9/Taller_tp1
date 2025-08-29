use std::io::{self, BufRead};

mod error;
mod flatlander;
mod shadow;
mod world;

/// Lee la primera línea de la entrada.
fn read_first_line<R: BufRead>(reader: &mut R) -> Result<String, error::InputError> {
    let mut lines = reader.lines();

    match lines.next() {
        Some(Ok(line)) => Ok(line),
        Some(Err(_)) => Err(error::InputError::Io),
        None => Err(error::InputError::Io),
    }
}

/// Obtiene los valores de entrada de una línea
/// Ignora los valores adicionales.
fn get_inputs(line: String) -> Result<(i32, u32), error::InputError> {
    let mut iter = line.split_whitespace();

    let x_str = match iter.next() {
        Some(str) => str,
        None => return Err(error::InputError::MissingValue),
    };

    let y_str = match iter.next() {
        Some(str) => str,
        None => return Err(error::InputError::MissingValue),
    };

    let x = match x_str.parse::<i32>() {
        Ok(v) => v,
        Err(_) => return Err(error::InputError::InvalidValue),
    };

    let y = match y_str.parse::<u32>() {
        Ok(v) => v,
        Err(_) => return Err(error::InputError::InvalidValue),
    };

    Ok((x, y))
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let first_line = match read_first_line(&mut reader) {
        Ok(line) => line,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let (ang, n) = match get_inputs(first_line) {
        Ok(values) => values,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let mut world = match world::World::new(ang, n) {
        Ok(world) => world,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                eprintln!("{}", error::InputError::Io);
                return;
            }
        };
        let (x, h) = match get_inputs(line) {
            Ok(values) => values,
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        };
        match world.add_flatlander(x, h) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        }
    }

    if world.len_flatlanders() < n {
        eprintln!("{}", error::InputError::MissingLine);
        return;
    }

    println!("{}", world.total_shadows_len());
}
