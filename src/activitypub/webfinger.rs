use super::{
    error::{ErrorWithStatus, ResponseResult},
    server::State,
};
use crate::cohost::types;
use anyhow::Context;
use axum::{extract::Query, Extension, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Representation of a WebFinger response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebFinger {
    /// `acct:` url of the subject
    pub subject: String,

    /// List of aliases, may be HTTP(S) links
    #[serde(default)]
    pub aliases: Vec<String>,

    /// Links for specific purposes
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Link {
    /// What this link refers to (e.g. "self")
    pub rel: String,
    /// MIME type of the resource
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Actual link in question
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// URL template, may appear instead of [href](self::href)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

impl WebFinger {
    pub fn with_cohost_handle(cohost_handle: &str, local_domain: &str) -> Self {
        Self {
            subject: format!("acct:{}@{}", cohost_handle, local_domain),
            aliases: vec![
                format!("https://{}/users/{}", local_domain, cohost_handle),
                format!("https://cohost.org/{}", cohost_handle),
            ],
            links: vec![
                Link {
                    rel: String::from("http://webfinger.net/rel/profile-page"),
                    mime_type: Some(String::from("text/html")),
                    href: Some(format!("https://cohost.org/{}", cohost_handle)),
                    template: None,
                },
                Link {
                    rel: String::from("self"),
                    mime_type: Some(String::from("application/activity+json")),
                    href: Some(format!("https://{}/users/{}", local_domain, cohost_handle)),
                    template: None,
                },
            ],
        }
    }
}

#[derive(Deserialize)]
pub struct WebFingerQuery {
    pub resource: String,
}

pub async fn handle_webfinger(
    query: Query<WebFingerQuery>,
    state: Extension<Arc<State>>,
) -> ResponseResult<Json<WebFinger>> {
    let (scheme, qualified_user) = query
        .resource
        .split_once(':')
        .ok_or(anyhow::anyhow!("no scheme"))?;
    if scheme != "acct" {
        return Err(anyhow::anyhow!("incorrect scheme").into());
    }
    let (username, domain) = qualified_user
        .split_once('@')
        .ok_or(anyhow::anyhow!("no domain"))?;
    if domain != state.domain {
        return Err(anyhow::anyhow!("incorrect domain").into());
    }

    let response_value = state
        .api
        .query_loader_state(&format!("/{}", username))
        .await?;
    match serde_json::from_value::<types::ProjectPageViewLoaderState>(response_value)
        .context("failed to parse cohost response")?
    {
        types::ProjectPageViewLoaderState::ProjectPageView(_) => {
            Ok(Json(WebFinger::with_cohost_handle(username, &state.domain)))
        }
        types::ProjectPageViewLoaderState::Error(_) => Err(ErrorWithStatus {
            status: StatusCode::NOT_FOUND,
            message: "no such user".to_string(),
        }
        .into()),
    }
}

pub async fn handle_host_meta(state: Extension<Arc<State>>) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
  <Link rel="lrdd" template="https://{}/.well-known/webfinger?resource={{uri}}"/>
</XRD>
"#,
        state.domain
    )
}
