CREATE TABLE delivery_points (
  delivery_point_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  position GEOGRAPHY NOT NULL,
  scheduled_time TIMESTAMPTZ NOT NULL,
  departure_time TIMESTAMPTZ NOT NULL
);
