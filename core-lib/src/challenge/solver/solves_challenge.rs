pub trait SolvesChallenge {
    fn get_challenge_id(&self) -> String;
    fn solve(&self, payload: &str) -> String;
}
