extern crate serde_json;

use challenge::solver::SolvesChallenge;
use solution::collision_course::{CollisionCourse, ProblemPayload, SolutionPayload};

const CHALLENGE_ID: &str = "collision_course";

pub struct CollisionCourseSolver {}

impl SolvesChallenge for CollisionCourseSolver {
    fn get_challenge_id() -> String {
        CHALLENGE_ID.to_string()
    }

    fn solve(&self, payload: &str) -> String {
        let problem = CollisionCourseSolver::build_problem(payload);
        let result = CollisionCourse::solve(&problem);
        let response = CollisionCourseSolver::convert_solution(&result);
        return response;
    }
}

impl CollisionCourseSolver {
    pub fn new() -> CollisionCourseSolver {
        CollisionCourseSolver {}
    }

    fn build_problem(json_data: &str) -> ProblemPayload {
        let payload = serde_json::from_str(&json_data).unwrap();
        return payload;
    }

    fn convert_solution(solution: &SolutionPayload) -> String {
        let result = serde_json::to_string(solution).unwrap();
        return result;
    }
}
