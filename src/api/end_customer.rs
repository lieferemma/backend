pub use crate::api::grpc::end_customer_server::{EndCustomer, EndCustomerServer};
use crate::api::grpc::{
    AvailableProductReply, AvailableProductRequest, Currency, CustomerInterestRequest,
    DeliveryProduct, MobileShop, OrderReply, OrderRequest, OrderStatusReply, OrderStatusRequest,
    OrderedProduct, Product,
};
use futures::Stream;
use prost_types::Timestamp;
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
        let product = Product {
            product_uiid: "91ea969e-6cd8-41ab-8faa-636cb9ffd991".to_string(),
            title: "Kaisersemmel".to_string(),
            description: "Unser Klassiker, das Kaiserbrötchen. Macht sich immer gut entweder mit Nutella oder Marmelade.".to_string(),
            url: "https://upload.wikimedia.org/wikipedia/commons/d/d0/Kaisersemmel-.jpg".to_string(),
            price: 90,
            currency: Currency::Eur as i32,
        };

        let ordered_product = OrderedProduct {
            product: Some(product),
            quantity_ordered: 5,
            total_price: 450,
            currency: Currency::Eur as i32,
        };

        let order_reply = OrderReply {
            order_uuid: "3feedb57-9f6e-476f-93fb-5515ea831d5f".to_string(),
            order_id: "abcd".to_string(),
            currency: Currency::Eur as i32,
            total: 450,
            ordered_products: vec![ordered_product],
        };

        Ok(Response::new(order_reply))
    }

    async fn available_products(
        &self,
        _request: Request<AvailableProductRequest>,
    ) -> Result<Response<AvailableProductReply>, Status> {
        let product = Product {
            product_uiid: "91ea969e-6cd8-41ab-8faa-636cb9ffd991".to_string(),
            title: "Kaisersemmel".to_string(),
            description: "Unser Klassiker, das Kaiserbrötchen. Macht sich immer gut entweder mit Nutella oder Marmelade.".to_string(),
            url: "https://upload.wikimedia.org/wikipedia/commons/d/d0/Kaisersemmel-.jpg".to_string(),
            price: 90,
            currency: Currency::Eur as i32,
        };

        let deliverable_product = DeliveryProduct {
            product: Some(product),
            quantity_available: 100,
        };

        let reply = AvailableProductReply {
            updated: Some(Timestamp::default()),
            deliverable_products: vec![deliverable_product],
        };

        Ok(Response::new(reply))
    }

    async fn order_status(
        &self,
        _request: Request<OrderStatusRequest>,
    ) -> Result<Response<OrderStatusReply>, Status> {
        unimplemented!()
    }
}
