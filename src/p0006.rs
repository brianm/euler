use clap::Clap;

/// Sum square difference
///
/// The sum of the squares of the first ten natural numbers is,
///     1^2 + 2^2 ... 10^2 = 385  
/// The square of the sum of the first ten natural numbers is,
///     (1 + 2 ... + 10)^2 = 55^2 = 3025
/// Hence the difference between the sum of the squares of the
/// first ten natural numbers and the square of the sum is
///     3025 − 385 = 2640
/// Find the difference between the sum of the squares of the
/// first one hundred natural numbers and the square of the sum.
#[derive(Clap)]
pub struct Solution {
    #[clap(short, long, default_value = "100")]
    through: usize,
}

impl Solution {
    pub fn run(&self) -> usize {
        doit(self.through)
    }
}

fn doit(limit: usize) -> usize {
    let sum = limit * (limit + 1)/2;
    let sum_sq = ((2 * limit) + 1) * (limit + 1) * limit/6;
    sum.pow(2) - sum_sq

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided() {
        let s = doit(10);
        assert_eq!(s, 2640);
    }
}
