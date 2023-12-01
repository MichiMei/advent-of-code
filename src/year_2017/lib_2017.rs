pub mod knot_hash {
    use std::fmt::Write;
    use std::fmt::{Display, Formatter};
    use std::ops::BitXor;

    pub struct KnotHash {
        numbers: Vec<u8>,
        position: usize,
        skip_size: usize,
    }

    impl KnotHash {
        pub fn new(len: u8) -> Self {
            let numbers = (0u8..=len).collect();
            let position = 0;
            let skip_size = 0;
            Self{numbers, position, skip_size}
        }

        fn hash(&mut self, len: usize) {
            let normalized = self.numbers[self.position..].iter().copied().chain(self.numbers[0..self.position].iter().copied()).collect::<Vec<_>>();
            let mut rotated = normalized[0..len].to_vec();
            rotated.reverse();
            rotated.extend(&normalized[len..]);
            self.numbers = rotated[rotated.len()-self.position..].iter().copied().chain(rotated[0..rotated.len()-self.position].iter().copied()).collect();
            self.position = (self.position+len+self.skip_size)%self.numbers.len();
            self.skip_size += 1;
        }

        pub fn execute_list(&mut self, numbers: &Vec<usize>) {
            for number in numbers {
                self.hash(*number);
            }
        }

        pub fn multi_hash(&mut self, runs: usize, numbers: &Vec<usize>) {
            for _ in 0..runs {
                self.execute_list(numbers);
            }
        }

        pub fn complete_hash(&mut self, input: &str) {
            let numbers = input.chars().map(|c| c as u8 as usize).chain([17, 31, 73, 47, 23]).collect::<Vec<_>>();
            self.multi_hash(64, &numbers);
        }

        pub fn get_dense_hash(&self) -> String {
            self.numbers
                .chunks_exact(16)
                .map(|bytes| bytes[1..]
                    .iter().
                    fold(bytes[0], |acc, b| acc.bitxor(b)))
                .fold(String::new(), |mut output, x| {
                    let _ = write!(output, "{:02x}", x);
                    output
                })
        }

        pub fn get_dense_hash_bytes(&self) -> Vec<u8> {
            self.numbers
                .chunks_exact(16)
                .map(|bytes| bytes[1..]
                    .iter().
                    fold(bytes[0], |acc, b| acc.bitxor(b)))
                .collect::<Vec<_>>()
        }

        pub fn get_hash_bytes(&self) -> Vec<u8> {
            self.numbers.to_vec()
        }

        pub fn get_start_product(&self) -> Option<usize> {
            if self.numbers.len() < 2 {
                return None
            }
            Some(self.numbers[0] as usize * self.numbers[1] as usize)
        }
    }

    impl Display for KnotHash {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut str = format!("pos({}) skip_size({}) ", self.position, self.skip_size);
            for elem in self.numbers.iter() {
                str = format!("{}{} ", str, elem);
            }
            write!(f, "{}", str)
        }
    }

}

