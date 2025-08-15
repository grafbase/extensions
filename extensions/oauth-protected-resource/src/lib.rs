use grafbase_sdk::{
    AuthenticationExtension,
    types::{Configuration, Error, ErrorResponse, GatewayHeaders, Headers, PublicMetadataEndpoint, RequestContext, Token},
};
use oauth_protected_resource_shared::OAuthConfig;

#[derive(AuthenticationExtension)]
struct OauthProtectedResourceMetadata(OAuthConfig);

impl AuthenticationExtension for OauthProtectedResourceMetadata {
    fn new(config: Configuration) -> Result<Self, Error> {
        let config = config.deserialize()?;

        Ok(Self(config))
    }

    fn authenticate(&mut self, _ctx: &RequestContext, _headers: &GatewayHeaders) -> Result<Token, ErrorResponse> {
        Ok(Token::anonymous())
    }

    fn public_metadata(&mut self) -> Result<Vec<PublicMetadataEndpoint>, Error> {
        let OauthProtectedResourceMetadata(config) = self;

        let mut response_body = config.metadata.other_parameters.clone();
        response_body.insert("resource".to_owned(), config.metadata.resource.clone().into());
        let response_body = serde_json::to_vec(&response_body).map_err(|err| Error::new(err.to_string()))?;

        let mut response_headers = Headers::new();

        response_headers.append("content-type", "application/json");

        Ok(vec![
            PublicMetadataEndpoint::new(config.metadata_path.clone(), response_body).with_headers(response_headers),
        ])
    }
}
