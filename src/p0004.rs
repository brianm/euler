use clap::Clap;
use itertools::Itertools;

/// Largest palindrome product
///
/// A palindromic number reads the same both ways. The
/// largest palindrome made from the product of two
/// 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product 
/// of two 3-digit numbers.
#[derive(Clap)]
pub struct Solution {
    #[clap(short, long, default_value = "1000")]
    limit: u64,
}

impl Solution {
    pub fn run(&self) -> u64 {
        (1..self.limit)
            .permutations(2)
            .map(|v| v[0] * v[1])
            .filter(is_palindrome)
            .max()
            .unwrap_or(0)
    }
}

fn is_palindrome(num: &u64) -> bool {    
    let d = digits(*num);
    for i in 0..(d.len() / 2) {
        if d[i] != d[d.len() - 1 - i] {
            return false;
        }
    }
    true
}

fn digits(n: u64) -> Vec<u64> {
    fn recur(n: u64, rs: &mut Vec<u64>) {
        if n >= 10 {
            recur(n / 10, rs);
        }
        rs.push(n % 10);
    }
    let mut rs = Vec::new();
    recur(n, &mut rs);
    rs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(&1));
        assert!(is_palindrome(&11));
        assert!(is_palindrome(&121));
        assert!(is_palindrome(&1221));
        assert!(is_palindrome(&12321));
        assert!(is_palindrome(&123321));

        assert!(!is_palindrome(&12));
        assert!(!is_palindrome(&123));
    }

    #[test]
    fn test_provided() {
        let s = Solution { limit: 100 };
        let palindrome = s.run();
        assert_eq!(palindrome, 9009);
    }
}
