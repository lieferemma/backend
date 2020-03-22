mod api;

use api::{
    driver::{DriverServer, DriverServerImpl},
    end_customer::{EndCustomerServer, EndCustomerServerImpl},
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let end_customer_server = EndCustomerServerImpl {};
    let driver_server = DriverServerImpl {};

    println!("EndCustomer gRPC API listening on {}", addr);

    Server::builder()
        .add_service(EndCustomerServer::new(end_customer_server))
        .add_service(DriverServer::new(driver_server))
        .serve(addr)
        .await?;

    Ok(())
}
