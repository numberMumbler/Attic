extern crate base64;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate regex;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate arrayref;

mod challenge;
mod toolbox;
mod util;

use challenge::help_me_unpack;
use util::{attic, settings};

const DEBUG: bool = true;

fn solve_challenge(access_token: &String, challenge_id: &String, solve: fn(String) -> String) -> String {
    let problem_payload = attic::get_problem_payload(challenge_id, access_token);
    if DEBUG { println!("problem: {}", problem_payload); }
    let solution_payload = solve(problem_payload);
    if DEBUG { println!("solution: {}", solution_payload); }
    let response = attic::send_solution_payload(challenge_id, access_token, solution_payload);
    return response;
}

fn main() {
    let settings = settings::get_settings();
    let access_token = settings.access_token;

    // challenge-specific
    let challenge_id = help_me_unpack::CHALLENGE_ID.to_string();
    let solve = help_me_unpack::solve;

    println!("Running {}...", challenge_id);
    let result = solve_challenge(&access_token, &challenge_id, solve);
    println!("Response: {}", result);
}
