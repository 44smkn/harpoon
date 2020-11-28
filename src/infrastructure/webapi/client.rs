use async_trait::async_trait;
use hyper::client::ResponseFuture;
use hyperlocal::Uri;

#[async_trait]
pub trait Client: Send {
    fn get(&self, path: &str) -> ResponseFuture;
}
