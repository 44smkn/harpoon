use crate::infrastructure::webapi::client::Client;
use hyper::client::connect::{Connect, HttpConnector};
use hyper::client::ResponseFuture;
use hyper::{self, Body};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};

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

impl<T: Connect + Clone + Send + Sync + 'static> Client for RestApi<T> {
    fn get(&self, path: &str) -> ResponseFuture {
        let uri = Uri::new(&self.url, path).into();
        self.client.get(uri)
    }
}
