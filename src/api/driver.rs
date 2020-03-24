pub use crate::api::grpc::driver_server::Driver;
use crate::api::grpc::{
    ConfirmOrderDeliveryReply, ConfirmOrderDeliveryRequest, DriverPosition, GetOrdersReply,
    GetOrdersRequest, GetRouteRequest, Route, SendPositionReply, UpdateDeliveryStatusReply,
    UpdateDeliveryStatusRequest,
};
use futures::Stream;
use std::pin::Pin;
use tonic::{self, Request, Response, Status};

pub use crate::api::grpc::driver_server::DriverServer;

type GetRouteStream = Pin<Box<dyn Stream<Item = Result<Route, Status>> + Send + Sync>>;

pub struct DriverServerImpl {}

#[tonic::async_trait]
impl Driver for DriverServerImpl {
    type GetRouteStream = GetRouteStream;

    async fn get_orders(
        &self,
        _request: Request<GetOrdersRequest>,
    ) -> Result<Response<GetOrdersReply>, Status> {
        unimplemented!()
    }

    async fn get_route(
        &self,
        _request: Request<GetRouteRequest>,
    ) -> Result<Response<Self::GetRouteStream>, Status> {
        unimplemented!()
    }

    async fn send_position(
        &self,
        _request: Request<DriverPosition>,
    ) -> Result<Response<SendPositionReply>, Status> {
        unimplemented!()
    }

    async fn confirm_order_delivery(
        &self,
        _request: Request<ConfirmOrderDeliveryRequest>,
    ) -> Result<Response<ConfirmOrderDeliveryReply>, Status> {
        unimplemented!()
    }

    async fn update_delivery_status(
        &self,
        _request: Request<UpdateDeliveryStatusRequest>,
    ) -> Result<Response<UpdateDeliveryStatusReply>, Status> {
        unimplemented!()
    }
}
