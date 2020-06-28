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
    use super::primes;

    #[test]
    fn test_fib() {
        let mut f = primes::factors(13195);
        assert_eq!(f.next(), Some(5));
        assert_eq!(f.next(), Some(7));
        assert_eq!(f.next(), Some(13));
        assert_eq!(f.next(), Some(29));
        assert_eq!(f.next(), None);
    }

    #[test]
    fn test_run() {
        let s = Solution { number: 13195 };
        assert_eq!(s.run(), 29);
    }
}
