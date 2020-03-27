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

#[allow(non_camel_case_types)]
#[derive(SqlType)]
#[postgres(type_name = "currency_codes")]
pub struct Currency_codes;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Currency_codes"]
pub enum CurrencyCodes {
    Eur,
    Usd,
    Chf,
}

impl ToSql<Currency_codes, Pg> for CurrencyCodes {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            CurrencyCodes::Eur => out.write_all(b"EUR")?,
            CurrencyCodes::Usd => out.write_all(b"USD")?,
            CurrencyCodes::Chf => out.write_all(b"CHF")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Currency_codes, Pg> for CurrencyCodes {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"EUR" => Ok(CurrencyCodes::Eur),
            b"USD" => Ok(CurrencyCodes::Usd),
            b"CHF" => Ok(CurrencyCodes::Chf),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(SqlType)]
#[postgres(type_name = "packaging_unit")]
pub struct Packaging_unit;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Packaging_unit"]
pub enum PackagingUnit {
    G,
    Piece,
    Meter,
    Crate,
    Bottle,
    Barrel,
    Ml,
}

impl ToSql<Packaging_unit, Pg> for PackagingUnit {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            PackagingUnit::G => out.write_all(b"G")?,
            PackagingUnit::Piece => out.write_all(b"PIECE")?,
            PackagingUnit::Bottle => out.write_all(b"BOTTLE")?,
            PackagingUnit::Meter => out.write_all(b"METER")?,
            PackagingUnit::Crate => out.write_all(b"CRATE")?,
            PackagingUnit::Barrel => out.write_all(b"BARREL")?,
            PackagingUnit::Ml => out.write_all(b"ML")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Packaging_unit, Pg> for PackagingUnit {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"G" => Ok(PackagingUnit::G),
            b"PIECE" => Ok(PackagingUnit::Piece),
            b"BOTTLE" => Ok(PackagingUnit::Bottle),
            b"METER" => Ok(PackagingUnit::Meter),
            b"CRATE" => Ok(PackagingUnit::Crate),
            b"BARREL" => Ok(PackagingUnit::Barrel),
            b"ML" => Ok(PackagingUnit::Ml),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
