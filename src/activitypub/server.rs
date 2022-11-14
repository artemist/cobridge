use anyhow::Context;
use hyper::{header, Body, Request, Response, StatusCode};
use log::info;
use std::net::SocketAddr;

use crate::cohost::{types, CohostApi};

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

    let response = state
        .api
        .trpc_query_single(&types::ProfilePostsInput {
            project_handle: username.to_string(),
            page: 0,
            options: types::ProfilePostsInputOptions {
                hide_replies: false,
                hide_shares: false,
            },
        })
        .await?
        .context("failed to query cohost");

    if response.is_err() {
        anyhow::bail!("no such user");
    }
    let webfinger = WebFinger::with_cohost_handle(username, &state.domain);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&webfinger)?.into())
        .unwrap())
}

pub async fn serve_inner(
    remote_addr: SocketAddr,
    request: Request<Body>,
    state: &'static State,
) -> anyhow::Result<Response<Body>> {
    info!(
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
            info!(
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
