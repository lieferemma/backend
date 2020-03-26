use crate::{
    api::grpc,
    db,
    db::{
        models,
        schema::{delivery_points, mobile_shops},
    },
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::{prelude::*, r2d2::ConnectionManager, PgConnection};
use diesel_geography::types::GeogPoint;
use prost_types::Timestamp;
use r2d2::PooledConnection;
use std::convert::{TryFrom, TryInto};

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::pg::Pg;

    #[test]
    fn test_query_mobile_shops() {
        let implicit_on_clause = mobile_shops::table.inner_join(
            delivery_points::table.on(mobile_shops::route_id.eq(delivery_points::route_id)),
        );
        let implicit_on_clause_sql =
            dbg!(diesel::debug_query::<Pg, _>(&implicit_on_clause).to_string());

        let pg_connection_manager =
            ConnectionManager::new("postgres://postgres:changeme@localhost/lieferemma");
        let pg_connection_pool = r2d2::Pool::new(pg_connection_manager).unwrap();
        let pg_connection = pg_connection_pool.get().unwrap();

        mobile_shops(pg_connection);
    }
}

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

impl TryFrom<models::DeliveryPoint> for grpc::DeliveryPoint {
    type Error = anyhow::Error;
    fn try_from(delivery_point: models::DeliveryPoint) -> Result<Self> {
        Ok(Self {
            delivery_point_uuid: delivery_point.delivery_point_id.to_string(),
            position: Some(delivery_point.position.into()),
            scheduled_time: Some(to_timestamp(delivery_point.scheduled_time)?),
            departure_time: Some(to_timestamp(delivery_point.departure_time)?),
        })
    }
}

fn to_timestamp(dt: DateTime<Utc>) -> Result<Timestamp> {
    let nanos = dt.timestamp_subsec_nanos().try_into()?;
    Ok(Timestamp {
        seconds: dt.timestamp(),
        nanos,
    })
}

pub fn mobile_shops(
    pg_connection: PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<grpc::MobileShop>> {
    let joined = mobile_shops::table
        .inner_join(delivery_points::table.on(mobile_shops::route_id.eq(delivery_points::route_id)))
        .limit(30)
        .load::<(models::MobileShop, models::DeliveryPoint)>(&pg_connection)?;

    println!("{:#?}", joined);

    // TODO

    joined
        .into_iter()
        .map(|(db_mobile_shop, delivery_points)| {
            let route = grpc::Route {
                route_uuid: db_mobile_shop.route_id.to_string(),
                route_points: Vec::new(),
                delivery_points: vec![delivery_points.try_into()?],
            };

            Ok(grpc::MobileShop {
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
                route: Some(route),
                phone_number: db_mobile_shop.phone_number.unwrap_or_default(),
                production_client_id: db_mobile_shop.production_client_id.unwrap_or_default(),
            })
        })
        .collect()
}
