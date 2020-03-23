# Lieferemma - Backend
![CI](https://github.com/lieferemma/backend/workflows/CI/badge.svg)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Cargo-Deny](https://img.shields.io/badge/cargo--deny-Dependencies%20checked-blue)
![Docker-Image](https://img.shields.io/docker/image-size/lieferemma/backend)

# Current State
- [ ] Database setup
  - [x] add PostgreSQL to docker-compose
  - [ ] Properly configure auth credentials
  - [ ] Helm Chart
  - [ ] Set up DB migrations using [diesel](https://diesel.rs)
- [ ] Object Storage
  - [x] add MinIO to docker-compose
  - [ ] Properly configure auth credentials
- [ ] EndCustomer API Implementation
  - [x] Set up initial static API
  - [ ] Connect to database
  - [ ] Connect to MinIO for article and shop image links
- [ ] Driver API Implementation
  - [x] Set up initial static API
  - [ ] Connect to database
  - [ ] Connect to MinIO for article and shop image links

# Getting started

## Run backend
Build and run from source using docker-compose:

```
docker-compose up -d
```

To avoid rebuilding the backend docker image during development you should directly run cargo:

```
RUST_LOG=debug cargo run
```

## Prepare database
This projects uses ["Diesel"](https://diesel.rs/) for database migrations.
The database migrations create the initial database schema, set up the required Postgres extensions and in general allow updating the schema of an existing database.
You must install the [diesel cli tool](https://diesel.rs/guides/getting-started/) in order to run the database migrations.

To run the migrations in a dev setup run:
```
source .dev.env
diesel migration run
```

# Architecture
- Database: [PostgreSQL](https://www.postgresql.org/)
  - Extension: [PostGIS](https://postgis.net/) - Spatial database extension of PostgreSQL
  - Extension: [uuid-ossp](https://www.postgresql.org/docs/current/uuid-ossp.html) - generate UUIDs using one of several standard algorithms
- Object Storage: [MinIO](https://min.io/)
- API: [gRPC](https://gRPC.io/)

# Database
- Host name/address postgres
- Port 5432
- Username as POSTGRES_USER, by default: postgres
- Password as POSTGRES_PASSWORD, by default changeme

## Pgadmin4
- Host name/address pgadmin
- Port 5050
- Username as PGADMIN_DEFAULT_EMAIL, by default: pgadmin4@pgadmin.org
- Password as PGADMIN_DEFAULT_PASSWORD, by default: admin
