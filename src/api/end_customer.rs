pub use crate::api::grpc::end_customer_server::{EndCustomer, EndCustomerServer};
use crate::{
    api::grpc::{
        AvailableProductReply, AvailableProductRequest, Currency, CustomerInterestRequest,
        DeliveryPoint, DeliveryProduct, MobileShop, Order, OrderReply, OrderRequest,
        OrderStatusReply, OrderStatusRequest, OrderedProduct, PaymentStatus, Position, Product,
        ShipmentStatus, Unit,
    },
    db::query,
};
use diesel::{prelude::*, r2d2::ConnectionManager};
use futures::channel::mpsc;
use futures_util::sink::SinkExt;
use prost_types::Timestamp;
use tonic::{Request, Response, Status};

pub struct EndCustomerServerImpl {
    pub pg_connection_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

type RegisterCustomerInterestStream = mpsc::Receiver<Result<MobileShop, Status>>;

#[tonic::async_trait]
impl EndCustomer for EndCustomerServerImpl {
    type RegisterCustomerInterestStream = RegisterCustomerInterestStream;

    async fn register_customer_interest(
        &self,
        _request: Request<CustomerInterestRequest>,
    ) -> Result<Response<Self::RegisterCustomerInterestStream>, Status> {
        let pg_connection_pool = self.pg_connection_pool.clone();
        let pg_connection = pg_connection_pool.get().unwrap();

        let mobile_shops = query::mobile_shops(pg_connection).unwrap();

        println!("DB Mobile Shops: {:#?}", mobile_shops);

        let (mut tx, rx) = mpsc::channel(4);
        // let shops = vec![mobile_shop];
        tokio::spawn(async move {
            for shop in mobile_shops {
                tx.send(Ok(shop.clone())).await.unwrap();
            }
        });

        Ok(Response::new(rx))
    }

    async fn place_order(
        &self,
        _request: Request<OrderRequest>,
    ) -> Result<Response<OrderReply>, Status> {
        let product = Product {
            product_uuid: "91ea969e-6cd8-41ab-8faa-636cb9ffd991".to_string(),
            title: "Kaisersemmel".to_string(),
            description: "Unser Klassiker, das Kaiserbrötchen. Macht sich immer gut entweder mit Nutella oder Marmelade.".to_string(),
            url: "https://upload.wikimedia.org/wikipedia/commons/d/d0/Kaisersemmel-.jpg".to_string(),
            price: 90,
            currency: Currency::Eur as i32,
            max_order: 100,
            unit: Unit::Piece as i32,
        };

        let ordered_product = OrderedProduct {
            product: Some(product),
            quantity_ordered: 5,
            total_price: 450,
            currency: Currency::Eur as i32,
        };

        let pick_up_point = DeliveryPoint {
            delivery_point_uuid: "a9848bee-0ae2-4479-92e6-7c64657b860e".to_string(),
            // position of this delivery point
            position: Some(Position {
                latitude: 52.521_751,
                longitude: 13.411_500,
            }),
            // planed arrival time for this delivery point
            scheduled_time: Some(Timestamp::default()),
            // minimum time in seconds delivery point is available at this position
            departure_time: Some(Timestamp::default()),
        };

        let order_reply = OrderReply {
            order: Some(Order {
                mobile_shop_uuid: "3107c9bd-dd92-45b5-b6eb-c7b7b83b213e".to_string(),
                creation_time: Some(Timestamp::default()),
                update_time: Some(Timestamp::default()),
                order_uuid: "3feedb57-9f6e-476f-93fb-5515ea831d5f".to_string(),
                order_id: "abcd".to_string(),
                pick_up_point: Some(pick_up_point),
                currency: Currency::Eur as i32,
                total: 450,
                ordered_products: vec![ordered_product],
                shipment_status: ShipmentStatus::Created as i32,
                payment_status: PaymentStatus::Notpayed as i32,
            }),
        };

        Ok(Response::new(order_reply))
    }

    async fn available_products(
        &self,
        _request: Request<AvailableProductRequest>,
    ) -> Result<Response<AvailableProductReply>, Status> {
        let product = Product {
            product_uuid: "91ea969e-6cd8-41ab-8faa-636cb9ffd991".to_string(),
            title: "Kaisersemmel".to_string(),
            description: "Unser Klassiker, das Kaiserbrötchen. Macht sich immer gut entweder mit Nutella oder Marmelade.".to_string(),
            url: "https://upload.wikimedia.org/wikipedia/commons/d/d0/Kaisersemmel-.jpg".to_string(),
            price: 90,
            currency: Currency::Eur as i32,
            max_order: 100,
            unit: Unit::Piece as i32,
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
