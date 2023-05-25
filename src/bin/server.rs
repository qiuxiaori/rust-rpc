#![feature(type_alias_impl_trait)]
use eas_rpc_server::layer::log::LogLayer;
use eas_rpc_server::router::cap_placement::P;
use eas_rpc_server::router::health::H;
use std::{env, net::SocketAddr};
use volo_grpc::server::{Server, ServiceBuilder};

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let host = env::var("server_host").unwrap();
    let port = env::var("server_port").unwrap();

    let addr: SocketAddr = (host + ":" + &port).parse().unwrap();
    let addr = volo::net::Address::from(addr);
    tracing::info!("Server listening at {:?}", addr);

    Server::new()
        .add_service(
            ServiceBuilder::new(volo_gen::cap_placement::PlaceServiceServer::new(P)).build(),
        )
        .add_service(ServiceBuilder::new(volo_gen::health::HealthServiceServer::new(H)).build())
        .layer_front(LogLayer)
        .run(addr)
        .await
        .unwrap();
}
