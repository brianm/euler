use clap::Clap;
use super::primes;

/// Largest prime factor
///
/// The prime factors of 13195 are 5, 7, 13 and 29.
/// What is the largest prime factor of the number 600851475143 ?
///
#[derive(Clap)]
pub struct Solution {
    #[clap(short, long, default_value = "600851475143")]
    number: usize,
}

impl Solution {
    pub fn run(&self) -> usize {
        primes::factors(self.number).last().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn test_run() {
        let s = Solution { number: 13195 };
        assert_eq!(s.run(), 29);
    }
}
