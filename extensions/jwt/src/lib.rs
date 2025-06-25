mod config;
mod decoder;

use config::{Config, Location};
use decoder::Decoder;
use grafbase_sdk::{
    AuthenticationExtension,
    types::{Configuration, Error, ErrorResponse, GatewayHeaders, HttpHeaders, PublicMetadataEndpoint, Token},
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
            .unwrap_or_else(|| {
                let mut headers = HttpHeaders::new();

                if let Some(metadata_endpoint) = self.config.oauth.as_ref().map(|oauth| &oauth.path) {
                    headers.append(
                        "WWW-Authenticate",
                        format!("Bearer resource_metadata=\"{metadata_endpoint}\""),
                    );
                }

                Err(ErrorResponse::unauthorized().with_error("Unauthorized"))
            })
    }

    fn public_metadata(&mut self) -> Result<Vec<PublicMetadataEndpoint>, Error> {
        let Some(oauth) = &self.config.oauth else {
            return Ok(vec![]);
        };

        let mut metadata = oauth.metadata.other_parameters.clone();
        metadata.insert("resource".to_owned(), oauth.metadata.resource.clone().into());

        metadata
            .entry("jwks_uri".to_owned())
            .or_insert_with(|| self.config.url.to_string().into());

        let response_body = serde_json::to_vec(&metadata).map_err(|err| {
            Error::new(format!(
                "Failed to serialize response body for public metadata endpoint: {err}",
            ))
        })?;

        let mut headers = HttpHeaders::new();
        headers.append("Content-Type", "application/json");

        Ok(vec![
            PublicMetadataEndpoint::new(oauth.path.clone(), response_body).with_headers(headers),
        ])
    }
}
