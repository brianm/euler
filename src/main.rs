use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Brian McCallister <brianm@skife.org>")]
struct Args {
    #[clap(subcommand)]
    subcmd: Command,
}

#[derive(Clap)]
enum Command {
    #[clap(name = "1")]
    One(euler::p0001::Solution),

    #[clap(name = "2")]
    Two(euler::p0002::Solution),

    #[clap(name = "3")]
    Three(euler::p0003::Solution),

    #[clap(name = "4")]
    Four(euler::p0004::Solution),

    #[clap(name = "5")]
    Five(euler::p0005::Solution),
}

fn main() {
    let args: Args = Args::parse();

    match args.subcmd {
        Command::One(sol) => println!("{}", sol.run()),
        Command::Two(sol) => println!("{}", sol.run()),
        Command::Three(sol) => println!("{}", sol.run()),
        Command::Four(sol) => println!("{}", sol.run()),
        Command::Five(sol) => println!("{}", sol.run()),
    }
}
