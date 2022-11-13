use crate::cohost::types;
use anyhow::Context;
use log::debug;
use serde::de::DeserializeOwned;

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
    token: Option<String>,
    http_client: Client<HttpsConnector<HttpConnector>>,
}

impl CohostApi {
    pub fn new() -> Self {
        let conn = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();

        Self {
            user_agent: HeaderValue::from_str(&format!("cobridge/{}", VERSION)).unwrap(),
            token: None,
            http_client: Client::builder().build(conn),
        }
    }

    pub async fn make_trpc_request(
        &self,
        fields: Vec<String>,
        batch: bool,
        request: &Value,
    ) -> anyhow::Result<Value> {
        debug!(
            "Making TRPC request, fields: {:?}, batch: {}, input: {:#?}",
            fields, batch, request
        );

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
            builder = builder.header(
                header::COOKIE,
                ["Cookie: connect.sid=", token.as_ref()].join(""),
            );
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

    pub fn parse_response<T: DeserializeOwned>(value: Value) -> anyhow::Result<T> {
        let response: types::Response =
            serde_json::from_value(value).context("Failed to find valid success or error")?;
        match response {
            types::Response::Failure(error) => Err(error).context("Got error in cohost response"),
            types::Response::Success(result) => serde_json::from_value(result.data)
                .context("Unable to deserialize successful cohost response"),
        }
    }
}
