extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use futures::{Future, Stream};
use hyper::{Client, Method, Request, Uri};
use hyper::header::{ContentLength, ContentType};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

pub fn get_problem_payload(challenge_id: &String, access_token: &String) -> String {
    let problem_uri = get_problem_uri(challenge_id, access_token);
    let result = https_get(problem_uri);
    return result;
}

pub fn send_solution_payload(challenge_id: &String, access_token: &String, json: String) -> String {
    let solution_uri = get_solution_uri(challenge_id, access_token);
    let response = https_post(solution_uri, json);
    return response;
}

fn get_problem_uri(challenge_id: &String, access_token: &String) -> Uri {
    return format!("https://hackattic.com/challenges/{}/problem?access_token={}",
                   challenge_id,
                   access_token).parse().unwrap();
}

fn get_solution_uri(challenge_id: &String, access_token: &String) -> Uri {
    return format!("https://hackattic.com/challenges/{}/solve?access_token={}",
                   challenge_id,
                   access_token).parse().unwrap();
}

fn https_get(uri: Uri) -> String {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let request = client.get(uri).and_then(|response|{
        return response.body().concat2();
    });

    let payload = core.run(request).unwrap();
    return String::from_utf8(payload.to_vec()).unwrap();
}

fn https_post(solution_uri: Uri, json: String) -> String {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let mut req = Request::new(Method::Post, solution_uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.set_body(json);

    let post = client.request(req).and_then(|response| {
        response.body().concat2()
    });

    let response_body = core.run(post).unwrap();
    return String::from_utf8(response_body.to_vec()).unwrap();
}
