//! TEMPLATE_NOTE(canardleteer): Overall, it's better if you build your
//!                              Service, stand alone from any Web Service
//!                              Framework implementation. We do this for
//!                              brevity here, but, you should consider making
//!                              the Service itself gRPC agnostic, and then
//!                              offering various `mod grpc::version` adapters
//!                              for the specific Framework + RPC API you are
//!                              offering.
use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{Request, Response, Status};
use tracing::instrument;

use {{crate_name}}_bindings::svc_v1alpha1_decl::{
    simple_timestamp_service_server::SimpleTimestampService, WhatTimeIsItRequest,
    WhatTimeIsItResponse,
};

#[derive(Default, Debug)]
pub struct TimeService {}

#[tonic::async_trait]
impl SimpleTimestampService for TimeService {
    /// We don't do a lot more then return the current time on this host.
    #[instrument(level = "info")]
    async fn what_time_is_it(
        &self,
        _request: Request<WhatTimeIsItRequest>,
    ) -> Result<Response<WhatTimeIsItResponse>, Status> {
        let since_the_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Status::internal("the service is time travelling again"))?;

        Ok(Response::new(WhatTimeIsItResponse {
            seconds_since_epoch: since_the_epoch.as_secs(),
        }))
    }
}
