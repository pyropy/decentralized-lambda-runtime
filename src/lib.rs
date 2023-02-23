mod types;

use http::Response;
use hyper::Body;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    fs,
};
pub use types::{Context, Error, EventError, EventSuccess, IntoResponse, LambdaEvent};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Config {
    pub input_path: String,
    pub output_path: String,
}

impl Config {
    pub fn default() -> Self {
        Config {
            input_path: String::from("/input/inputs.json"),
            output_path: String::from("/output/outputs.json"),
        }
    }
}

fn run_handler<F, A, T, E>(handler: F, config: &Config) -> Result<(), Error>
where
    F: Fn(LambdaEvent<A>) -> Result<T, E>,
    T: Serialize,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
    A: for<'de> Deserialize<'de>,
{
    let ctx = Context::new(); // TODO: Load context from fs

    let body = match fs::read(config.input_path.as_str()) {
        Ok(body) => body,
        Err(err) => {
            let response = build_event_error_request(err)?;
            println!("{:?}", response);
            return Ok(());
        }
    };

    let body = match serde_json::from_slice(&body) {
        Ok(body) => body,
        Err(err) => {
            let response = build_event_error_request(err)?;
            println!("{:?}", response);
            return Ok(());
        }
    };

    let response = match handler(LambdaEvent::new(body, ctx)) {
        Ok(response) => EventSuccess { body: response }.into_rsp(),
        Err(err) => {
            let error_type = type_name_of_val(&err);
            let msg = "Panic";
            // let msg = if let Some(msg) = err.downcast_ref::<&str>() {
            //     format!("Lambda panicked: {msg}")
            // } else {
            //     "Lambda panicked".to_string()
            // };
            EventError::new(error_type, &msg).into_rsp()
        }
    };

    // TODO: Persist response
    println!("{:?}", response);

    return Ok(());
}

pub fn run<F, A, T, E>(handler: F) -> Result<(), Error>
where
    F: Fn(LambdaEvent<A>) -> Result<T, E>,
    T: Serialize,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
    A: for<'de> Deserialize<'de>,
{
    let config = Config::default();
    run_handler(handler, &config)
}

fn type_name_of_val<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn build_event_error_request<T>(err: T) -> Result<Response<Body>, Error>
where
    T: Display + Debug,
{
    let error_type = type_name_of_val(&err);
    let msg = format!("{err}");

    EventError::new(error_type, &msg).into_rsp()
}
