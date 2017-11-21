use std::str::FromStr;

use tokio_core::reactor;
use futures::Future;
use hyper::{self, Request, Method, Uri};
use hyper_rustls;
use serde_json;

use Message;

use ::errors::*;


#[derive(Debug, Clone)]
pub struct Client {
    uri: Uri,
}

impl Client {
    pub fn new(url: &str) -> Result<Client> {
        Ok(Client {
            uri: Uri::from_str(url)?,
        })
    }

    pub fn post(&self, msg: Message) -> Result<()> {
        let mut core = reactor::Core::new()?;
        let handle = core.handle();

        let client = hyper::Client::configure()
            .connector(hyper_rustls::HttpsConnector::new(4, &handle))
            .build(&handle);

        let mut req = Request::new(Method::Post, self.uri.clone());
        let body = serde_json::to_string(&msg)?;
        req.set_body(body);

        let work = client.request(req).map(|res| {
            if res.status().is_success() {
                Ok(())
            } else {
                Err(Error::from(ErrorKind::Status(res.status())))
            }
        });

        core.run(work)?
    }
}
