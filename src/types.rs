use http::Response;
use serde::{Deserialize, Serialize};

pub type Body = Vec<u8>;

#[derive(Debug, Clone, PartialEq)]
pub struct Context {}
impl Context {
    pub fn new() -> Self {
        Context {}
    }
}

#[derive(Debug, Clone)]
pub struct LambdaEvent<T> {
    pub payload: T,
    pub context: Context,
}

impl<T> LambdaEvent<T> {
    pub fn new(payload: T, context: Context) -> Self {
        LambdaEvent { payload, context }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait IntoResponse {
    fn into_rsp(self) -> Result<Response<Body>, Error>;
}

pub trait SaveResult {
    fn save_result(self) -> Result<(), Error>;
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSuccess<T> {
    pub body: T,
}

impl<T> IntoResponse for EventSuccess<T>
where
    T: for<'serialize> Serialize,
{
    fn into_rsp(self) -> Result<Response<Body>, Error> {
        let body = serde_json::to_vec(&self.body)?;
        let resp = Response::builder().status(200).body(body)?;

        Ok(resp)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostics<'a> {
    pub error_type: &'a str,
    pub error_msg: &'a str,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventError<'a> {
    pub diagnostic: Diagnostics<'a>,
}

impl<'a> EventError<'a> {
    pub fn new(error_type: &'a str, error_msg: &'a str) -> Self {
        EventError {
            diagnostic: Diagnostics {
                error_type,
                error_msg,
            },
        }
    }
}

impl<'a> IntoResponse for EventError<'a> {
    fn into_rsp(self) -> Result<Response<Body>, Error> {
        let body = serde_json::to_vec(&self.diagnostic)?;
        let resp = Response::builder().status(500).body(body)?;

        Ok(resp)
    }
}
