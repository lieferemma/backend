//! Maps database models to gRPC typess

use crate::{api::grpc, db};
use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel_geography::types::GeogPoint;
use prost_types::Timestamp;
use std::convert::{TryFrom, TryInto};

impl From<GeogPoint> for grpc::Position {
    fn from(geog_point: GeogPoint) -> Self {
        Self {
            latitude: geog_point.x, // TODO this is very likely not the correct conversion
            longitude: geog_point.y,
        }
    }
}

impl From<db::custom_types::DeliveryStatus> for grpc::DeliveryStatus {
    fn from(delivery_status: db::custom_types::DeliveryStatus) -> Self {
        match delivery_status {
            db::custom_types::DeliveryStatus::OnTour => Self::OnTour,
            db::custom_types::DeliveryStatus::Parked => Self::Parked,
        }
    }
}

fn to_timestamp(dt: DateTime<Utc>) -> Result<Timestamp> {
    let nanos = dt.timestamp_subsec_nanos().try_into()?;
    Ok(Timestamp {
        seconds: dt.timestamp(),
        nanos,
    })
}

// fn to_grpc_mobile_shop()
impl TryFrom<db::models::MobileShop> for grpc::MobileShop {
    type Error = anyhow::Error;

    fn try_from(db_mobile_shop: db::models::MobileShop) -> Result<Self> {
        Ok(Self {
            mobile_shop_uuid: db_mobile_shop.mobile_shop_id.to_string(),
            image_url: db_mobile_shop.image_url.unwrap_or_default(),
            title: db_mobile_shop.title,
            current_position: db_mobile_shop.current_position.map(|x| x.into()),
            last_seen: db_mobile_shop.last_seen.map(to_timestamp).transpose()?,
            current_delivery_point: None,
            next_delivery_point: None,
            delivery_status: db_mobile_shop
                .delivery_status
                .map(|x| {
                    let ds: grpc::DeliveryStatus = x.into();
                    ds as i32
                })
                .unwrap_or_default(),
            next_delivery_point_eta: db_mobile_shop
                .next_delivery_point_eta
                .map(to_timestamp)
                .transpose()?,
            pick_up_delivery_point_eta: db_mobile_shop
                .pick_up_delivery_point_eta
                .map(to_timestamp)
                .transpose()?,
            route: None,
            phone_number: db_mobile_shop.phone_number.unwrap_or_default(),
            production_client_id: "".to_string(),
        })
    }
}
