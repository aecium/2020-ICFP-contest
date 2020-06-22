use std::path::PathBuf;
use structopt::clap::arg_enum;
use structopt::StructOpt;

mod problems;
use problems::*;

mod basic;

arg_enum! {
    #[derive(PartialEq, Debug)]
    enum Solver {
        Basic
    }
}

impl Solver {
    fn solve<'a> (&self, problemset: &'a ProblemSet) -> SolutionSet<'a> {
        match self {
            Solver::Basic => basic::solve(problemset),
            _ => panic!("That solver isn't supported...yet?")
        }
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
    let mut opt = Opt::from_args();
    opt.solvers.dedup();

    //parse the problems into a problemset for consumption by solvers
    let problemset = ProblemSet::parse(&opt.problems);

    //solve the problems with the correct solvers
    let solutionSets = opt.solvers.iter().map(|s| s.solve(&problemset)).collect::<Vec<_>>();
}
