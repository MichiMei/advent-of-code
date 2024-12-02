use crate::errors::{AoCError, AoCResult};
use crate::geometrics::{Point, Point3D};

pub fn part_1(input: &[String]) -> AoCResult<String> {
    let points_and_velocity = input_to_points_and_velocity(input)?;
    let equations = points_and_velocity_to_equations(&points_and_velocity)
        .ok_or_else(|| AoCError::NoSolutionFoundError("Creating equation failed".to_string()))?;
    let min = 200000000000000.0;
    let max = 400000000000000.0;
    let area = ((min, min), (max, max));
    let count = count_intersecting_in(&equations, area);
    Ok(count.to_string())
}

pub fn part_2(_input: &Vec<String>) -> AoCResult<String> {
    todo!()
}

fn input_to_points_and_velocity(input: &[String]) -> AoCResult<Vec<(Point<i64>, Point<i64>)>> {
    let points_and_velocity = input.iter()
        .map(|line| parse_line(line))
        .collect::<AoCResult<Vec<_>>>()?;
    Ok(points_and_velocity.into_iter()
        .map(|(p0, p1)| points_3d_to_2d(p0, p1))
        .collect())
}

fn points_and_velocity_to_equations(points: &[(Point<i64>, Point<i64>)]) -> Option<Vec<Equation>> {
    points.iter()
        .map(|(p, v)| Equation::from_point_and_velocity(p, v))
        .collect()
}

fn parse_line(line: &str) -> AoCResult<(Point3D<i64>, Point3D<i64>)> {
    let split = line.split(" @ ").collect::<Vec<_>>();
    if split.len() != 2 {
        return Err(AoCError::BadInputFormat(
            format!("Parsing line failed, expected 'x, y, z @ vx, vy, vz'. Found '{}'.", line)))
    }
    let p0 = parse_point(split[0])?;
    let velocity = parse_point(split[1])?;
    Ok((p0, velocity))
}

fn parse_point(str: &str) -> AoCResult<Point3D<i64>> {
    let split = str.split(", ").collect::<Vec<_>>();
    if split.len() != 3 {
        return Err(AoCError::BadInputFormat(
            format!("Parsing point failed, expected 'x, y, z'. Found '{}'", str)))
    }
    let x = split[0].trim().parse()
        .map_err(|e| AoCError::BadInputFormat(
            format!("Parsing number failed, expected number, found '{}'. {}", split[0], e)))?;
    let y = split[1].trim().parse()
        .map_err(|e| AoCError::BadInputFormat(
            format!("Parsing number failed, expected number, found '{}'. {}", split[1], e)))?;
    let z = split[2].trim().parse()
        .map_err(|e| AoCError::BadInputFormat(
            format!("Parsing number failed, expected number, found '{}'. {}", split[2], e)))?;
    Ok(Point3D {
        x,
        y,
        z,
    })
}

fn points_3d_to_2d(p0: Point3D<i64>, p1: Point3D<i64>) -> (Point<i64>, Point<i64>) {
    let p0 = (p0.x, p0.y);
    let p1 = (p1.x, p1.y);
    (p0, p1)
}

fn count_intersecting_in(list: &[Equation], area: (Point<f64>, Point<f64>)) -> usize {
    let mut count = 0;
    for index0 in 0..list.len() {
        for index1 in index0+1..list.len() {
//            print!("checking {} and {}...", index0, index1);
            let intersection = list[index0].intersect_after_x(&list[index1]);
            if let Some(point) = intersection {
                if point.0 >= area.0.0 && point.0 <= area.1.0 &&
                    point.1 >= area.0.1 && point.1 <= area.1.1 {
                    count += 1;
//                    println!("intersection inside at ({}, {})", point.0, point.1);
                } else {
//                    println!("intersection outside at ({}, {})", point.0, point.1);
                }
            } else {
//                println!("no intersection");
            }
        }
    }
    count
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Equation {
    p_start: Point<i64>,
    p_velocity: Point<i64>,
    m: f64,
    t: f64,
}

impl Equation {
    fn from_point_and_velocity(p: &Point<i64>, v: &Point<i64>) -> Option<Self> {
        let p1 = Self::point_from_velocity(p, v);
        let (m, t) = Self::m_and_t_from_points(p, &p1)?;
        Some(Self {
            p_start: *p,
            p_velocity: *v,
            m,
            t,
        })
    }

    fn point_from_velocity(p: &Point<i64>, v: &Point<i64>) -> Point<i64> {
        (p.0 + v.0, p.1 + v.1)
    }

    fn m_and_t_from_points(p0: &Point<i64>, p1: &Point<i64>) -> Option<(f64, f64)> {
        if p0 == p1 {
            return None
        }
        let dx = (p0.0-p1.0) as f64;
        let dy = (p0.1-p1.1) as f64;
        let m = dy / dx;
        let t = p0.1 as f64 - m * p0.0 as f64;
        Some((m, t))
    }

    fn intersect(&self, other: &Self) -> Option<Point<f64>> {
        if self.m == other.m {
            return None
        }
        let x = (self.t-other.t)/(other.m-self.m);
        let y = self.solve(x);

        Some((
            x,
            y,
        ))
    }

    fn intersect_after_x(&self, other: &Self) -> Option<Point<f64>> {
        let intersection = self.intersect(other)?;
        if self.is_in_future(&intersection) && other.is_in_future(&intersection) {
            Some(intersection)
        } else {
            None
        }
    }

    fn is_in_future(&self, p: &Point<f64>) -> bool {
        let start = (self.p_start.0 as f64, self.p_start.1 as f64);
        let diff = (p.0 - start.0, p.1 - start.1);
        if self.p_velocity.0.signum() as f64 == diff.0.signum() &&
            self.p_velocity.1.signum() as f64 == diff.1.signum() {
            true
        } else {
            false
        }
    }

    fn solve(&self, x: f64) -> f64 {
        self.m * x + self.t
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "19, 13, 30 @ -2,  1, -2".to_string(),
            "18, 19, 22 @ -1, -1, -2".to_string(),
            "20, 25, 34 @ -2, -2, -4".to_string(),
            "12, 31, 28 @ -1, -2, -1".to_string(),
            "20, 19, 15 @  1, -5, -3".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() -> AoCResult<()> {
        let input = get_example_input();
        let points = input_to_points_and_velocity(&input)?;
        let equations = points_and_velocity_to_equations(&points)
            .ok_or_else(|| AoCError::NoSolutionFoundError("Creating Equation failed".to_string()))?;
        let area = ((7.0, 7.0), (27.0, 27.0));
        assert_eq!(count_intersecting_in(&equations, area), 2);
        Ok(())
    }

    #[test]
    fn check_input_part_1() -> AoCResult<()> {
        let input = get_input(2023, 24)?;
        assert_eq!(part_1(&input), Ok("14799".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["input".to_string()]), Ok("expected".to_string()));
    }

    #[test]
    fn check_input_part_2() -> AoCResult<()> {
        let input = get_input(2023, 24)?;
        assert_eq!(part_2(&input), Ok("expected".to_string())); // TODO
        Ok(())
    }
}