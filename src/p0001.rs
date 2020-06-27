use anyhow::Result;
use clap::Clap;

#[derive(Clap)]
pub struct Solution {
    #[clap(short, long, default_value="1000")]
    limit: u64 ,
}

impl Solution {
    pub fn run(&self) -> Result<u64> {
        let mut sum = 0;
        for i in 1 .. self.limit {
            sum = sum + if (i % 3 == 0) || (i % 5 == 0) {
                i
            } else {
                0
            }
        }
        return Ok(sum);
    }
}