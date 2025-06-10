use crate::{conversions, schema};
use grafbase_sdk::{
    Subscription,
    host_io::grpc::GrpcStreamingResponse,
    types::{Error, Response, SubscriptionItem},
};

pub(crate) struct StreamingResponse<'a> {
    pub(super) response: GrpcStreamingResponse,
    pub(super) output_message: &'a schema::Message,
    pub(super) schema: &'a schema::Schema,
}

impl Subscription for StreamingResponse<'_> {
    fn next(&mut self) -> Result<Option<SubscriptionItem>, Error> {
        let response_proto = match self.response.next_message() {
            Ok(Some(message)) => message,
            Ok(None) => return Ok(None),
            Err(err) => {
                return Err(Error::new(format!(
                    "gRPC error. Status code: {:?}. Message: {}",
                    err.code(),
                    err.message()
                )));
            }
        };

        Ok(Some(
            Response::data(conversions::MessageSerialize::new(
                &response_proto.into(),
                self.output_message,
                self.schema,
            ))
            .into(),
        ))
    }
}
