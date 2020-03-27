CREATE TYPE currency AS ENUM ('DEFAULT', 'EUR');

CREATE TYPE unit AS ENUM ('G', 'PIECE', 'METER', 'CRATE', 'BOTTLE', 'BARREL', 'ML');

CREATE TABLE products (
    product_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- Title of the product
    title VARCHAR(1024) NOT NULL,
    -- Description of the Product
    description VARCHAR(1024) NOT NULL,
    -- link to image of the product
    image_url VARCHAR(1024),
    -- Price in Euro cents
    price BIGINT NOT NULL,
    -- Currency to display
    currency currency,
    -- the unit the product is sold in e.g ( Stueck, kg, Kasten, m )
    unit unit,
    -- The max possible order size for each product ( Defined to not have to create a inventory)
    max_order BIGINT NOT NULL
);