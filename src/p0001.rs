use clap::Clap;

/// Multiples of 3 and 5
#[derive(Clap)]
pub struct Solution {
    #[clap(short, long, default_value = "1000")]
    limit: u64,
}

impl Solution {
    pub fn run(&self) -> u64 {
        let mut sum = 0;
        for i in 1..self.limit {
            sum = sum + if (i % 3 == 0) || (i % 5 == 0) { i } else { 0 }
        }
        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided() {
        let s = Solution { limit: 10 };
        let val = s.run();
        assert_eq!(val, 23);
    }
}
