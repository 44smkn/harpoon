use crate::infrastructure::webapi::client::Client;
use async_trait::async_trait;
use futures_util::stream::TryStreamExt;
use hyper::client::connect::{Connect, HttpConnector};
use hyper::client::ResponseFuture;
use hyper::{self, Body};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use std::error::Error;

pub struct RestApi<T: Connect> {
    pub client: hyper::Client<T, Body>,
    pub url: String,
    // 認証情報とかを後で追加する
}

impl RestApi<UnixConnector> {
    pub fn new(unix_socket_path: &str) -> RestApi<UnixConnector> {
        let client = hyper::Client::unix();
        let url = unix_socket_path.to_string();
        RestApi { client, url }
    }
}

impl RestApi<HttpConnector> {
    pub fn new(domain: &str) -> RestApi<HttpConnector> {
        let client = hyper::Client::new();
        let url = domain.to_string();
        RestApi { client, url }
    }
}

#[async_trait]
impl<T> Client for RestApi<T>
where
    T: Connect + Clone + Send + Sync + 'static,
{
    async fn get(&self, path: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let uri = Uri::new(&self.url, path).into();
        let response_body = self.client.get(uri).await?.into_body();
        let bytes = response_body
            .try_fold(Vec::default(), |mut buf, bytes| async {
                buf.extend(bytes);
                Ok(buf)
            })
            .await?;
        Ok(bytes)
    }
}
