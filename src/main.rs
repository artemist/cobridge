#![allow(dead_code)]
use cohost::CohostApi;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Server};
use log::info;
use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use structopt::StructOpt;

use crate::activitypub::server::{serve, State};

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

    // This would be kept around until main exits anyway and Arc is a pain
    let state: &'static State = Box::leak(Box::new(State {
        api: CohostApi::new(),
        domain: options.domain.clone(),
    }));

    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| async move {
                Ok::<_, Infallible>(serve(remote_addr, req, state).await)
            }))
        }
    });

    let server = Server::bind(&socket_addr).serve(make_svc);

    server.await.unwrap();
    Ok(())
}
