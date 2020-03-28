use anyhow::Result;
use diesel::r2d2::ConnectionManager;
use futures::{prelude::*, select};
use lieferemma::{
    grpc::{
        end_customer_client::EndCustomerClient, Currency, OrderRequest, OrderRequestProduct,
        TransactionCompleteRequest,
    },
    DriverServer, DriverServerImpl, EndCustomerServer, EndCustomerServerImpl,
};
use tonic::transport::{Channel, Server};

async fn run_server() -> Result<()> {
    let database_url = "postgres://postgres:changeme@localhost/lieferemma";

    let pg_connection_manager = ConnectionManager::new(database_url);
    let pg_connection_pool = r2d2::Pool::new(pg_connection_manager).unwrap();

    let end_customer_server = EndCustomerServerImpl { pg_connection_pool };
    let driver_server = DriverServerImpl {};

    Server::builder()
        .add_service(EndCustomerServer::new(end_customer_server))
        .add_service(DriverServer::new(driver_server))
        .serve("127.0.0.1:50051".parse()?)
        .await?;

    Ok(())
}

async fn place_order(client: &mut EndCustomerClient<Channel>) -> Result<()> {
    let ordered_products = vec![OrderRequestProduct {
        quantity_ordered: 10,
        product_uuid: "38ac554d-e2b4-4105-b6be-04b2daf7733e".to_string(),
    }];

    let request = tonic::Request::new(OrderRequest {
        shop_uuid: "04abad45-dde2-440b-9690-a5a99b2a154e".to_string(),
        pick_up_point: None,
        currency: Currency::Eur as i32,
        ordered_products,
    });

    let response = client.place_order(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

async fn complete_transaction(client: &mut EndCustomerClient<Channel>) -> Result<()> {
    let request = TransactionCompleteRequest {
        order_uuid: "76881098-fd2f-4374-aa0d-10d3cc2a9b1e".to_string(),
        paypal_transaction_id: "PAY-NONETWORKPAYIDEXAMPLE123".to_string(),
    };

    let response = client.complete_transaction(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

async fn process() -> Result<()> {
    let mut client = EndCustomerClient::connect("http://127.0.0.1:50051").await?;
    place_order(&mut client).await?;
    complete_transaction(&mut client).await?;

    Ok(())
}

#[tokio::test]
async fn order_test() -> Result<(), Box<dyn std::error::Error>> {
    select! {
        e = run_server().fuse() => panic!("Server quit unexpectetly: {:?}", e),
        _ = process().fuse() => (),
    }

    Ok(())
}
