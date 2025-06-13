#[derive(Debug)]
pub struct RestEndpoint {
    pub subgraph_name: String,
    pub args: RestEndpointArgs,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RestEndpointArgs {
    pub name: String,
    #[serde(rename = "baseURL")]
    pub base_url: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rest<'a> {
    pub endpoint: &'a str,
    pub method: HttpMethod,
    pub path: &'a str,
    pub selection: Option<&'a str>,
    pub body: Option<Body<'a>>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body<'a> {
    pub selection: Option<&'a str>,
    pub r#static: Option<serde_json::Value>,
}

impl<'a> Body<'a> {
    pub fn into_case(self) -> Option<BodyCase<'a>> {
        self.r#static
            .map(BodyCase::Static)
            .or_else(|| self.selection.map(BodyCase::Selection))
    }
}

pub(crate) enum BodyCase<'a> {
    Selection(&'a str),
    Static(serde_json::Value),
}

#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Trace,
    Patch,
}

impl From<HttpMethod> for ::http::Method {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::Get => Self::GET,
            HttpMethod::Post => Self::POST,
            HttpMethod::Put => Self::PUT,
            HttpMethod::Delete => Self::DELETE,
            HttpMethod::Head => Self::HEAD,
            HttpMethod::Options => Self::OPTIONS,
            HttpMethod::Connect => Self::CONNECT,
            HttpMethod::Trace => Self::TRACE,
            HttpMethod::Patch => Self::PATCH,
        }
    }
}
