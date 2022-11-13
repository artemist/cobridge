use anyhow::Context;
use hyper::{header, Body, Request, Response, StatusCode};
use log::debug;
use serde_json::json;
use std::net::SocketAddr;

use crate::cohost::CohostApi;

use super::webfinger::WebFinger;

pub struct State {
    pub api: CohostApi,
    pub domain: String,
}

fn json_error(status: StatusCode, msg: &str) -> Response<Body> {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .body(format!(r#"{{"error":"{}"}}"#, msg).into())
        .unwrap()
}

async fn handle_webfinger(
    request: Request<Body>,
    state: &'static State,
) -> anyhow::Result<Response<Body>> {
    let querystring = request.uri().query().ok_or(anyhow::anyhow!("no query"))?;
    let acct_encoded = querystring
        .split('&')
        .filter_map(|part| part.split_once('='))
        .filter(|(key, _)| key == &"resource")
        .map(|(_, value)| value)
        .next()
        .ok_or(anyhow::anyhow!("no resource specified"))?;
    let acct = urlencoding::decode(acct_encoded)
        .context("invalid urlencoding")?
        .into_owned();
    let (scheme, qualified_user) = acct.split_once(':').ok_or(anyhow::anyhow!("no scheme"))?;
    if scheme != "acct" {
        anyhow::bail!("incorrect scheme");
    }
    let (username, domain) = qualified_user
        .split_once('@')
        .ok_or(anyhow::anyhow!("no domain"))?;
    if domain != state.domain {
        anyhow::bail!("incorrect domain");
    }

    let mut trpc_response = state
        .api
        .make_trpc_request(
            vec!["posts.profilePosts".to_string()],
            true,
            &json!({
                "0": {
                    "projectHandle": username,
                    "page": 0,
                    "options": {
                        "hideReplies": false,
                        "hideShares": false,
                    },
                },
            }),
        )
        .await
        .context("unable to query cohost")?;

    let first = trpc_response
        .as_array_mut()
        .and_then(|array| array.first_mut())
        .ok_or(anyhow::anyhow!("invalid cohost response"))?;
    match serde_json::from_value::<crate::cohost::types::Response>(first.take())
        .context("unable to parse cohost response")?
    {
        crate::cohost::types::Response::Success(_) => {
            let webfinger = WebFinger::with_cohost_handle(username, &state.domain);
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(serde_json::to_string(&webfinger)?.into())
                .unwrap())
        }
        crate::cohost::types::Response::Failure(_) => anyhow::bail!("no such user"),
    }
}

pub async fn serve_inner(
    remote_addr: SocketAddr,
    request: Request<Body>,
    state: &'static State,
) -> anyhow::Result<Response<Body>> {
    debug!(
        "Got connection from {}: {} {}",
        remote_addr,
        request.method(),
        request.uri()
    );

    if request.method() == "GET" && request.uri().path() == "/.well-known/webfinger" {
        handle_webfinger(request, state).await
    } else {
        anyhow::bail!("no such path")
    }
}

pub async fn serve(
    remote_addr: SocketAddr,
    request: Request<Body>,
    state: &'static State,
) -> Response<Body> {
    match serve_inner(remote_addr, request, state).await {
        Ok(response) => response,
        Err(error) => {
            debug!(
                "Request failed, returning 404, backtrace: {}",
                error
                    .chain()
                    .map(|cause| cause.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            json_error(StatusCode::NOT_FOUND, &error.to_string())
        }
    }
}
