use clap::Clap;
use primal;

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
        factors(self.number).last().unwrap_or(0)
    }
}

struct Factors {
    n: usize,
    last_prime: usize,
    primes: primal::Primes,
}

impl Iterator for Factors {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.n == 0 || self.n == 1 {
            return None;
        }
        loop {
            if self.n % self.last_prime == 0 {
                self.n = self.n / self.last_prime;
                return Some(self.last_prime);
            }

            self.last_prime = self.primes.next().expect("unable to get next prime!");
        }
    }
}

fn factors(n: usize) -> Factors {
    Factors {
        n: n,
        last_prime: 2,
        primes: primal::Primes::all(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let mut f = factors(13195);
        assert_eq!(f.next(), Some(5));
        assert_eq!(f.next(), Some(7));
        assert_eq!(f.next(), Some(13));
        assert_eq!(f.next(), Some(29));
        assert_eq!(f.next(), None);
    }

    #[test] 
    fn test_run() {
        let s = Solution {
            number: 13195
        };
        assert_eq!(s.run(), 29);
    }
}
