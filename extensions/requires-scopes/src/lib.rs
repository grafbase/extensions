use grafbase_sdk::{
    AuthorizationExtension, IntoQueryAuthorization,
    types::{AuthorizationDecisions, Configuration, Error, ErrorResponse, QueryElements, SubgraphHeaders, Token},
};

#[derive(AuthorizationExtension)]
struct RequiresScopes;

#[derive(serde::Deserialize)]
struct Claims<'a> {
    #[serde(borrow)]
    scope: &'a str,
}

#[derive(serde::Deserialize)]
struct DirectiveArguments<'a> {
    #[serde(borrow)]
    scopes: Vec<Vec<&'a str>>,
}

impl AuthorizationExtension for RequiresScopes {
    fn new(_config: Configuration) -> Result<Self, Error> {
        Ok(Self)
    }

    fn authorize_query(
        &mut self,
        _headers: &mut SubgraphHeaders,
        token: Token,
        elements: QueryElements<'_>,
    ) -> Result<impl IntoQueryAuthorization, ErrorResponse> {
        let Some(bytes) = token.into_bytes() else {
            // Anonymous user.
            return Ok(AuthorizationDecisions::deny_all("Not authorized"));
        };
        let Ok(Claims { scope }) = serde_json::from_slice(&bytes) else {
            // Unsupported token.
            return Ok(AuthorizationDecisions::deny_all("Not authorized: unsupported token."));
        };
        let token_scopes = scope.split(' ').collect::<Vec<_>>();

        let mut builder = AuthorizationDecisions::deny_some_builder();
        let mut lazy_error_id = None;

        for element in elements {
            let DirectiveArguments { scopes } = element.directive_arguments::<DirectiveArguments>()?;
            let has_matching_scopes = scopes
                .iter()
                .any(|scopes| scopes.iter().all(|scope| token_scopes.contains(scope)));

            if !has_matching_scopes {
                let error_id =
                    *lazy_error_id.get_or_insert_with(|| builder.push_error("Not authorized: insufficient scopes"));
                // We re-use the same GraphQL error here to avoid sending duplicate data back to
                // the gateway. The GraphQL response will have an individual error for each element
                // however.
                builder.deny_with_error_id(element, error_id);
            }
        }

        Ok(builder.build())
    }
}
