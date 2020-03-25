use diesel::{
    deserialize::{self, FromSql},
    pg::Pg,
    serialize::{self, IsNull, Output, ToSql},
    *,
};
use std::io::Write;

#[allow(non_camel_case_types)]
#[derive(SqlType)]
#[postgres(type_name = "delivery_status")]
pub struct Delivery_status;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Delivery_status"]
pub enum DeliveryStatus {
    OnTour,
    Parked,
}

impl ToSql<Delivery_status, Pg> for DeliveryStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            DeliveryStatus::OnTour => out.write_all(b"ON_TOUR")?,
            DeliveryStatus::Parked => out.write_all(b"PARKED")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Delivery_status, Pg> for DeliveryStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"ON_TOUR" => Ok(DeliveryStatus::OnTour),
            b"PARKED" => Ok(DeliveryStatus::Parked),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
