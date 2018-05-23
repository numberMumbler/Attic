extern crate serde_json;

use challenge::solver::SolvesChallenge;
use solution::DefinesSolution;
use solution::help_me_unpack::{HelpMeUnpack, ProblemPayload, SolutionPayload};

pub struct HelpMeUnpackSolver {}

impl SolvesChallenge for HelpMeUnpackSolver {
    fn get_challenge_id(&self) -> String {
        HelpMeUnpack::get_challenge_id()
    }

    fn solve(&self, payload: &str) -> String {
        HelpMeUnpackSolver::go(
            &HelpMeUnpack::new(),
            payload,
        )
    }
}

impl HelpMeUnpackSolver {
    fn go(
        solver: &DefinesSolution<ProblemPayload, SolutionPayload>,
        payload: &str,
    ) -> String {
        let problem: ProblemPayload = serde_json::from_str(payload).unwrap();
        let result = solver.solve(&problem);
        let response = solver.process_solution(&result);
        return response;
    }

    pub fn new() -> HelpMeUnpackSolver {
        HelpMeUnpackSolver {}
    }
}
