extern crate base64;

use toolbox::collision_attack;
use solution::DefinesSolution;
extern crate serde;

use serde::{Serialize, Deserialize};


pub struct CollisionCourse {}

impl CollisionCourse {
    pub fn new() -> CollisionCourse {
        CollisionCourse {}
    }
}

impl<'a> DefinesSolution<'a, ProblemPayload, SolutionPayload> for CollisionCourse {
   fn solve(&self, problem: &ProblemPayload) -> SolutionPayload {
        let (result_1, result_2) =
            collision_attack::generate_colliders(&problem.include.as_bytes());
        return SolutionPayload::new(&result_1, &result_2);
    }
}

#[derive(Deserialize)]
pub struct ProblemPayload {
    include: String,
}

#[derive(Serialize)]
pub struct SolutionPayload {
    pub files: [String; 2],
}

impl SolutionPayload {
    pub fn new(content_1: &Vec<u8>, content_2: &Vec<u8>) -> SolutionPayload {
        SolutionPayload {
            files: [base64::encode(&content_1), base64::encode(&content_2)],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_basic_test() {
        let given = ProblemPayload {
            include: "f4e97c930597f11cc9e5e22642ec16e5".to_string(),
        };
        let expected = [
            "DjBlYVWap4fQC8b3C73+NATPA2WecE+FNMAP+2WcTIdAzJQv6y2hFaP0FVy7hgdJc4ZlbX0fNKQgWdePWo3R72Y0ZTk3YzkzMDU5N2YxMWNjOWU1ZTIyNjQyZWMxNmU1".to_string(),
            "DjBlYVWap4fQC8b3C73+NATPA2WedE+FNMAP+2WcTIdAzJQv6y2hFaP0Fdy7hgdJc4ZlbX0fNKQgWdePWo3R72Y0ZTk3YzkzMDU5N2YxMWNjOWU1ZTIyNjQyZWMxNmU1".to_string()
        ];

        let solver = CollisionCourse::new();
        let result = solver.solve(&given);

        assert_eq!(expected[0], result.files[0], "element 0 mismatch");
        assert_eq!(expected[1], result.files[1], "element 1 mismatch");
    }
}
