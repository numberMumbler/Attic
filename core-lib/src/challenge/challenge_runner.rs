use challenge::ChallengeGateway;
use challenge::solver::*;

pub struct ChallengeRunner<T: ChallengeGateway> {
    pub challenge_gateway: T,
    pub is_verbose: bool,
}

impl<T: ChallengeGateway> ChallengeRunner<T> {
    pub fn new(challenge_gateway: T, is_verbose: bool) -> ChallengeRunner<T> {
        ChallengeRunner {
            challenge_gateway,
            is_verbose,
        }
    }

    pub fn solve_challenge(&self, challenge_id: &str) -> String {
        // TODO: expand this to Builder or something
        if challenge_id == HelpMeUnpackSolver::get_challenge_id() {
            let solver = HelpMeUnpackSolver::new();
            return self.solve(solver, challenge_id);
        } else if challenge_id == CollisionCourseSolver::get_challenge_id() {
            let solver = CollisionCourseSolver::new();
            return self.solve(solver, challenge_id);
        } else {
            panic!(format!("Unknown challenge ID: {}", challenge_id));
        }
    }

    pub fn solve<U: SolvesChallenge>(&self, solver: U, challenge_id: &str) -> String {
        // TODO: use a logger
        if self.is_verbose { println!("Running {}...", challenge_id); }
        let problem_payload = self.challenge_gateway.get_problem_payload(challenge_id);
        if self.is_verbose { println!("problem: {}", problem_payload); }
        let solution_payload = solver.solve(problem_payload);
        if self.is_verbose { println!("solution: {}", solution_payload); }

        let response = self.challenge_gateway.send_solution_payload(challenge_id, &solution_payload);
        return response;
    }
}
