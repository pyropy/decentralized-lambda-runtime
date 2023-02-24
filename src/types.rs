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