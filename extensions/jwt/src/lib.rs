mod types;

use base64::Engine as _;
use grafbase_sdk::{
    AuthenticationExtension,
    host_io::{
        cache::{self, CachedItem},
        http::{self, HttpRequest},
    },
    types::{Configuration, Error, ErrorResponse, GatewayHeaders, Token},
};
use jwt_compact::{Algorithm, AlgorithmExt, TimeOptions, UntrustedToken, jwk::JsonWebKey};
use serde::de::DeserializeOwned;
use types::{Alg, CustomClaims, Jwk, Jwks, JwtConfig};

#[derive(AuthenticationExtension)]
struct Jwt {
    pub config: JwtConfig,
}

impl AuthenticationExtension for Jwt {
    fn new(config: Configuration) -> Result<Self, Error> {
        let config = config.deserialize()?;

        Ok(Self { config })
    }

    fn authenticate(&mut self, headers: &GatewayHeaders) -> Result<Token, ErrorResponse> {
        let Some(token_str) = headers.get(self.config.header_name()).and_then(|value| {
            let stripped = value.to_str().ok()?.strip_prefix(self.config.header_value_prefix());
            stripped.map(ToString::to_string)
        }) else {
            return Err(unauthorized());
        };

        let jwks: Jwks<'_> = cache::get("jwt:jwks", || {
            let request = HttpRequest::get(self.config.url.clone()).build();
            let response = http::execute(&request)?;
            let jwks: Jwks = response.json()?;

            Ok(CachedItem::new(jwks, Some(self.config.poll_interval)))
        })
        .map_err(|_| ErrorResponse::internal_server_error())?;

        let token = UntrustedToken::new(&token_str).map_err(|_| unauthorized())?;
        let token = decode_token(jwks.keys, token).ok_or_else(unauthorized)?;

        if let Some(expected) = self.config.issuer.as_ref() {
            if token.claims().custom.issuer.as_ref() != Some(expected) {
                return Err(unauthorized());
            }
        }

        if let Some(expected) = self.config.audience.as_ref() {
            let audience = token.claims().custom.audience.as_ref().ok_or_else(unauthorized)?;

            if audience.iter().all(|aud| aud != expected) {
                return Err(unauthorized());
            }
        }

        // We just validated the JWT token. Instead of de-serializing and re-serializing the
        // payload, we re-use the original token payload.
        let [_headers, payload, _signature] = token_str.split('.').collect::<Vec<_>>()[..] else {
            unreachable!("Token was successfully valdiated");
        };

        Ok(Token::from_bytes(
            base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(payload)
                .expect("Token was successfully validated"),
        ))
    }
}

fn unauthorized() -> ErrorResponse {
    ErrorResponse::unauthorized().with_error("Unauthorized")
}

fn decode_token(jwks: Vec<Jwk<'_>>, untrusted_token: UntrustedToken<'_>) -> Option<jwt_compact::Token<CustomClaims>> {
    use jwt_compact::alg::*;

    let time_options = TimeOptions::default();

    jwks.iter()
        // If 'kid' was provided, we only use the jwk with the correct id.
        .filter(|jwk| match (&untrusted_token.header().key_id, &jwk.key_id) {
            (Some(expected), Some(kid)) => expected == kid,
            (Some(_), None) => false,
            (None, _) => true,
        })
        .filter_map(|jwk| match Alg::try_from(untrusted_token.algorithm()).ok()? {
            Alg::HS256 => decode(Hs256, jwk, &untrusted_token),
            Alg::HS384 => decode(Hs384, jwk, &untrusted_token),
            Alg::HS512 => decode(Hs512, jwk, &untrusted_token),
            Alg::ES256 => decode(Es256, jwk, &untrusted_token),
            Alg::RS256 => decode(Rsa::rs256(), jwk, &untrusted_token),
            Alg::RS384 => decode(Rsa::rs384(), jwk, &untrusted_token),
            Alg::RS512 => decode(Rsa::rs512(), jwk, &untrusted_token),
            Alg::PS256 => decode(Rsa::ps256(), jwk, &untrusted_token),
            Alg::PS384 => decode(Rsa::ps384(), jwk, &untrusted_token),
            Alg::PS512 => decode(Rsa::ps512(), jwk, &untrusted_token),
            Alg::EdDSA => decode(Ed25519, jwk, &untrusted_token),
        })
        .find(|token| {
            token
                .claims()
                .validate_expiration(&time_options)
                .and_then(|claims| {
                    if claims.not_before.is_some() {
                        claims.validate_maturity(&time_options)
                    } else {
                        Ok(claims)
                    }
                })
                .is_ok()
        })
}

fn decode<A: Algorithm, T: DeserializeOwned>(
    alg: A,
    jwk: &JsonWebKey<'_>,
    untrusted_token: &UntrustedToken<'_>,
) -> Option<jwt_compact::Token<T>>
where
    A::VerifyingKey: std::fmt::Debug + for<'a> TryFrom<&'a JsonWebKey<'a>>,
{
    let key = A::VerifyingKey::try_from(jwk).ok()?;

    alg.validator(&key).validate(untrusted_token).ok()
}
