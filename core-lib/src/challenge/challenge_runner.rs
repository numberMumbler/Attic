use challenge::ChallengeGateway;
use challenge::solver::*;

pub struct ChallengeRunner<'a, T: 'a + ChallengeGateway> {
    pub challenge_gateway: &'a T,
    pub is_verbose: bool,
}

impl<'a, T: 'a + ChallengeGateway> ChallengeRunner<'a, T> {
    pub fn new(challenge_gateway: &'a T, is_verbose: bool) -> ChallengeRunner<'a, T> {
        ChallengeRunner {
            challenge_gateway,
            is_verbose,
        }
    }

    pub fn solve_challenge(&self, challenge_id: &str) -> String {
        let solvers = [
            &HelpMeUnpackSolver::new() as &SolvesChallenge,
            &CollisionCourseSolver::new() as &SolvesChallenge,
        ];
        for &solver in solvers.iter() {
            if challenge_id == solver.get_challenge_id() {
                return self.solve(solver, challenge_id);
            }
        }
        panic!(format!("Unknown challenge ID: {}", challenge_id));
    }

    pub fn solve(&self, solver: &SolvesChallenge, challenge_id: &str) -> String {
        // TODO: use a logger
        if self.is_verbose {
            println!("Running {}...", challenge_id);
        }
        let problem_payload = self.challenge_gateway.get_problem_payload(challenge_id);
        if self.is_verbose {
            println!("problem: {}", problem_payload);
        }
        let solution_payload = solver.solve(&problem_payload);
        if self.is_verbose {
            println!("solution: {}", solution_payload);
        }

        let response = self.challenge_gateway
            .send_solution_payload(challenge_id, &solution_payload);
        return response;
    }
}
