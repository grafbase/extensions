use grafbase_sdk::{
    HooksExtension,
    host_io::event_queue::EventQueue,
    host_io::http::{Method, StatusCode},
    types::{Configuration, Error, ErrorResponse, GatewayHeaders},
};

#[derive(HooksExtension)]
struct WwwAuthenticate {
    config: WwwAuthenticateConfig,
}

#[derive(serde::Deserialize)]
struct WwwAuthenticateConfig {
    www_authenticate_header_value: String,
}

impl HooksExtension for WwwAuthenticate {
    fn new(config: Configuration) -> Result<Self, Error> {
        let config = config.deserialize()?;

        Ok(Self { config })
    }

    fn on_request(&mut self, _url: &str, _method: Method, _headers: &mut GatewayHeaders) -> Result<(), ErrorResponse> {
        Ok(())
    }

    fn on_response(
        &mut self,
        status: StatusCode,
        headers: &mut GatewayHeaders,
        _event_queue: EventQueue,
    ) -> Result<(), String> {
        if status == StatusCode::UNAUTHORIZED {
            headers.append("WWW-Authenticate", &self.config.www_authenticate_header_value);
        }

        Ok(())
    }
}
