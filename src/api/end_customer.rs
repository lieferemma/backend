pub use crate::api::grpc::end_customer_server::{EndCustomer, EndCustomerServer};
use crate::api::grpc::{
    AvailableProductReply, AvailableProductRequest, CustomerInterestRequest, MobileShop,
    OrderReply, OrderRequest, OrderStatusReply, OrderStatusRequest,
};
use futures::Stream;
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};

pub struct EndCustomerServerImpl {}

type RegisterCustomerInterestStream =
    Pin<Box<dyn Stream<Item = Result<MobileShop, Status>> + Send + Sync>>;

#[tonic::async_trait]
impl EndCustomer for EndCustomerServerImpl {
    type RegisterCustomerInterestStream = RegisterCustomerInterestStream;

    async fn register_customer_interest(
        &self,
        _request: Request<Streaming<CustomerInterestRequest>>,
    ) -> Result<Response<Self::RegisterCustomerInterestStream>, Status> {
        unimplemented!()
    }

    async fn place_order(
        &self,
        _request: Request<OrderRequest>,
    ) -> Result<Response<OrderReply>, Status> {
        unimplemented!()
    }

    async fn available_products(
        &self,
        _request: Request<AvailableProductRequest>,
    ) -> Result<Response<AvailableProductReply>, Status> {
        unimplemented!()
    }

    async fn order_status(
        &self,
        _request: Request<OrderStatusRequest>,
    ) -> Result<Response<OrderStatusReply>, Status> {
        unimplemented!()
    }
}
