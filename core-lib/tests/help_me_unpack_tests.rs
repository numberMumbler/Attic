extern crate attic_core_lib;

mod fake_challenge_gateway;

use attic_core_lib::ChallengeRunner;
use fake_challenge_gateway::FakeChallengeGateway;

const CHALLENGE_ID: &str = "help_me_unpack";

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

            let runner = ChallengeRunner::new(&challenge_gateway, false);
            let result = runner.solve_challenge(CHALLENGE_ID);

            assert_eq!(expected_response, result);
          }
        )*
    }
}

run_tests! {
    help_me_unpack_0: (
        r#"{"bytes":"FqyCgL5+3tiGogAAfuiXQ4U+XqGSkFJAQFKQkqFePoU="}"#,
        r#"{"int":-2138919914,"uint":3638460094,"short":-23930,"float":303.81634521484375,"double":74.2589496059755,"big_endian_double":74.2589496059755}"#
    ),
    help_me_unpack_1: (
        r#"{"bytes":"kgVuinkuw/mhawAAvyO2QgTUkze8pHZAQHakvDeT1AQ="}"#,
        r#"{"int":-1972501102,"uint":4190318201,"short":27553,"float":91.06981658935547,"double":362.2959514402903,"big_endian_double":362.2959514402903}"#,
    ),
    help_me_unpack_2: (
        r#"{"bytes":"q7jhjOYeqs/4lQAA+ypqQ1owWBeoOFVAQFU4qBdYMFo="}"#,
        r#"{"int":-1931364181,"uint":3484032742,"short":-27144,"float":234.16789245605469,"double":84.88525947200961,"big_endian_double":84.88525947200961}"#,
    ),
}
