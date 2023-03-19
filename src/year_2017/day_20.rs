use std::collections::HashMap;
use std::ops::{AddAssign, Sub};
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let particles = parse_particles(input)?;
    let min_acceleration = get_min_acceleration(particles);
    let min_velocity = get_min_secondary_coordinate(min_acceleration, Particle::velocity, Particle::acceleration);
    let min_position = get_min_secondary_coordinate(min_velocity, Particle::position, Particle::velocity);
    if min_position.is_empty() {
        return Err(AoCError::NoSolutionFoundError("Input was empty.".to_string()))
    }
    if min_position.len() > 1 {
        return Err(AoCError::MultipleSolutionsFoundError(
            "No definite closest to (0, 0, 0) particle found.".to_string()))
    }
    Ok(min_position[0].index.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut particles = parse_particles(input)?;
    particles = remove_collisions(particles);
    loop {
        step_particles(&mut particles);
        particles = remove_collisions(particles);
        if finished(&particles) {
            break
        }
    }
    Ok(particles.len().to_string())
}

fn parse_particles(input: &[String]) -> Result<Vec<Particle>, AoCError<String>> {
    input.iter()
        .enumerate()
        .map(|(index, line)| Particle::parse(line, index))
        .collect()
}

fn get_min_acceleration(mut particles: Vec<Particle>) -> Vec<Particle> {
    if particles.len() <= 1 {
        return particles
    }
    particles.sort_unstable_by_key(|p| p.acceleration.absolute_value());
    let min_acceleration = particles.first()
        .expect("Vec is not empty").acceleration.absolute_value();
    particles.into_iter()
        .filter(|p| p.acceleration.absolute_value() == min_acceleration)
        .collect()
}

fn get_min_secondary_coordinate(mut particles: Vec<Particle>, secondary: fn(&Particle)->Coordinate,
                                primary: fn(&Particle)->Coordinate)
    -> Vec<Particle> {
    if particles.len() <= 1 {
        return particles
    }
    particles.sort_unstable_by_key(
        |p| secondary(p).normalized_unified_value(&primary(p)));
    let tmp = particles.first().expect("Vec is not empty");
    let min_secondary = secondary(tmp).normalized_unified_value(&primary(tmp));
    particles.into_iter()
        .filter(|p|
            secondary(p).normalized_unified_value(&primary(p)) == min_secondary)
        .collect()
}

fn remove_collisions(particles: Vec<Particle>) -> Vec<Particle> {
    let mut counts = HashMap::new();
    for particle in particles.iter() {
        let count = counts.get(&particle.position).unwrap_or(&0);
        counts.insert(particle.position, count+1);
    }
    particles.into_iter().filter(|p| counts.get(&p.position) == Some(&1)).collect()
}

fn step_particles(particles: &mut [Particle]) {
    particles.iter_mut().for_each(Particle::step)
}

fn finished(particles: &[Particle]) -> bool {
    for (index, p0) in particles.iter().enumerate() {
        for p1 in particles[index+1..].iter() {
            let new_dist = p0.position.distance(&p1.position);
            let old_dist =
                (p0.position- p0.velocity).distance(&(p1.position- p1.velocity));
            if old_dist > new_dist {
                return false
            }
        }
    }
    true
}

#[derive(Debug)]
struct Particle {
    index: usize,
    position: Coordinate,
    velocity: Coordinate,
    acceleration: Coordinate,
}

impl Particle {
    fn parse(line: &str, index: usize) -> Result<Self, AoCError<String>> {
        let words = line.split(", ").collect::<Vec<_>>();
        if words.len() != 3 {
            return Err(AoCError::BadInputFormat(format!(
                "Parsing particle failed. Expected 'p=<<x>,<y>,<z>>, v=<<x>,<y>,<z>>, \
                a=<<x>,<y>,<z>>', found '{}'.", line)))
        }
        let position = Coordinate::parse(words[0])?;
        let velocity = Coordinate::parse(words[1])?;
        let acceleration = Coordinate::parse(words[2])?;

        Ok(Self{index, position, velocity, acceleration})
    }

    fn step(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }

    fn position(&self) -> Coordinate {
        self.position
    }

    fn velocity(&self) -> Coordinate {
        self.velocity
    }

    fn acceleration(&self) -> Coordinate {
        self.acceleration
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn parse(str: &str) -> Result<Self, AoCError<String>> {
        let mut words = str.split(',').collect::<Vec<_>>();
        words[0] = words[0][1..].strip_prefix("=<")
            .ok_or_else(|| AoCError::BadInputFormat(format!(
                "Parsing coordinate failed. Expected '[p|v|a]=<<x>,<y>,<z>>', found '{}'.", str)))?;
        words[2] = words[2].strip_suffix('>')
            .ok_or_else(|| AoCError::BadInputFormat(format!(
                "Parsing coordinate failed. Expected '[p|v|a]=<<x>,<y>,<z>>', found '{}'.", str)))?;
        let x = words[0].parse()
            .map_err(|e| AoCError::BadInputFormat(format!(
                "Parsing x value failed. Expected number, found '{}'. {}", words[0], e)))?;
        let y = words[1].parse()
            .map_err(|e| AoCError::BadInputFormat(format!(
                "Parsing y value failed. Expected number, found '{}'. {}", words[1], e)))?;
        let z = words[2].parse()
            .map_err(|e| AoCError::BadInputFormat(format!(
                "Parsing z value failed. Expected number, found '{}'. {}", words[2], e)))?;
        Ok(Self{x, y, z})
    }

    fn absolute_value(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    /// Coordinates are inverted if the corresponding other coordinate is negative.
    /// Then all coordinates of self are summed.
    /// The smallest value will correspond to the closest to (0, 0, 0) in the long term, iff
    /// self == velocity and other == acceleration or self == position and other == velocity.
    fn normalized_unified_value(&self, other: &Self) -> i32 {
        let x = if other.x < 0 {
            -self.x
        } else {
            self.x
        };
        let y = if other.y < 0 {
            -self.y
        } else {
            self.y
        };
        let z = if other.z < 0 {
            -self.z
        } else {
            self.z
        };
        x + y + z
    }

    fn distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) +
            self.y.abs_diff(other.y) +
            self.z.abs_diff(other.z)
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x-rhs.x;
        let y = self.y-rhs.y;
        let z = self.z-rhs.z;
        Self{x, y, z}
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>".to_string(),
            "p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>".to_string(),
        ];
        assert_eq!(part_1(&v), Ok("0".to_string()));
    }

    #[test]
    fn check_examples_part_1_edge_case() {
        let v = vec![
            "p=<-1,0,0>, v=<1,0,0>, a=<0,0,0>".to_string(),
            "p=<0,0,0>, v=<1,0,0>, a=<0,0,0>".to_string(),
        ];
        assert_eq!(part_1(&v), Ok("0".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_20.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("364".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = vec![
            "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>".to_string(),
            "p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>".to_string(),
            "p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>".to_string(),
            "p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>".to_string(),
        ];
        assert_eq!(part_2(&v), Ok("1".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_20.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("420".to_string()));
        Ok(())
    }
}