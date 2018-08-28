use reqwest::{Client, Request, RequestBuilder, Response};
use std::fmt::Debug;
use Result;

pub trait HttpSend: Clone + Debug {
    fn execute(&self, client: &Client, request: Request) -> Result<Response>;
    fn send(&self, client: &Client, builder: &mut RequestBuilder) -> Result<Response> {
        let request = builder.build()?;
        self.execute(client, request)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HttpSender;

impl HttpSend for HttpSender {
    fn execute(&self, client: &Client, request: Request) -> Result<Response> {
        Ok(client.execute(request)?)
    }
}
