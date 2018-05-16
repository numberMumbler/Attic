extern crate serde;

use serde::{Deserialize, Serialize};

pub trait DefinesSolution<'a, P: Deserialize<'a>, S: Serialize> {
    fn solve(&self, problem: &P) -> S;
}
