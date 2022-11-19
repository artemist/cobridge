#![allow(dead_code)]
use crate::activitypub::server::State;
use crate::activitypub::user::handle_user;
use crate::activitypub::webfinger::{handle_host_meta, handle_webfinger};
use axum::routing::get;
use axum::{Extension, Router};
use cohost::CohostApi;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use structopt::StructOpt;
use tower_http::trace::TraceLayer;
use tracing::info;

mod activitypub;
mod cohost;

#[derive(Debug, StructOpt)]
#[structopt(name = "cobridge", about = "Bridge from cohost to ActivityPub")]
struct Options {
    /// Publically-accessible domain
    #[structopt(short, long, default_value = "localhost")]
    domain: String,

    /// Local bind address
    #[structopt(short = "b", long = "bind", default_value = "::")]
    bind_addr: String,

    /// Port
    #[structopt(short, long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let options = Options::from_args();

    let socket_addr = SocketAddr::new(IpAddr::from_str(&options.bind_addr)?, options.port);
    info!(
        "Binding to {}, serving on domain {}",
        socket_addr, &options.domain
    );

    let state = Arc::new(State {
        api: CohostApi::new(),
        domain: options.domain.clone(),
    });

    let app = Router::new()
        .route("/.well-known/webfinger", get(handle_webfinger))
        .route("/.well-known/host-meta", get(handle_host_meta))
        .route("/users/:user", get(handle_user))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(state));

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
