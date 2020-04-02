
-- maps to shipment status in proto messages
CREATE TYPE order_status AS ENUM ('CREATED', 'LOADED', 'SENT', 'COLLECTED');

CREATE TYPE payment_status AS ENUM ('NOTPAYED', 'PAYED');


CREATE TABLE oder_status_updates (
    order_uuid UUID NOT NULL REFERENCES orders,

    update_time TIMESTAMPTZ NOT NULL,

    current_status order_status,

    PRIMARY KEY (order_uuid)
);

CREATE TABLE payment_status (
    order_uuid UUID NOT NULL REFERENCES orders,

    update_time TIMESTAMPTZ NOT NULL,

    current_status payment_status,

    PRIMARY KEY (order_uuid)
);


CREATE TABLE order_items (
    product_id UUID NOT NULL REFERENCES products ON DELETE RESTRICT,

    order_uuid UUID NOT NULL REFERENCES orders ON DELETE CASCADE,

    quantity BIGINT NOT NULL,

    PRIMARY KEY (product_id, order_uuid)
);

CREATE TABLE orders (
    order_uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- human readable order id
    order_id VARCHAR(1024) NOT NULL,
    -- Shop that will receive the payment
    
    mobile_shop_uuid UUID NOT NULL REFERENCES mobile_shops(mobile_shop_id),


    oder_status oder_status NOT NULL REFERENCES oder_status_updates(order_uuid),

    delivery_point_id UUID NOT NULL REFERENCES delivery_points(delivery_point_id),

    
   -- Currency to display
    currency currency_codes,
    -- calculated total of the order
    total BIGINT NOT NULL,
    -- Items that have been ordered by the customer
    ordered_products NOT NULL REFERENCES order_items(order_uuid)
    
);

