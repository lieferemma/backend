# Lieferemma - Backend

![CI](https://github.com/lieferemma/backend/workflows/CI/badge.svg)

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

# Run

Build and run from source:

```
RUST_LOG=debug cargo run
```

# Architecture

Database: [PostgreSQL](https://www.postgresql.org/)
Object Storage: [MinIO](https://min.io/)
API: [gRPC](gRPC.io/)

# Database

- Host name/address postgres
- Port 5432
- Username as POSTGRES_USER, by default: postgres
- Password as POSTGRES_PASSWORD, by default changeme
