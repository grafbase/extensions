use base64::Engine as _;
use grafbase_sdk::{
    host_io::{
        cache::{self, CachedItem},
        http::{self, HttpRequest},
    },
    types::{ErrorResponse, Token},
};
use jwt_compact::{Algorithm, AlgorithmExt, TimeOptions, UntrustedToken, jwk::JsonWebKey};
use serde::de::DeserializeOwned;
use std::borrow::Cow;

use crate::Config;

pub(crate) struct Decoder<'a> {
    config: &'a Config,
    jwks: Option<Jwks<'static>>,
}

impl<'a> Decoder<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config, jwks: None }
    }

    pub fn decode(&mut self, token_str: &str) -> Option<Result<Token, ErrorResponse>> {
        let token = UntrustedToken::new(&token_str).ok()?;
        let token = match self.jwks() {
            Ok(jwks) => decode_untrusted_token(&jwks.keys, token)?,
            Err(err) => {
                return Some(Err(err));
            }
        };

        if let Some(expected) = self.config.issuer.as_ref() {
            if token.claims().custom.issuer.as_ref() != Some(expected) {
                return None;
            }
        }

        if let Some(expected) = self.config.audience.as_ref() {
            let aud_claims = token.claims().custom.audience.as_ref()?;
            if aud_claims.iter().all(|claim| !expected.contains(claim)) {
                return None;
            }
        }

        // We just validated the JWT token. Instead of de-serializing and re-serializing the
        // payload, we re-use the original token payload.
        let [_headers, payload, _signature] = token_str.split('.').collect::<Vec<_>>()[..] else {
            unreachable!("Token was successfully validated");
        };

        Some(Ok(Token::from_bytes(
            base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(payload)
                .expect("Token was successfully validated"),
        )))
    }

    fn jwks(&mut self) -> Result<&Jwks<'static>, ErrorResponse> {
        if self.jwks.is_none() {
            let jwks = cache::get("jwks", || {
                let request = HttpRequest::get(self.config.url.clone()).build();
                let response = http::execute(request)?;
                let jwks: Jwks = response.json()?;

                Ok(CachedItem::new(jwks, Some(self.config.poll_interval)))
            })
            .map_err(|_| ErrorResponse::internal_server_error())?;
            self.jwks = Some(jwks);
        }
        Ok(self.jwks.as_ref().unwrap())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Jwks<'a> {
    keys: Vec<Jwk<'a>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Jwk<'a> {
    #[serde(flatten)]
    key: JsonWebKey<'a>,
    #[serde(rename = "kid")]
    key_id: Option<Cow<'a, str>>,
}

#[serde_with::serde_as]
#[derive(Debug, serde::Deserialize)]
struct CustomClaims {
    #[serde(default, rename = "iss")]
    issuer: Option<String>,
    #[serde_as(deserialize_as = "Option<serde_with::OneOrMany<_>>")]
    #[serde(default, rename = "aud")]
    audience: Option<Vec<String>>,
}

impl<'a> std::ops::Deref for Jwk<'a> {
    type Target = JsonWebKey<'a>;

    fn deref(&self) -> &Self::Target {
        &self.key
    }
}

#[derive(Debug, strum::EnumString)]
enum Alg {
    HS256,
    HS384,
    HS512,
    ES256,
    RS256,
    RS384,
    RS512,
    PS256,
    PS384,
    PS512,
    EdDSA,
}

fn decode_untrusted_token(
    jwks: &[Jwk<'_>],
    untrusted_token: UntrustedToken<'_>,
) -> Option<jwt_compact::Token<CustomClaims>> {
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
