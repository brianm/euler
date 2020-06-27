use clap::Clap;
use anyhow::Result;
mod p0001;
mod p0002;

#[derive(Clap)]
#[clap(version = "1.0", author = "Brian McCallister <brianm@skife.org>")]
struct Args {
    #[clap(subcommand)]
    subcmd: Command,
}

#[derive(Clap)]
enum Command {    
    #[clap(name="1")]
    One(p0001::Solution),
    
    #[clap(name="2")]
    Two(p0002::Solution),
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    match args.subcmd {
        Command::One(sol) => {
            Ok(println!("{}", sol.run()))
        }
        Command::Two(sol) => {
            Ok(println!("{}", sol.run()))
        }
    }
}