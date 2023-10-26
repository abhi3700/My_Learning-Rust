extern crate structopt;
// use clap::ArgEnum;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "cli_app",
    about = "A simple CLI app example using clap and structopt"
)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(about = "Say hello")]
    Hello {
        #[structopt(
            short = "a",
            long,
            default_value = "World",
            help = "Who to say hello to"
        )]
        name: String,
    },
    #[structopt(about = "Add two numbers")]
    Add {
        #[structopt(short = "b", long, help = "First number")]
        num1: i32,
        #[structopt(short = "c", long, help = "Second number")]
        num2: i32,
    },
}

fn main() {
    let opt = Cli::from_args();
    match opt.cmd {
        Command::Hello { name } => {
            println!("Hello, {}!", name);
        }
        Command::Add { num1, num2 } => {
            println!("The sum of {} and {} is {}", num1, num2, num1 + num2);
        }
    }
}
