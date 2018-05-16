extern crate serde;

use serde::{Serialize, Deserialize};

pub trait DefinesSolution<'a, P: Deserialize<'a>, S: Serialize> {
    fn solve(&self, problem: &P) -> S;
}