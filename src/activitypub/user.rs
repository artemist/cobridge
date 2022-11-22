use super::{
    activitystreams::ActorPage,
    error::{ErrorWithStatus, ResponseResult},
    server::State,
};
use crate::cohost::types;
use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use http::{header, HeaderMap, StatusCode};
use serde_json::Value;
use std::sync::Arc;

pub async fn handle_user(
    Path(user): Path<String>,
    state: Extension<Arc<State>>,
) -> ResponseResult<(HeaderMap, Json<Value>)> {
    let response_value = state.api.query_loader_state(&format!("/{}", &user)).await?;
    match serde_json::from_value::<types::ProjectPageViewLoaderState>(response_value)
        .context("failed to parse cohost response")?
    {
        types::ProjectPageViewLoaderState::ProjectPageView(project_page_view) => {
            let actor = ActorPage::with_project(&state.domain, &project_page_view.project);
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                "application/activity+json; charset=utf-8".parse().unwrap(),
            );
            Ok((
                headers,
                Json(serde_json::to_value(actor).context("unable to serialize actor")?),
            ))
        }
        types::ProjectPageViewLoaderState::Error(_) => Err(ErrorWithStatus {
            status: StatusCode::NOT_FOUND,
            message: "no such user".to_string(),
        }
        .into()),
    }
}
