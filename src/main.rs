extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod api;
mod cli_opts;
mod db;

use anyhow::Result;
use api::{
    driver::{DriverServer, DriverServerImpl},
    end_customer::{EndCustomerServer, EndCustomerServerImpl},
};
use cli_opts::Opt;
use diesel::r2d2::ConnectionManager;
use log::info;
use structopt::StructOpt;
use tonic::transport::Server;

// This includes the diesel migrations in the binary
embed_migrations!();

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let opt = Opt::from_args();
    info!("Running with following options:\n{:#?}", opt);

    let pg_connection_manager = ConnectionManager::new(opt.database_url());
    let pg_connection_pool = r2d2::Pool::new(pg_connection_manager)?;

    // Run the diesel migrations
    let pg_connection = pg_connection_pool.get()?;
    embedded_migrations::run_with_output(&pg_connection, &mut std::io::stdout())?;

    let end_customer_server = EndCustomerServerImpl { pg_connection_pool };
    let driver_server = DriverServerImpl {};

    info!("gRPC API served from {}", opt.grpc_api_addr());

    Server::builder()
        .add_service(EndCustomerServer::new(end_customer_server))
        .add_service(DriverServer::new(driver_server))
        .serve(*opt.grpc_api_addr())
        .await?;

    Ok(())
}
