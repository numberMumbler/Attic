extern crate serde_json;

use challenge::solver::SolvesChallenge;
use solution::DefinesSolution;
use solution::collision_course::{CollisionCourse, ProblemPayload, SolutionPayload};

pub struct CollisionCourseSolver {}

impl SolvesChallenge for CollisionCourseSolver {
    fn get_challenge_id(&self) -> String { CollisionCourse::get_challenge_id() }

    fn solve(&self, payload: &str) -> String {
        CollisionCourseSolver::go(
            &CollisionCourse::new(),
            payload,
        )
    }
}

impl CollisionCourseSolver {
    fn go(
        solver: &DefinesSolution<ProblemPayload, SolutionPayload>,
        payload: &str,
    ) -> String {
        let problem: ProblemPayload = serde_json::from_str(payload).unwrap();
        let result = solver.solve(&problem);
        let response = solver.convert_solution(&result);
        return response;
    }

    pub fn new() -> CollisionCourseSolver {
        CollisionCourseSolver {}
    }
}
