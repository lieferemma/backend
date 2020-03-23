table! {
    delivery_points (delivery_point_id) {
        delivery_point_id -> Uuid,
        position -> Geography,
        scheduled_time -> Timestamptz,
        departure_time -> Timestamptz,
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

allow_tables_to_appear_in_same_query!(
    delivery_points,
    spatial_ref_sys,
);
