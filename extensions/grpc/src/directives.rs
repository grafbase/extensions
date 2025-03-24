use grafbase_sdk::types::Error;

use crate::Grpc;

#[derive(Debug)]
pub(crate) struct GrpcMethod {
    pub(crate) service: String,
    pub(crate) method: String,
    pub(crate) input: Vec<u8>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoMessages {
    pub(crate) messages: Vec<ProtoMessage>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoMessage {
    pub(crate) name: String,
    pub(crate) fields: Vec<ProtoField>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoField {
    pub(crate) name: String,
    pub(crate) number: u32,
    #[serde(default)]
    pub(crate) repeated: bool,
    pub(crate) r#type: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoServices {
    pub(crate) services: Vec<ProtoService>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoService {
    pub(crate) name: String,
    pub(crate) methods: Vec<ProtoMethod>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProtoMethod {
    pub(crate) name: String,
    pub(crate) input_type: String,
    pub(crate) output_type: String,
}
