mod api;
mod cli_opts;
mod db;

use anyhow::Result;
use api::{
    driver::{DriverServer, DriverServerImpl},
    end_customer::{EndCustomerServer, EndCustomerServerImpl},
};
use cli_opts::Opt;
use log::info;
use sqlx::postgres::PgPool;
use structopt::StructOpt;
use tokio_postgres::NoTls;
use tonic::transport::Server;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let opt = Opt::from_args();
    info!("Running with following options:\n{:#?}", opt);

    // Run the database migrations
    let (mut migration_client, migration_connection) =
        tokio_postgres::connect(opt.database_url(), NoTls).await?;

    tokio::spawn(async move {
        migration_connection.await.unwrap();
    });

    embedded::migrations::runner()
        .run_async(&mut migration_client)
        .await?;

    let pg_connection_pool = PgPool::builder()
        .max_size(10)
        .build(opt.database_url())
        .await?;

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
