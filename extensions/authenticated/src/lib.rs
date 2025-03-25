use grafbase_sdk::{
    AuthorizationExtension, IntoQueryAuthorization,
    types::{AuthorizationDecisions, Configuration, Error, ErrorResponse, QueryElements, SubgraphHeaders, Token},
};

#[derive(AuthorizationExtension)]
struct Authenticated;

impl AuthorizationExtension for Authenticated {
    fn new(_: Configuration) -> Result<Self, Error> {
        Ok(Self)
    }

    fn authorize_query(
        &mut self,
        _headers: &mut SubgraphHeaders,
        token: Token,
        _elements: QueryElements<'_>,
    ) -> Result<impl IntoQueryAuthorization, ErrorResponse> {
        Ok(if token.is_anonymous() {
            AuthorizationDecisions::deny_all("Not authenticated")
        } else {
            AuthorizationDecisions::grant_all()
        })
    }
}
