#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RestEndpointArgs {
    pub name: String,
    #[serde(default)]
    pub headers: Vec<HttpHeaderMapping>,
    #[serde(rename = "baseURL")]
    pub base_url: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HttpHeaderMapping {
    pub name: String,
    pub value: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rest<'a> {
    pub endpoint: &'a str,
    #[serde(borrow)]
    pub http: ConnectHttp<'a>,
    #[serde(borrow)]
    pub selection: Option<&'a str>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ConnectHttp<'a> {
    #[serde(flatten, borrow)]
    pub method_path: HttpMethodPath<'a>,
    #[serde(borrow)]
    pub body: Option<Body<'a>>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HttpMethodPath<'a> {
    Connect(&'a str),
    Delete(&'a str),
    Get(&'a str),
    Head(&'a str),
    Options(&'a str),
    Post(&'a str),
    Put(&'a str),
    Patch(&'a str),
    Trace(&'a str),
}

impl<'a> HttpMethodPath<'a> {
    pub fn split(self) -> (http::Method, &'a str) {
        match self {
            HttpMethodPath::Get(path) => (http::Method::GET, path),
            HttpMethodPath::Post(path) => (http::Method::POST, path),
            HttpMethodPath::Put(path) => (http::Method::PUT, path),
            HttpMethodPath::Delete(path) => (http::Method::DELETE, path),
            HttpMethodPath::Patch(path) => (http::Method::PATCH, path),
            HttpMethodPath::Head(path) => (http::Method::HEAD, path),
            HttpMethodPath::Options(path) => (http::Method::OPTIONS, path),
            HttpMethodPath::Trace(path) => (http::Method::TRACE, path),
            HttpMethodPath::Connect(path) => (http::Method::CONNECT, path),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body<'a> {
    #[serde(borrow)]
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
