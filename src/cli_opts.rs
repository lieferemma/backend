use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Address to bind gRPC api to.
    #[structopt(long, env = "GRPC_API_ADDR", default_value = "0.0.0.0:50051")]
    grpc_api_addr: SocketAddr,

    /// Postgres database url
    /// E.g.: `postgres://postgres:changeme@localhost/lieferemma`
    #[structopt(long, env = "DATABASE_URL")]
    database_url: String,
}
