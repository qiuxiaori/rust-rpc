use dotenv::dotenv;
use std::{
    env,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    str::FromStr,
};

// 服务检测用
#[volo::main]
async fn main() {
    dotenv().ok();
    let host = env::var("client_host").unwrap();
    let ip = Ipv4Addr::from_str(&host).unwrap();

    let port = env::var("client_port").unwrap();
    let port = port.parse::<u16>().unwrap();

    let client: volo_gen::health::HealthServiceClient = {
        let addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(ip, port));
        volo_gen::health::HealthServiceClientBuilder::new("health")
            .address(addr)
            .build()
    };

    tracing_subscriber::fmt::init();
    let req = volo_gen::health::HealthCheckRequest {
        service: "health".to_string().into(),
    };
    let resp = client.check(req).await;
    match resp {
        Ok(info) => tracing::info!("{:?}", info),
        Err(e) => tracing::error!("{:?}", e),
    }
}
