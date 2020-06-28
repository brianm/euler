use clap::Clap;

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
    through: u64,
}

impl Solution {
    pub fn run(&self) -> u64 {
        todo!("implement")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided() {
       
    }
}
