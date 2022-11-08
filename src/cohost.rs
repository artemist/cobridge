use anyhow::Context;
use std::sync::Arc;

use hyper::{
    client::HttpConnector,
    header::{self, HeaderValue},
    Body, Client, Request, Uri,
};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

use serde_json::Value;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug)]
pub struct CohostApi {
    user_agent: HeaderValue,
    token: Option<Box<str>>,
    http_client: Client<HttpsConnector<HttpConnector>>,
}

impl CohostApi {
    pub fn new() -> Arc<Self> {
        let conn = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();

        Arc::new(Self {
            user_agent: HeaderValue::from_str(&format!("cobridge/{}", VERSION)).unwrap(),
            token: None,
            http_client: Client::builder().build(conn),
        })
    }

    pub async fn make_trpc_request(
        self: Arc<Self>,
        fields: Arc<[String]>,
        batch: bool,
        request: &Value,
    ) -> anyhow::Result<Value> {
        let path = format!(
            "/api/v1/trpc/{}?batch={}&input={}",
            fields.join(","),
            batch as u32,
            urlencoding::encode(&request.to_string())
        );

        let uri = Uri::builder()
            .scheme("https")
            .authority("cohost.org")
            .path_and_query(path)
            .build()
            .unwrap();

        
        let mut builder = Request::builder()
            .uri(uri)
            .header(header::USER_AGENT, self.user_agent.clone());

        if let Some(token) = &self.token {
            builder = builder.header(header::COOKIE, ["Cookie: connect.sid=", &token.as_ref()].join(""));
        }
        

        let request = builder.body(Body::empty())?;

        let response = self
            .http_client
            .request(request)
            .await
            .context("Failed to make RPC request")?;

        serde_json::from_slice(&hyper::body::to_bytes(response.into_body()).await?)
            .context("Failed to parse response as JSON")
    }
}
