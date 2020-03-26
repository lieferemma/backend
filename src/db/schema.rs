table! {
    use crate::db::custom_types::Delivery_status;
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    /// Representation of the `delivery_points` table.
    ///
    /// (Automatically generated by Diesel.)
    delivery_points (delivery_point_id) {
        /// The `delivery_point_id` column of the `delivery_points` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        delivery_point_id -> Uuid,
        /// The `position` column of the `delivery_points` table.
        ///
        /// Its SQL type is `Geography`.
        ///
        /// (Automatically generated by Diesel.)
        position -> Geography,
        /// The `scheduled_time` column of the `delivery_points` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        scheduled_time -> Timestamptz,
        /// The `departure_time` column of the `delivery_points` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        departure_time -> Timestamptz,
    }
}

table! {
    use crate::db::custom_types::Delivery_status;
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    /// Representation of the `mobile_shops` table.
    ///
    /// (Automatically generated by Diesel.)
    mobile_shops (mobile_shop_id) {
        /// The `mobile_shop_id` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        mobile_shop_id -> Uuid,
        /// The `image_url` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        image_url -> Nullable<Varchar>,
        /// The `title` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Varchar,
        /// The `phone_number` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        phone_number -> Nullable<Varchar>,
        /// The `current_position` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Nullable<Geography>`.
        ///
        /// (Automatically generated by Diesel.)
        current_position -> Nullable<Geography>,
        /// The `last_seen` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        last_seen -> Nullable<Timestamptz>,
        /// The `current_delivery_point_id` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        current_delivery_point_id -> Nullable<Uuid>,
        /// The `next_delivery_point_id` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        next_delivery_point_id -> Uuid,
        /// The `delivery_status` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Nullable<Delivery_status>`.
        ///
        /// (Automatically generated by Diesel.)
        delivery_status -> Nullable<Delivery_status>,
        /// The `next_delivery_point_eta` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        next_delivery_point_eta -> Nullable<Timestamptz>,
        /// The `pick_up_delivery_point_eta` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        pick_up_delivery_point_eta -> Nullable<Timestamptz>,
        /// The `route_id` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        route_id -> Uuid,
        /// The `production_client_id` column of the `mobile_shops` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        production_client_id -> Nullable<Varchar>,
    }
}

table! {
    use crate::db::custom_types::Delivery_status;
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    /// Representation of the `routes` table.
    ///
    /// (Automatically generated by Diesel.)
    routes (route_id) {
        /// The `route_id` column of the `routes` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        route_id -> Uuid,
    }
}

table! {
    use crate::db::custom_types::Delivery_status;
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    /// Representation of the `routes_delivery_points` table.
    ///
    /// (Automatically generated by Diesel.)
    routes_delivery_points (routes_delivery_point_id) {
        /// The `routes_delivery_point_id` column of the `routes_delivery_points` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        routes_delivery_point_id -> Uuid,
        /// The `route_id` column of the `routes_delivery_points` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        route_id -> Uuid,
        /// The `delivery_point_id` column of the `routes_delivery_points` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        delivery_point_id -> Uuid,
    }
}

table! {
    use crate::db::custom_types::Delivery_status;
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    /// Representation of the `routes_route_points` table.
    ///
    /// (Automatically generated by Diesel.)
    routes_route_points (route_point_id) {
        /// The `route_point_id` column of the `routes_route_points` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        route_point_id -> Uuid,
        /// The `route_id` column of the `routes_route_points` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        route_id -> Uuid,
        /// The `position` column of the `routes_route_points` table.
        ///
        /// Its SQL type is `Geography`.
        ///
        /// (Automatically generated by Diesel.)
        position -> Geography,
    }
}

table! {
    use crate::db::custom_types::Delivery_status;
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    /// Representation of the `spatial_ref_sys` table.
    ///
    /// (Automatically generated by Diesel.)
    spatial_ref_sys (srid) {
        /// The `srid` column of the `spatial_ref_sys` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        srid -> Int4,
        /// The `auth_name` column of the `spatial_ref_sys` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        auth_name -> Nullable<Varchar>,
        /// The `auth_srid` column of the `spatial_ref_sys` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        auth_srid -> Nullable<Int4>,
        /// The `srtext` column of the `spatial_ref_sys` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        srtext -> Nullable<Varchar>,
        /// The `proj4text` column of the `spatial_ref_sys` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        proj4text -> Nullable<Varchar>,
    }
}

joinable!(mobile_shops -> routes (route_id));
joinable!(routes_delivery_points -> delivery_points (delivery_point_id));
joinable!(routes_delivery_points -> routes (route_id));
joinable!(routes_route_points -> routes (route_id));

allow_tables_to_appear_in_same_query!(
    delivery_points,
    mobile_shops,
    routes,
    routes_delivery_points,
    routes_route_points,
    spatial_ref_sys,
);
