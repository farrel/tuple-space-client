use crate::result::Result;
use tuple_space::tuple::Tuple;

enum Hyper {
    Http(hyper::Client<hyper::client::connect::HttpConnector>),
    Https(hyper::Client<hyper_tls::HttpsConnector<hyper::client::connect::HttpConnector>>),
}

pub struct Client {
    hyper: Hyper,
}

pub struct Builder {}

impl Client {
    pub fn builder() -> Builder {
        Builder::new()
    }
    pub fn write_tuple(&self, tuple: &Tuple) -> Result<()> {
        Ok(())
    }

    pub fn read_tuple(&self, tuple: &Tuple) -> Result<Option<Tuple>> {
        Ok(None)
    }

    pub fn take_tuple(&self, tuple: &Tuple) -> Result<Option<Tuple>> {
        Ok(None)
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder {}
    }

    pub fn build_http(&self) -> Client {
        Client {
            hyper: Hyper::Http(hyper::Client::new()),
        }
    }

    pub fn build_https(&self) -> Client {
        Client {
            hyper: Hyper::Https(
                hyper::Client::builder().build::<_, hyper::Body>(hyper_tls::HttpsConnector::new()),
            ),
        }
    }
}

#[test]
fn test_init() {}
