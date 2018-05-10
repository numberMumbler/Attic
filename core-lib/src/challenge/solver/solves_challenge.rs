pub trait SolvesChallenge {
    fn get_challenge_id() -> String;
    fn solve(&self, payload: String) -> String;
}
