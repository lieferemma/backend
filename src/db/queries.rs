use super::models::MobileShop;
use crate::db::schema::mobile_shops::dsl::*;
use anyhow::Result;
use diesel::prelude::*;

fn query_mobile_shops(database_url: &str) -> Result<Vec<MobileShop>> {
    let connection = PgConnection::establish(database_url)?;

    let results = mobile_shops.limit(5).load::<MobileShop>(&connection)?;

    Ok(results)
}
