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
