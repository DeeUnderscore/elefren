use Result;
use reqwest::{Client, Request, RequestBuilder, Response};

pub trait HttpSend {
    fn execute(&self, client: &Client, request: Request) -> Result<Response>;
    fn send(&self, client: &Client, builder: &mut RequestBuilder) -> Result<Response> {
        let request = builder.build()?;
        self.execute(client, request)
    }
}

pub struct HttpSender;

impl HttpSend for HttpSender {
    fn execute(&self, client: &Client, request: Request) -> Result<Response> {
        Ok(client.execute(request)?)
    }
}
