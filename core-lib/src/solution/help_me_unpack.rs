// https://hackattic.com/challenges/help_me_unpack

extern crate base64;

use toolbox::convert;

pub struct HelpMeUnpack {}

impl HelpMeUnpack {
    pub fn solve(problem: &ProblemPayload) -> SolutionPayload {
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
}

#[derive(Deserialize)]
pub struct ProblemPayload {
    bytes: String,
}

#[derive(Serialize)]
pub struct SolutionPayload {
    pub int: i32,
    pub uint: u32,
    pub short: i16,
    pub float: f32,
    pub double: f64,
    pub big_endian_double: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let result = HelpMeUnpack::solve(&given);
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

        let result = HelpMeUnpack::solve(&given);
        payload_assert_eq(expected, result);
    }
}
