mod config;
mod decoder;

use std::time::Instant;

use config::{Config, Location};
use decoder::Decoder;
use grafbase_sdk::{
    AuthenticationExtension,
    host_io::{
        cache::Cache,
        http::{self, HttpRequest},
        logger::log,
    },
    types::{Configuration, Error, ErrorResponse, GatewayHeaders, Headers, PublicMetadataEndpoint, Token},
};

use crate::decoder::Jwks;

#[derive(AuthenticationExtension)]
struct Jwt {
    pub config: Config,
    jwks_cache: Cache,
    jwks: Option<(Jwks, Instant)>,
}

impl AuthenticationExtension for Jwt {
    fn new(config: Configuration) -> Result<Self, Error> {
        let config: Config = config.deserialize()?;

        Ok(Self {
            jwks_cache: Cache::builder("jwks", 1).timeout(config.poll_interval).build(),
            jwks: None,
            config,
        })
    }

    fn authenticate(&mut self, headers: &GatewayHeaders) -> Result<Token, ErrorResponse> {
        let mut decoder = self.decoder()?;

        decoder
            .config
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
                let mut headers = Headers::new();

                if let Some(metadata_endpoint) = self
                    .config
                    .oauth
                    .as_ref()
                    .map(|oauth| &oauth.protected_resource.metadata_path)
                {
                    headers.append(
                        "WWW-Authenticate",
                        format!("Bearer resource_metadata=\"{metadata_endpoint}\""),
                    );
                }

                let mut error_response = ErrorResponse::unauthorized().with_error("Unauthorized");

                for header in &self.config.unauthenticated_headers {
                    error_response.push_header(&header.name, header.value.as_bytes()).ok();
                }

                Err(error_response)
            })
    }

    fn public_metadata(&mut self) -> Result<Vec<PublicMetadataEndpoint>, Error> {
        let Some(oauth) = &self.config.oauth else {
            return Ok(vec![]);
        };

        let mut metadata = oauth.protected_resource.metadata.other_parameters.clone();
        metadata.insert(
            "resource".to_owned(),
            oauth.protected_resource.metadata.resource.clone().into(),
        );

        metadata
            .entry("jwks_uri".to_owned())
            .or_insert_with(|| self.config.url.to_string().into());

        let response_body = serde_json::to_vec(&metadata).map_err(|err| {
            Error::new(format!(
                "Failed to serialize response body for public metadata endpoint: {err}",
            ))
        })?;

        let mut headers = Headers::new();
        headers.append("Content-Type", "application/json");

        Ok(vec![
            PublicMetadataEndpoint::new(oauth.protected_resource.metadata_path.clone(), response_body)
                .with_headers(headers),
        ])
    }
}

impl Jwt {
    fn decoder(&mut self) -> Result<Decoder<'_>, ErrorResponse> {
        if self
            .jwks
            .as_ref()
            .is_none_or(|(_, ts)| ts.elapsed() > self.config.poll_interval)
        {
            let ts = Instant::now();
            let (jwks, bytes) = self
                .jwks_cache
                .get_or_insert(self.config.url.as_str(), || {
                    let request = HttpRequest::get(self.config.url.clone()).build();
                    let response = http::execute(request)?;
                    let bytes = response.into_bytes();
                    let jwks: Jwks = serde_json::from_slice(&bytes).map_err(|err| err.to_string())?;
                    Ok((jwks, bytes))
                })
                .map_err(|err: Error| {
                    log::error!("Failed to retrieve JWKS: {err}");
                    ErrorResponse::internal_server_error()
                })?;
            let jwks: Jwks = match jwks {
                Some(jwks) => jwks,
                _ => serde_json::from_slice(&bytes).map_err(|err| {
                    log::error!("Failed to parse JWKS: {err}");
                    ErrorResponse::internal_server_error()
                })?,
            };
            self.jwks = Some((jwks, ts));
        }
        Ok(Decoder {
            config: &self.config,
            jwks: &self.jwks.as_ref().unwrap().0,
        })
    }
}
