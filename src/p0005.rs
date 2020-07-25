use clap::Clap;

/// Smallest Multiple
///
/// 2520 is the smallest number that can be divided by
/// each of the numbers from 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly
/// divisible by all of the numbers from 1 to 20?
#[derive(Clap)]
pub struct Solution {
    #[clap(short, long, default_value = "20")]
    through: usize,
}

impl Solution {
    pub fn run(&self) -> usize {
        (1..self.through + 1).fold(1, num::integer::lcm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided() {
        let s = Solution { through: 10 };
        let n = s.run();
        assert_eq!(n, 2520);
    }
}
