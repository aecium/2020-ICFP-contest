use std::path::PathBuf;
pub struct ProblemSet {
    problems : Vec<Problem>
}

pub struct Problem {
    // TODO This should represent the problem 
    // I'm not really sure what needs to be here
}

impl ProblemSet {
    pub fn parse(problems: &PathBuf) -> Self{
        Self {
            problems : vec![]
        }
    }
}

pub struct SolutionSet<'p> {
    // A solutionset doesn't need to be complete, None means that we couldn't solve this particular problem
    pub solutions : Vec<Option<Solution<'p>>>
}

pub struct Solution<'p> {
    // TODO This should contain a complete solution to a problem
    // I'm not really sure what needs to be here
    pub sourceproblem : &'p Problem
}