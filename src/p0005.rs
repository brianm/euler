use crate::primes;
use clap::Clap;
use std::collections::HashMap;

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
        let mut all_factors: HashMap<usize, usize> = HashMap::new();
        for n in 1..(self.through + 1) {
            let mut factors = HashMap::new();
            for f in primes::factors(n) {
                if factors.contains_key(&f) {
                    factors.insert(f, factors.get(&f).unwrap() + 1);
                } else {
                    factors.insert(f, 1);
                }
            }
            for (k, v) in factors {
                let av = all_factors.get(&k).unwrap_or(&0);
                if v > *av {
                    all_factors.insert(k, v);
                }
            }
        }
        let mut num = 1;
        for (k, v) in all_factors {
            num *= k.pow(v as u32);
        }
        num
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
