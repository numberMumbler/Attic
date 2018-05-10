extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate attic_core_lib;

#[macro_use]
extern crate serde_derive;

mod util;

use attic_core_lib::ChallengeRunner;
use util::{Settings, Attic};

const DEBUG: bool = true;

fn main() {
    let settings = Settings::new();
    let challenge_gateway = Attic::new(settings.access_token);

    let runner = ChallengeRunner::new(challenge_gateway, DEBUG);
    let result = runner.solve_challenge("help_me_unpack".to_string());
    println!("Response: {}", result);
}
