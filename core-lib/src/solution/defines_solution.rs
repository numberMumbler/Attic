extern crate serde_json;
extern crate serde;

use serde::{Deserialize, Serialize};

pub trait DefinesSolution<'a, P: Deserialize<'a>, S: Serialize> {
    fn solve(&self, problem: &P) -> S;

    fn convert_solution(&self, result: &S) -> String {
        serde_json::to_string(result).unwrap()
    }
}
