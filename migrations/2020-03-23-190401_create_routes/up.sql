CREATE TABLE routes (
  route_id UUID PRIMARY KEY DEFAULT uuid_generate_v4()
);

CREATE TABLE routes_delivery_points (
    routes_delivery_point_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    route_id UUID NOT NULL REFERENCES routes(route_id),
    delivery_point_id UUID NOT NULL REFERENCES delivery_points(delivery_point_id)
);

-- Route delivery points further refining the route
CREATE TABLE routes_route_points (
    route_point_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    route_id UUID NOT NULL REFERENCES routes(route_id),
    position GEOGRAPHY NOT NULL
);
