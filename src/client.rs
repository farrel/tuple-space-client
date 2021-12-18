use crate::error::Error;
use crate::result::Result;
use reqwest::{StatusCode, Url};
use tuple_space::tuple::Tuple;

pub struct Client {
    write_url: Url,
    read_url: Url,
    take_url: Url,
    http_client: reqwest::Client,
}

pub struct Builder {}

impl Client {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub async fn write_tuple(&self, tuple: &Tuple) -> Result<()> {
        let response = self
            .http_client
            .post(self.write_url.clone())
            .body(serde_json::to_string(tuple)?)
            .send()
            .await?;

        match response.status() {
            StatusCode::CREATED => Ok(()),
            _ => Err(Error::ServerError),
        }
    }

    pub async fn read_tuple(&self, tuple: &Tuple) -> Result<Option<Tuple>> {
        let response = self
            .http_client
            .post(self.read_url.clone())
            .body(serde_json::to_string(tuple)?)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(Some(response.json::<Tuple>().await?)),
            StatusCode::NOT_FOUND => Ok(None),
            _ => Err(Error::ServerError),
        }
    }

    pub async fn take_tuple(&self, tuple: &Tuple) -> Result<Option<Tuple>> {
        let response = self
            .http_client
            .post(self.take_url.clone())
            .body(serde_json::to_string(tuple)?)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(Some(response.json::<Tuple>().await?)),
            StatusCode::NOT_FOUND => Ok(None),
            _ => Err(Error::ServerError),
        }
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder {}
    }

    pub fn build(&self, server: &str) -> Result<Client> {
        let base_server = Url::parse(server)?;
        let read_url = base_server.join("read_tuple")?;
        let take_url = base_server.join("take_tuple")?;
        let write_url = base_server.join("write_tuple")?;

        Ok(Client {
            http_client: reqwest::Client::new(),
            read_url,
            take_url,
            write_url,
        })
    }
}
