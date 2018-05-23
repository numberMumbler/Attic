extern crate regex;
extern crate serde_json;

use challenge::solver::SolvesChallenge;
use regex::Regex;
use solution::DefinesSolution;
use solution::help_me_unpack::{HelpMeUnpack, ProblemPayload, SolutionPayload};

pub struct HelpMeUnpackSolver {}

impl SolvesChallenge for HelpMeUnpackSolver {
    fn get_challenge_id(&self) -> String {
        HelpMeUnpack::get_challenge_id()
    }

    fn solve(&self, payload: &str) -> String {
        HelpMeUnpackSolver::go(&HelpMeUnpack::new(), payload)
    }
}

impl HelpMeUnpackSolver {
    fn go(solver: &DefinesSolution<ProblemPayload, SolutionPayload>, payload: &str) -> String {
        let problem: ProblemPayload = serde_json::from_str(payload).unwrap();
        let result = solver.solve(&problem);
        let response = HelpMeUnpackSolver::process_solution(&result);
        return response;
    }

    pub fn new() -> HelpMeUnpackSolver {
        HelpMeUnpackSolver {}
    }

    fn build_problem(json_data: &str) -> ProblemPayload {
        let payload = serde_json::from_str(&json_data).unwrap();
        return payload;
    }

    fn process_solution(solution: &SolutionPayload) -> String {
        let result = serde_json::to_string(&solution).unwrap();

        // default float formatting does not include enough precision
        let float_re: Regex = Regex::new(r#""float":\d+\.\d+"#).unwrap();
        let float_value: &str = &format!("\"float\":{:.14}", solution.float);
        let updated = float_re.replace(&result, float_value).to_string();

        return updated;
    }
}
