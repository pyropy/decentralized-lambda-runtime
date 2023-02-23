// This example requires the following input to succeed:
// { "command": "do something" }

use decentralised_lambda_runtime::{Error, LambdaEvent};
use serde::{Deserialize, Serialize};

/// This is also a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    name: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    msg: String,
}

fn main() -> Result<(), Error> {
    decentralised_lambda_runtime::run(my_handler)?;
    Ok(())
}

pub(crate) fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // extract some useful info from the request
    let command = event.payload.name;

    // prepare the response
    let resp = Response {
        msg: format!("Hello, {}", command),
    };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}
