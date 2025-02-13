use crate::{container::operations::AcquireLeaseResponse, prelude::*};
use azure_core::headers::*;
use azure_core::Method;

pub type RenewLeaseResponse = AcquireLeaseResponse;

operation! {
    RenewLease,
    client: ContainerLeaseClient,
}

impl RenewLeaseBuilder {
    pub fn into_future(mut self) -> RenewLease {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("restype", "container");
            url.query_pairs_mut().append_pair("comp", "lease");

            let mut headers = Headers::new();
            headers.insert(LEASE_ACTION, "renew");
            headers.add(self.client.lease_id());

            let mut request = self
                .client
                .finalize_request(url, Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            RenewLeaseResponse::from_headers(response.headers())
        })
    }
}
