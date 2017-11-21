use std::io;
use futures::{Future, Stream};
use hyper::{Client, Error};
use hyper::{Method, Request};
use hyper::header::{ContentLength, ContentType};
use tokio_core::reactor::Core;
use serde_json;
use super::{request, response};

pub fn request_translate(uri: &str, word_to_translate: request::Translation) -> Result<response::Translation, Error> {
    let uri = uri.parse()?;
    let json = (json!(word_to_translate)).to_string();

    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.set_body(json);
    let post = client.request(req).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().concat2().and_then(move |body| {
            parse_json(&body)
        })
    });
    core.run(post)
}

pub fn parse_json(body: &[u8]) -> Result<response::Translation, Error> {
    let translation: response::Translation = serde_json::from_slice(body).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            e
        )
    })?;
    Ok(translation)
}