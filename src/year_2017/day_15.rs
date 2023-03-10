use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let (seed0, seed1) = parse_input(input)?;
    let gen0 =
        RandomGenerator::new(seed0, 16807, 2147483647, Some(40000000));
    let gen1 =
        RandomGenerator::new(seed1, 48271, 2147483647, Some(40000000));
    Ok(compare_random_generators(Box::new(gen0), Box::new(gen1)).to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let (seed0, seed1) = parse_input(input)?;
    let gen0 =
        RandomGeneratorTailingZeros::new(seed0, 16807, 2147483647,
                                         5000000, 2);
    let gen1 =
        RandomGeneratorTailingZeros::new(seed1, 48271, 2147483647,
                                         5000000, 3);
    Ok(compare_random_generators(Box::new(gen0), Box::new(gen1)).to_string())
}

fn parse_input(input: &Vec<String>) -> Result<(u64, u64), AoCError<String>> {
    if input.len() != 2 {
        return Err(AoCError::UnexpectedInputLength("Expected exactly two lines containing the seed \
        for the random generators.".to_string()))
    }
    let gen0 = if let Some(word) = input[0].split_whitespace().nth(4) {
        word.parse().map_err(|e| AoCError::BadInputFormat(format!(
            "Parsing input failed, expected 'Generator <index> starts with <seed>', found '{}'. {}",
            input[0], e)))?
    } else {
        return Err(AoCError::BadInputFormat(format!(
            "Expected 'Generator <index> starts with <seed>', found '{}'.", input[0])))
    };
    let gen1 = if let Some(word) = input[1].split_whitespace().nth(4) {
        word.parse().map_err(|e| AoCError::BadInputFormat(format!(
            "Parsing input failed, expected 'Generator <index> starts with <seed>', found '{}'. {}",
            input[1], e)))?
    } else {
        return Err(AoCError::BadInputFormat(format!(
            "Expected 'Generator <index> starts with <seed>', found '{}'.", input[1])))
    };
    Ok((gen0, gen1))
}

fn compare_random_generators(gen0: Box<dyn Iterator<Item=u64>>, gen1: Box<dyn Iterator<Item=u64>>)
    -> usize {
    gen0
        .zip(gen1)
        .filter(|(r0, r1)| r0%(1<<16) == r1%(1<<16))
        .count()
}

struct RandomGeneratorTailingZeros {
    generator: RandomGenerator,
    tailing_zeros: usize,
    counter: usize,
}

impl RandomGeneratorTailingZeros {
    fn new(seed: u64, factor: u64, modulo: u64, counter: usize, tailing_zeros: usize) -> Self {
        let generator = RandomGenerator::new(seed, factor, modulo, None);
        Self{generator, tailing_zeros, counter}
    }
}

impl Iterator for RandomGeneratorTailingZeros {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == 0 {
            return None
        }
        self.counter -= 1;
        let mut next = self.generator.next()
            .expect("generator was created with infinite numbers");
        while next % (1<<self.tailing_zeros) != 0 {
            next = self.generator.next()
                .expect("generator was created with infinite numbers");
        }
        Some(next)
    }
}

struct RandomGenerator {
    prev: u64,
    factor: u64,
    modulo: u64,
    counter: Option<usize>,
}

impl RandomGenerator {
    fn new(seed: u64, factor: u64, modulo: u64, counter: Option<usize>) -> Self {
        Self{prev: seed, factor, modulo, counter}
    }
}

impl Iterator for RandomGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter.is_some() && self.counter.unwrap() == 0 {
            return None
        }
        self.counter.map(|c| c-1);
        let next = (self.prev * self.factor) % self.modulo;
        self.prev = next;
        Some(next)
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "Generator A starts with 65".to_string(),
            "Generator B starts with 8921".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let v = get_example_input();
        assert_eq!(part_1(&v), Ok("588".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_15.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("650".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = get_example_input();
        assert_eq!(part_2(&v), Ok("309".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_15.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("336".to_string()));
        Ok(())
    }
}