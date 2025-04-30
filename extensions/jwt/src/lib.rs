mod config;
mod decoder;

use config::{Config, Location};
use decoder::Decoder;
use grafbase_sdk::{
    AuthenticationExtension,
    types::{Configuration, Error, ErrorResponse, GatewayHeaders, Token},
};

#[derive(AuthenticationExtension)]
struct Jwt {
    pub config: Config,
}

impl AuthenticationExtension for Jwt {
    fn new(config: Configuration) -> Result<Self, Error> {
        let config = config.deserialize()?;

        Ok(Self { config })
    }

    fn authenticate(&mut self, headers: &GatewayHeaders) -> Result<Token, ErrorResponse> {
        let mut decoder = Decoder::new(&self.config);

        self.config
            .locations
            .iter()
            .find_map(|location| match location {
                Location::Header { name, value_prefix } => headers.get(name).and_then(|value| {
                    value
                        .to_str()
                        .ok()
                        .and_then(|s| match value_prefix {
                            Some(prefix) => s.strip_prefix(prefix),
                            None => Some(s),
                        })
                        .and_then(|token_str| decoder.decode(token_str))
                }),
                Location::Cookie { name } => headers.get("Cookie").and_then(|cookies| {
                    cookies.to_str().ok()?.split("; ").find_map(|cookie| {
                        let equal_value = cookie.strip_prefix(name)?;
                        let value = equal_value.strip_prefix("=")?;
                        decoder.decode(value)
                    })
                }),
            })
            .unwrap_or_else(|| Err(ErrorResponse::unauthorized().with_error("Unauthorized")))
    }
}
