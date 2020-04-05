CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "postgis";

CREATE TABLE delivery_points (
  delivery_point_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  route_id UUID NOT NULL,
  position GEOGRAPHY NOT NULL,
  scheduled_time TIMESTAMPTZ NOT NULL,
  departure_time TIMESTAMPTZ NOT NULL
);

CREATE INDEX delivery_points_idx ON delivery_points (route_id);

CREATE TABLE routes (
  route_id UUID PRIMARY KEY DEFAULT uuid_generate_v4()
);

CREATE TABLE routes_delivery_points (
    routes_delivery_point_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    route_id UUID NOT NULL REFERENCES routes(route_id),
    delivery_point_id UUID NOT NULL REFERENCES delivery_points(delivery_point_id)
);

CREATE TYPE delivery_status AS ENUM ('on_tour', 'parked');

CREATE TABLE mobile_shops (
    mobile_shop_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- link to image of shop
    image_url VARCHAR(1024),
    -- Title of delivery to be displayed to customer e.g. Bakery John Doe
    title VARCHAR(1024) NOT NULL,
    -- Phone number the customer can call if she has questions about the products
    phone_number VARCHAR(80),
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
    route_id UUID NOT NULL REFERENCES routes(route_id),
    -- Payment transaction id
    production_client_id varchar(64)
);
