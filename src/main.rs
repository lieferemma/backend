use crate::api::end_customer_server::{EndCustomer, EndCustomerServer};
use crate::api::AvailableProductReply;
use crate::api::AvailableProductRequest;
use crate::api::CustomerInterestRequest;
use crate::api::MobileShop;
use crate::api::OrderReply;
use crate::api::OrderRequest;
use crate::api::OrderStatusReply;
use crate::api::OrderStatusRequest;
use futures::Stream;
use std::pin::Pin;
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub mod api {
    tonic::include_proto!("lieferemma");
}

pub struct EndCustomerServerImpl {}

type RegisterCustomerInterestStream =
    Pin<Box<dyn Stream<Item = Result<MobileShop, Status>> + Send + Sync>>;

#[tonic::async_trait]
impl EndCustomer for EndCustomerServerImpl {
    type RegisterCustomerInterestStream = RegisterCustomerInterestStream;

    async fn register_customer_interest(
        &self,
        request: Request<Streaming<CustomerInterestRequest>>,
    ) -> Result<Response<Self::RegisterCustomerInterestStream>, Status> {
        unimplemented!()
    }

    async fn place_order(
        &self,
        request: Request<OrderRequest>,
    ) -> Result<Response<OrderReply>, Status> {
        unimplemented!()
    }

    async fn available_products(
        &self,
        request: Request<AvailableProductRequest>,
    ) -> Result<Response<AvailableProductReply>, Status> {
        unimplemented!()
    }

    async fn order_status(
        &self,
        request: Request<OrderStatusRequest>,
    ) -> Result<Response<OrderStatusReply>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let end_customer_server = EndCustomerServerImpl {};

    println!("EndCustomer gRPC API listening on {}", addr);

    Server::builder()
        .add_service(EndCustomerServer::new(end_customer_server))
        .serve(addr)
        .await?;

    Ok(())
}
