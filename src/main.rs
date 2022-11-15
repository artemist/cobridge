#![allow(dead_code)]
use axum::routing::get;
use axum::{Extension, Router};
use cohost::CohostApi;
use log::info;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use structopt::StructOpt;

use crate::activitypub::server::State;
use crate::activitypub::webfinger::handle_webfinger;

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
    env_logger::init();
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
        .layer(Extension(state));

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
