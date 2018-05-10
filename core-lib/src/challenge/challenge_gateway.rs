pub trait ChallengeGateway {
    fn get_problem_payload(&self, challenge_id: &String) -> String;
    fn send_solution_payload(&self, challenge_id: &String, json: String) -> String;
}
