// https://hackattic.com/challenges/help_me_unpack

extern crate base64;
extern crate serde_json;
extern crate regex;

use toolbox::convert;
use regex::Regex;

pub const CHALLENGE_ID: &'static str = "help_me_unpack";

#[derive(Deserialize)]
struct ProblemPayload {
    bytes: String,
}

impl ProblemPayload {
    fn new(json_data: String) -> ProblemPayload {
        let payload = serde_json::from_str(&json_data).unwrap();
        return payload;
    }
}

#[derive(Serialize)]
struct SolutionPayload {
    pub int: i32,
    pub uint: u32,
    pub short: i16,
    pub float: f32,
    pub double: f64,
    pub big_endian_double: f64,
}

impl SolutionPayload {
    fn to_json(&self) -> String {
        let result = serde_json::to_string(&self).unwrap();

        // default float formatting does not include enough precision
        let float_re: Regex = Regex::new(r#""float":\d+\.\d+"#).unwrap();
        let float_value: &str = &format!("\"float\":{:.14}", self.float).to_owned();
        let original: &str = &result.to_owned();
        let updated = float_re.replace(original, float_value).to_string();

        return updated;
    }
}

pub fn solve(payload: String) -> String {
    let problem = ProblemPayload::new(payload);

    let result = process_payload(problem);
    let response = result.to_json();
    return response;
}

fn process_payload(problem: ProblemPayload) -> SolutionPayload {
    let bytes = base64::decode(&problem.bytes).unwrap();

    // bytes at position 10, 11 are always zero
    let result = SolutionPayload {
        int: convert::bytes_to_int(array_ref!(bytes, 0, 4)),
        uint: convert::bytes_to_uint(array_ref!(bytes, 4, 4)),
        short: convert::bytes_to_short(array_ref!(bytes, 8, 2)),
        float: convert::bytes_to_float(array_ref!(bytes, 12, 4)),
        double: convert::bytes_to_double(array_ref!(bytes, 16, 8)),
        big_endian_double: convert::bytes_to_double_big_endian(array_ref!(bytes, 24, 8)),
    };
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://stackoverflow.com/questions/34662713/how-can-i-create-parameterized-tests-in-rust

    fn payload_assert_eq(expected: SolutionPayload, result: SolutionPayload) {
        assert_eq!(expected.int, result.int, "int");
        assert_eq!(expected.uint, result.uint, "uint");
        assert_eq!(expected.short, result.short, "short");
        assert_eq!(expected.float, result.float, "float");
        assert_eq!(expected.double, result.double, "double");
        assert_eq!(expected.big_endian_double, result.big_endian_double, "big_endian_double");
    }

    #[test]
    fn process_payload_zero_string_returns_zeros() {
        let given = ProblemPayload {
            bytes: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_string()
        };
        let expected = SolutionPayload {
            int: 0,
            uint: 0,
            short: 0,
            float: 0.0,
            double: 0.0,
            big_endian_double: 0.0,
        };

        let result = process_payload(given);
        payload_assert_eq(expected, result);
    }

    #[test]
    fn process_payload_returns_expected() {
        let given = ProblemPayload {
            bytes: "FqyCgL5+3tiGogAAfuiXQ4U+XqGSkFJAQFKQkqFePoU=".to_string()
        };
        let expected = SolutionPayload {
            int: -2138919914,
            uint: 3638460094,
            short: -23930,
            float: 303.81634521484375,
            double: 74.2589496059755,
            big_endian_double: 74.2589496059755,
        };

        let result = process_payload(given);
        payload_assert_eq(expected, result);
    }

    macro_rules! bytes_to_double_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
              #[test]
              fn $name() {
                let (input, expected) = $value;
                let result = solve(input);
                assert_eq!(expected, result);;
              }
            )*
        }
    }

    bytes_to_double_tests! {
        solve_0: (
            r#"{"bytes":"FqyCgL5+3tiGogAAfuiXQ4U+XqGSkFJAQFKQkqFePoU="}"#.to_string(),
            r#"{"int":-2138919914,"uint":3638460094,"short":-23930,"float":303.81634521484375,"double":74.2589496059755,"big_endian_double":74.2589496059755}"#.to_string()
        ),
        solve_1: (
            r#"{"bytes":"kgVuinkuw/mhawAAvyO2QgTUkze8pHZAQHakvDeT1AQ="}"#.to_string(),
            r#"{"int":-1972501102,"uint":4190318201,"short":27553,"float":91.06981658935547,"double":362.2959514402903,"big_endian_double":362.2959514402903}"#
        ),
        solve_2: (
            r#"{"bytes":"q7jhjOYeqs/4lQAA+ypqQ1owWBeoOFVAQFU4qBdYMFo="}"#.to_string(),
            r#"{"int":-1931364181,"uint":3484032742,"short":-27144,"float":234.16789245605469,"double":84.88525947200961,"big_endian_double":84.88525947200961}"#
        ),
    }
}
