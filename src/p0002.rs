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
        let mut sum = 0;
        let mut i = 0;
        loop {
            let val = fib(i);
            if val > self.limit {
                break
            }
            if val % 2 == 0 {
                sum += val;
            }            
            i += 1;
        }

        return sum;
    }
}

fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
    }
}
