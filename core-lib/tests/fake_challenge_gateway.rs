extern crate attic_core_lib;

use attic_core_lib::ChallengeGateway;

const CORRECT_RESPONSE: &str = r#"ok"#;
const INCORRECT_RESPONSE: &str = r#"nope"#;

pub struct FakeChallengeGateway {
    pub problem_payload: String,
    pub expected_solution: String,
}

impl FakeChallengeGateway {
    pub fn new(problem_payload: String, expected_solution: String) -> FakeChallengeGateway {
        FakeChallengeGateway {
            problem_payload,
            expected_solution,
        }
    }

    pub fn get_expected_response() -> String {
        CORRECT_RESPONSE.to_string()
    }
}

impl ChallengeGateway for FakeChallengeGateway {
    fn get_problem_payload(&self, _challenge_id: &str) -> String {
        self.problem_payload.clone()
    }

    fn send_solution_payload(&self, _challenge_id: &str, json: &str) -> String {
        if *json == self.expected_solution { CORRECT_RESPONSE.to_string() } else { INCORRECT_RESPONSE.to_string() }
    }
}
