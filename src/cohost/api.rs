use super::types::{CohostError, TrpcInput};
use crate::cohost::types;
use anyhow::Context;
use hyper::{
    client::HttpConnector,
    header::{self, HeaderValue},
    Body, Client, Request, Uri,
};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde::Serialize;
use serde_json::{json, Value};
use tracing::info;

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

    pub async fn trpc_query(
        &self,
        queries: Vec<&str>,
        batch: bool,
        input: &Value,
    ) -> anyhow::Result<Value> {
        info!(
            "Making TRPC request, queries: {:?}, batch: {}, input: {:#?}",
            queries, batch, input
        );

        let path = format!(
            "/api/v1/trpc/{}?batch={}&input={}",
            queries.join(","),
            batch as u32,
            urlencoding::encode(&input.to_string())
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

    pub async fn trpc_query_single<Q: Serialize + TrpcInput>(
        &self,
        input: &Q,
    ) -> anyhow::Result<Result<Q::Response, CohostError>> {
        let mut response_raw = self
            .trpc_query(
                vec![Q::query_name()],
                true,
                &json!({
                    "0": serde_json::to_value(input).context("failed to serialize request")?
                }),
            )
            .await?;

        let response_0 = response_raw
            .as_array_mut()
            .and_then(|obj| obj.first_mut())
            .ok_or(anyhow::anyhow!("no response given"))?;

        match serde_json::from_value(response_0.take())
            .context("failed to parse success or error")?
        {
            types::CohostResponse::Success(success) => {
                Ok(Ok(serde_json::from_value::<Q::Response>(success.data)
                    .context("failed to parse success")?))
            }
            types::CohostResponse::Failure(failure) => Ok(Err(failure)),
        }
    }
}
