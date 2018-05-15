extern crate attic_core_lib;

mod fake_challenge_gateway;

use attic_core_lib::ChallengeRunner;
use fake_challenge_gateway::FakeChallengeGateway;

const CHALLENGE_ID: &str = "collision_course";

macro_rules! run_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
          #[test]
          fn $name() {
            let (problem_payload, expected_solution) = $value;
            let challenge_gateway = FakeChallengeGateway::new(
                problem_payload.to_string(),
                expected_solution.to_string());
            let expected_response = FakeChallengeGateway::get_expected_response();

            let runner = ChallengeRunner::new(challenge_gateway, false);
            let result = runner.solve_challenge(CHALLENGE_ID);

            assert_eq!(expected_response, result);
          }
        )*
    }
}

run_tests! {
    collision_course_0: (
        r#"{"include": "8caa4712a3831fa77c30a681148011ea"}"#,
        r#"{"files":["DjBlYVWap4fQC8b3C73+NATPA2WecE+FNMAP+2WcTIdAzJQv6y2hFaP0FVy7hgdJc4ZlbX0fNKQgWdePWo3R7zhjYWE0NzEyYTM4MzFmYTc3YzMwYTY4MTE0ODAxMWVh","DjBlYVWap4fQC8b3C73+NATPA2WedE+FNMAP+2WcTIdAzJQv6y2hFaP0Fdy7hgdJc4ZlbX0fNKQgWdePWo3R7zhjYWE0NzEyYTM4MzFmYTc3YzMwYTY4MTE0ODAxMWVh"]}"#
    ),
    collision_course_1: (
        r#"{"include": "4e1338d1a118693c38ac6c9189e7ef5f"}"#,
        r#"{"files":["DjBlYVWap4fQC8b3C73+NATPA2WecE+FNMAP+2WcTIdAzJQv6y2hFaP0FVy7hgdJc4ZlbX0fNKQgWdePWo3R7zRlMTMzOGQxYTExODY5M2MzOGFjNmM5MTg5ZTdlZjVm","DjBlYVWap4fQC8b3C73+NATPA2WedE+FNMAP+2WcTIdAzJQv6y2hFaP0Fdy7hgdJc4ZlbX0fNKQgWdePWo3R7zRlMTMzOGQxYTExODY5M2MzOGFjNmM5MTg5ZTdlZjVm"]}"#
    ),
}
