use std::path::PathBuf;
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(PartialEq, Debug)]
    enum Solver {
        Basic
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
  problems: PathBuf,

  #[structopt(possible_values = &Solver::variants(), case_insensitive = true)]
  #[structopt(min_values = 1)]
  #[structopt(short, long)]
  solvers: Vec<Solver>
}

fn main() {
    println!("Hello, world!");
    let mut opt = Opt::from_args();
    opt.solvers.dedup();
    println!("{:?}", opt);
}
