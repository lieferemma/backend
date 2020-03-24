// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use diesel_geography::types::GeogPoint;
use crate::db::custom_types::DeliveryStatus;
use crate::db::schema::*;

use uuid::Uuid;
use chrono::DateTime;
use chrono::offset::Utc;

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(delivery_point_id)]
#[table_name = "delivery_points"]
pub struct DeliveryPoint {
    pub delivery_point_id: Uuid,
    pub position: GeogPoint,
    pub scheduled_time: DateTime<Utc>,
    pub departure_time: DateTime<Utc>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(mobile_shop_id)]
#[table_name = "mobile_shops"]
pub struct MobileShop {
    pub mobile_shop_id: Uuid,
    pub image_url: Option<String>,
    pub title: String,
    pub current_position: Option<GeogPoint>,
    pub last_seen: Option<DateTime<Utc>>,
    pub current_delivery_point_id: Option<Uuid>,
    pub next_delivery_point_id: Uuid,
    pub delivery_status: Option<DeliveryStatus>,
    pub next_delivery_point_eta: Option<DateTime<Utc>>,
    pub pick_up_delivery_point_eta: Option<DateTime<Utc>>,
    pub route_id: Uuid,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(route_id)]
#[table_name = "routes"]
pub struct Route {
    pub route_id: Uuid,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(routes_delivery_point_id)]
#[table_name = "routes_delivery_points"]
pub struct RoutesDeliveryPoint {
    pub routes_delivery_point_id: Uuid,
    pub route_id: Uuid,
    pub delivery_point_id: Uuid,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(route_point_id)]
#[table_name = "routes_route_points"]
pub struct RoutesRoutePoint {
    pub route_point_id: Uuid,
    pub route_id: Uuid,
    pub position: GeogPoint,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(srid)]
#[table_name = "spatial_ref_sys"]
pub struct SpatialRefSy {
    pub srid: i32,
    pub auth_name: Option<String>,
    pub auth_srid: Option<i32>,
    pub srtext: Option<String>,
    pub proj4text: Option<String>,
}

