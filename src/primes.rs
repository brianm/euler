pub struct PrimeFactors {
    n: usize,
    last_prime: usize,
    primes: primal::Primes,
}

impl Iterator for PrimeFactors {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.n == 0 || self.n == 1 {
            return None;
        }
        loop {
            if self.n % self.last_prime == 0 {
                self.n /= self.last_prime;
                return Some(self.last_prime);
            }

            self.last_prime = self.primes.next().expect("unable to get next prime!");
        }
    }
}

pub fn factors(n: usize) -> PrimeFactors {
    PrimeFactors {
        n,
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
    fn test_factor_6() {
        let mut f = factors(6);
        assert_eq!(f.next(), Some(2));
        assert_eq!(f.next(), Some(3));
        assert_eq!(f.next(), None);
    }

    #[test]
    fn test_factor_12() {
        let mut f = factors(12);
        assert_eq!(f.next(), Some(2));
        assert_eq!(f.next(), Some(2));
        assert_eq!(f.next(), Some(3));
        assert_eq!(f.next(), None);
    }
}
