use crate::api::grpc;
use anyhow::Result;
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use sqlx::{
    PgPool, {self},
};
use std::convert::TryInto;
use uuid::Uuid;

fn to_timestamp(dt: DateTime<Utc>) -> Result<Timestamp> {
    let nanos = dt.timestamp_subsec_nanos().try_into()?;
    Ok(Timestamp {
        seconds: dt.timestamp(),
        nanos,
    })
}

#[derive(Debug, sqlx::FromRow)]
pub struct MobileShopWithDeliveryPoint {
    pub mobile_shop_id: Uuid,
    pub image_url: Option<String>,
    pub title: String,
    pub phone_number: Option<String>,
    pub last_seen: Option<DateTime<Utc>>,
    pub current_delivery_point_id: Option<Uuid>,
    pub next_delivery_point_id: Uuid,
    pub next_delivery_point_eta: Option<DateTime<Utc>>,
    pub pick_up_delivery_point_eta: Option<DateTime<Utc>>,
    pub route_id: Uuid,
    pub production_client_id: Option<String>,
    pub delivery_point_id: Uuid,
    pub scheduled_time: DateTime<Utc>,
    pub departure_time: DateTime<Utc>,
}

pub async fn mobile_shops(pg_pool: &PgPool) -> Result<Vec<grpc::MobileShop>> {
    let mut tx = pg_pool.begin().await?;
    let recs = sqlx::query_as!(
        MobileShopWithDeliveryPoint,
        "
SELECT
ms.mobile_shop_id,
ms.image_url,
ms.title,
ms.phone_number,
ms.last_seen,
ms.current_delivery_point_id,
ms.next_delivery_point_id,
ms.next_delivery_point_eta,
ms.pick_up_delivery_point_eta,
ms.route_id,
ms.production_client_id,
dp.delivery_point_id,
dp.scheduled_time,
dp.departure_time
FROM mobile_shops ms
INNER JOIN delivery_points dp ON ms.route_id = dp.route_id
        "
    )
    .fetch_all(&mut tx)
    .await?;

    println!("{:#?}", recs);

    recs.into_iter()
        .map(|row| {
            let route = grpc::Route {
                route_uuid: row.route_id.to_string(),
                route_points: Vec::new(),
                delivery_points: vec![], // TODO
            };

            Ok(grpc::MobileShop {
                mobile_shop_uuid: row.mobile_shop_id.to_string(),
                image_url: row.image_url.unwrap_or_default(),
                title: row.title,
                current_position: Default::default(), // TODO
                last_seen: row.last_seen.map(to_timestamp).transpose()?,
                current_delivery_point: None,
                next_delivery_point: None,
                delivery_status: Default::default(), // TODO,
                next_delivery_point_eta: row
                    .next_delivery_point_eta
                    .map(to_timestamp)
                    .transpose()?,
                pick_up_delivery_point_eta: row
                    .pick_up_delivery_point_eta
                    .map(to_timestamp)
                    .transpose()?,
                route: Some(route),
                phone_number: row.phone_number.unwrap_or_default(),
                production_client_id: row.production_client_id.unwrap_or_default(),
            })
        })
        .collect()
}
