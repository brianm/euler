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