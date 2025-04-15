#[derive(serde::Deserialize)]
pub(crate) struct GrpcConfiguration {
    pub(crate) services: Vec<Service>,
}

#[derive(serde::Deserialize)]
pub(crate) struct Service {
    pub(crate) name: String,
    pub(crate) address: String,
}
