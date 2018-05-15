extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use futures::{Future, Stream};
use hyper::{Client, Method, Request, Uri};
use hyper::header::{ContentLength, ContentType};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use attic_core_lib::ChallengeGateway;

pub struct Attic {
    access_token: String,
}

impl ChallengeGateway for Attic {
    fn get_problem_payload(&self, challenge_id: &str) -> String {
        let problem_uri = self.get_problem_uri(challenge_id);
        let result = Attic::https_get(&problem_uri);
        return result;
    }

    fn send_solution_payload(&self, challenge_id: &str, json: &str) -> String {
        let solution_uri = self.get_solution_uri(challenge_id);
        let response = Attic::https_post(&solution_uri, json);
        return response;
    }
}

impl Attic {
    pub fn new(access_token: String) -> Attic {
        return Attic {
            access_token
        };
    }

    fn get_problem_uri(&self, challenge_id: &str) -> Uri {
        return format!("https://hackattic.com/challenges/{}/problem?access_token={}",
                       challenge_id,
                       self.access_token).parse().unwrap();
    }

    fn get_solution_uri(&self, challenge_id: &str) -> Uri {
        return format!("https://hackattic.com/challenges/{}/solve?access_token={}",
                       challenge_id,
                       self.access_token).parse().unwrap();
    }

    fn https_get(uri: &Uri) -> String {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);

        let request = client.get(uri.clone())
            .and_then(|response| {
                return response.body().concat2();
            });

        let payload = core.run(request).unwrap();
        return String::from_utf8(payload.to_vec()).unwrap();
    }

    fn https_post(solution_uri: &Uri, json: &str) -> String {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);

        let mut req = Request::new(Method::Post, solution_uri.clone());
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json.to_string());

        let post = client.request(req)
            .and_then(|response| {
                response.body().concat2()
            });

        let response_body = core.run(post).unwrap();
        return String::from_utf8(response_body.to_vec()).unwrap();
    }
}
