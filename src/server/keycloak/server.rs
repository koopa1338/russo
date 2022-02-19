use futures::executor;
use reqwest::{Client, Method};
use serde::Deserialize;

use super::model::{KeycloakAuth, KeycloakGroup, KeycloakRole};
use crate::{SSOConnector, SSOError};

#[allow(dead_code)]
pub struct KeycloakServer {
    url: String,
    realm: String,
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
    roles: Option<Vec<KeycloakRole>>,
    groups: Option<Vec<KeycloakGroup>>,
    auth: Option<KeycloakAuth>,
    client: Client,
}

// REVIEW: Check if we should do some kind of caching mechanism to minimize requests
impl KeycloakServer {
    pub fn new(
        url: String,
        realm: String,
        client_id: String,
        client_secret: String,
        username: String,
        password: String,
    ) -> Self {
        Self {
            url,
            realm,
            client_id,
            client_secret,
            username,
            password,
            client: Client::new(),
            roles: None,
            groups: None,
            auth: None,
        }
    }

    pub fn keycloak_request<T>(&self, url: String, method: Method) -> Result<T, SSOError>
    where
        T: Clone,
        for<'de> T: Deserialize<'de> + 'static,
    {
        let result = executor::block_on(
            self.client
                .request(method, url)
                .bearer_auth(&self.auth.as_ref().unwrap().access_token)
                .send(),
        );
        Ok(executor::block_on(result.unwrap().json::<T>()).unwrap())
    }

    pub fn role_by_id(&mut self, role_id: String) -> Result<KeycloakRole, SSOError> {
        let roles = match &self.roles {
            None => {
                // not fetched the roles from the server
                self.roles()?
            }
            Some(roles) => roles.to_vec(),
        };

        let role = roles
            .iter()
            .find(|&r| *r.id.as_ref().unwrap() == role_id)
            .unwrap()
            .clone();
        Ok(role)
    }

    pub fn role_by_name(&mut self, role_name: String) -> Result<KeycloakRole, SSOError> {
        let roles = match &self.roles {
            None => {
                // not fetched the roles from the server
                self.roles()?
            }
            Some(roles) => roles.to_vec(),
        };

        let role = roles
            .iter()
            .find(|&r| *r.name.as_ref().unwrap() == role_name)
            .unwrap()
            .clone();
        Ok(role)
    }

    pub fn groups(&mut self) -> Result<Vec<KeycloakGroup>, SSOError> {
        let groups = self.keycloak_request::<Vec<KeycloakGroup>>(
            format!(
                "{}/{}/clients/{}/groups",
                self.url, self.realm, self.client_id
            ),
            Method::GET,
        )?;
        self.groups = Some(groups.clone());
        Ok(groups)
    }
}

impl SSOConnector<KeycloakAuth> for KeycloakServer {
    type R = KeycloakRole;

    fn authenticate(&mut self) -> Result<KeycloakAuth, SSOError> {
        let auth = self.keycloak_request::<KeycloakAuth>(
            format!(
                "{}/auth/realms/{}/protocol/openid-connect/token",
                self.url, self.realm
            ),
            Method::POST,
        )?;
        self.auth = Some(auth.clone());
        Ok(auth)
    }

    fn roles(&mut self) -> Result<Vec<KeycloakRole>, SSOError> {
        let roles = self.keycloak_request::<Vec<KeycloakRole>>(
            format!(
                "{}/{}/clients/{}/roles",
                self.url, self.realm, self.client_id
            ),
            Method::GET,
        )?;
        self.roles = Some(roles.clone());
        Ok(roles)
    }
}
