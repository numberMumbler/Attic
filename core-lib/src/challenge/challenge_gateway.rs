pub trait ChallengeGateway {
    fn get_problem_payload(&self, challenge_id: &str) -> String;
    fn send_solution_payload(&self, challenge_id: &str, json: &str) -> String;
}
