#![feature(type_alias_impl_trait)]
pub struct H;

use volo_gen::health::*;

#[volo::async_trait]
impl HealthService for H {
    async fn check(
        &self,
        _req: volo_grpc::Request<HealthCheckRequest>,
    ) -> core::result::Result<volo_grpc::Response<HealthCheckResponse>, volo_grpc::Status> {
        Ok(volo_grpc::Response::new(HealthCheckResponse {
            status: ServingStatus::Serving,
        }))
    }
}
