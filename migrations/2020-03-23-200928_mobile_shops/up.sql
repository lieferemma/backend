
CREATE TYPE delivery_status AS ENUM ('ON_TOUR', 'PARKED');

CREATE TABLE mobile_shops (
    mobile_shop_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- link to image of shop
    image_url VARCHAR(1024),
    -- Title of delivery to be displayed to customer e.g. Bakery John Doe
    title VARCHAR(1024) NOT NULL,
    -- Last location updated
    current_position GEOGRAPHY,
    -- Last location update
    last_seen TIMESTAMPTZ,
    -- Current delivery point
    current_delivery_point_id UUID REFERENCES delivery_points(delivery_point_id),
    -- Next delivery point
    next_delivery_point_id UUID NOT NULL REFERENCES delivery_points(delivery_point_id),
    -- Is the delivery vehicle currently stationary or not
    delivery_status delivery_status,
    -- Estimated time of arrival at next delivery point
    next_delivery_point_eta TIMESTAMPTZ,
    -- Estimated time of arrival at the pick up delivery point
    pick_up_delivery_point_eta TIMESTAMPTZ,
    -- Current route of this mobile shop
    route_id UUID NOT NULL REFERENCES routes(route_id)
);