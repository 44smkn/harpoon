use crate::infrastructure::webapi::client::Client;
use async_trait::async_trait;
use chrono::prelude::*;
use futures_util::stream::TryStreamExt;
use hyper::client::ResponseFuture;
use hyper::{self, Body};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

pub struct RestApi {
    pub client: hyper::Client<UnixConnector, Body>,
    pub unix_socket: String,
    // 認証情報とかを後で追加する
}

impl RestApi {
    pub fn new(unix_socket_path: &str) -> RestApi {
        let client = hyper::Client::unix();
        let unix_socket = unix_socket_path.to_string();
        RestApi {
            client,
            unix_socket,
        }
    }
}

impl Client for RestApi {
    fn get(&self, path: &str) -> ResponseFuture {
        let uri = Uri::new(&self.unix_socket, path).into();
        self.client.get(uri)
    }
}
