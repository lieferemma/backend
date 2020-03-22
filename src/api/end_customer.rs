pub use api::end_customer_server::{EndCustomer, EndCustomerServer};
use api::{
    AvailableProductReply, AvailableProductRequest, CustomerInterestRequest, MobileShop,
    OrderReply, OrderRequest, OrderStatusReply, OrderStatusRequest,
};
use futures::Stream;
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};

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
