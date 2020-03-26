DROP TABLE routes_route_points;

ALTER TABLE delivery_points ADD COLUMN route_id UUID NOT NULL;

CREATE INDEX delivery_points_idx ON delivery_points (route_id);
