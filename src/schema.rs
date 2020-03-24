table! {
    delivery_points (delivery_point_id) {
        delivery_point_id -> Uuid,
        position -> Geography,
        scheduled_time -> Timestamptz,
        departure_time -> Timestamptz,
    }
}

table! {
    mobile_shops (mobile_shop_id) {
        mobile_shop_id -> Uuid,
        image_url -> Nullable<Varchar>,
        title -> Varchar,
        current_position -> Nullable<Geography>,
        last_seen -> Nullable<Timestamptz>,
        current_delivery_point_id -> Nullable<Uuid>,
        next_delivery_point_id -> Uuid,
        delivery_status -> Nullable<Delivery_status>,
        next_delivery_point_eta -> Nullable<Timestamptz>,
        pick_up_delivery_point_eta -> Nullable<Timestamptz>,
        route_id -> Uuid,
    }
}

table! {
    routes (route_id) {
        route_id -> Uuid,
    }
}

table! {
    routes_delivery_points (routes_delivery_point_id) {
        routes_delivery_point_id -> Uuid,
        route_id -> Uuid,
        delivery_point_id -> Uuid,
    }
}

table! {
    routes_route_points (route_point_id) {
        route_point_id -> Uuid,
        route_id -> Uuid,
        position -> Geography,
    }
}

table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
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
