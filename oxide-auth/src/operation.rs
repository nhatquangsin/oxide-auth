use crate::oxide_auth_actix::{OAuthRequest, OAuthResponse, OAuthOperation, WebError};
use crate::{
    endpoint::{AccessTokenFlow, AuthorizationFlow, Endpoint, RefreshFlow, ResourceFlow},
    primitives::grant::Grant,
};

/// Authorization-related operations
pub struct Authorize(pub OAuthRequest);

impl OAuthOperation for Authorize {
    type Item = OAuthResponse;
    type Error = WebError;

    fn run<E>(self, endpoint: E) -> Result<Self::Item, Self::Error>
    where
        E: Endpoint<OAuthRequest>,
        WebError: From<E::Error>,
    {
        AuthorizationFlow::prepare(endpoint)?
            .execute(self.0)
            .map_err(WebError::from)
    }
}

/// Token-related operations
pub struct Token(pub OAuthRequest);

impl OAuthOperation for Token {
    type Item = OAuthResponse;
    type Error = WebError;

    fn run<E>(self, endpoint: E) -> Result<Self::Item, Self::Error>
    where
        E: Endpoint<OAuthRequest>,
        WebError: From<E::Error>,
    {
        AccessTokenFlow::prepare(endpoint)?
            .execute(self.0)
            .map_err(WebError::from)
    }
}

/// Refresh-related operations
pub struct Refresh(pub OAuthRequest);

impl OAuthOperation for Refresh {
    type Item = OAuthResponse;
    type Error = WebError;

    fn run<E>(self, endpoint: E) -> Result<Self::Item, Self::Error>
    where
        E: Endpoint<OAuthRequest>,
        WebError: From<E::Error>,
    {
        RefreshFlow::prepare(endpoint)?
            .execute(self.0)
            .map_err(WebError::from)
    }
}

/// Resource-related operations
pub struct Resource(pub OAuthRequest);

impl OAuthOperation for Resource {
    type Item = Grant;
    type Error = Result<OAuthResponse, WebError>;

    fn run<E>(self, endpoint: E) -> Result<Self::Item, Self::Error>
    where
        E: Endpoint<OAuthRequest>,
        WebError: From<E::Error>,
    {
        ResourceFlow::prepare(endpoint)
            .map_err(|e| Err(WebError::from(e)))?
            .execute(self.0)
            .map_err(|r| r.map_err(WebError::from))
    }
}
