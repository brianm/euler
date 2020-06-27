use clap::Clap;

/// Even Fibonacci numbers
///
/// Each new term in the Fibonacci sequence is generated
/// by adding the previous two terms. By starting with 1
/// and 2, the first 10 terms will be:
///
///      1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
///
/// By considering the terms in the Fibonacci sequence
/// whose values do not exceed four million, find the
/// sum of the even-valued terms.
#[derive(Clap)]
pub struct Solution {
    #[clap(short, long, default_value = "4000000")]
    limit: u64,
}

impl Solution {
    pub fn run(&self) -> u64 {
        fibonacci()
            .filter(|v| v % 2 == 0)
            .take_while(|val| val < &self.limit)
            .sum()
    }
}

struct Fib {
    curr: u64,
    next: u64,
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fib {
    Fib { curr: 0, next: 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let mut f = fibonacci();

        assert_eq!(f.next().expect("should have had first value"), 1);
        assert_eq!(f.next().expect("should have had first value"), 1);
        assert_eq!(f.next().expect("should have had first value"), 2);
        assert_eq!(f.next().expect("should have had first value"), 3);
        assert_eq!(f.next().expect("should have had first value"), 5);
    }
}
