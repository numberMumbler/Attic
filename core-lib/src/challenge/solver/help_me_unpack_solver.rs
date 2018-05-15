extern crate serde_json;
extern crate regex;

use regex::Regex;
use challenge::solver::SolvesChallenge;
use solution::help_me_unpack::{HelpMeUnpack, ProblemPayload, SolutionPayload};

const CHALLENGE_ID: &str = "help_me_unpack";

pub struct HelpMeUnpackSolver {}

impl SolvesChallenge for HelpMeUnpackSolver {
    fn get_challenge_id() -> String { CHALLENGE_ID.to_string() }

    fn solve(&self, payload: String) -> String {
        let problem = HelpMeUnpackSolver::build_problem(payload);
        let result = HelpMeUnpack::solve(problem);
        let response = HelpMeUnpackSolver::convert_solution(result);
        return response;
    }
}

impl HelpMeUnpackSolver {
    pub fn new() -> HelpMeUnpackSolver { HelpMeUnpackSolver {} }

    fn build_problem(json_data: String) -> ProblemPayload {
        let payload = serde_json::from_str(&json_data).unwrap();
        return payload;
    }

    fn convert_solution(solution: SolutionPayload) -> String {
        let result = serde_json::to_string(&solution).unwrap();

        // default float formatting does not include enough precision
        let float_re: Regex = Regex::new(r#""float":\d+\.\d+"#).unwrap();
        let float_value: &str = &format!("\"float\":{:.14}", solution.float).to_owned();
        let original: &str = &result.to_owned();
        let updated = float_re.replace(original, float_value).to_string();

        return updated;
    }
}
