use futures::executor;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Policy, SSOConnector, SSOError};

#[derive(Clone)]
enum DecisionStrat {
    Affirmative,
    Unanimous,
    Consensus,
}

#[derive(Clone)]
enum Logic {
    Positive,
    Negative,
}

pub struct KeycloakServer {
    url: String,
    realm: String,
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
    groups: Option<Vec<String>>,
    auth: Option<KeycloakAuth>,
}

impl KeycloakServer {
    pub fn roles_by_id(&self, role_id: String) {
        unimplemented!();
    }

    pub fn roles_by_name(&self, role_name: String) {
        unimplemented!();
    }

    pub fn groups(&self) -> Vec<KeycloakGroup> {
        unimplemented!();
    }
}

#[derive(Clone, Deserialize)]
pub struct KeycloakRole {
    attributes: Option<HashMap<String, String>>, //REVIEW: not sure what the types are here, use strings for now
    client_role: Option<bool>,
    composite: Option<bool>,
    // composites: do we need this, if so we need a struct for that
    description: Option<String>,
    id: Option<String>,
    name: Option<String>,
}

#[derive(Clone)]
pub struct KeycloakPolicy {
    config: Option<HashMap<String, String>>, //REVIEW: not sure what the types are here, use strings for now
    decision_strat: Option<DecisionStrat>,
    description: Option<String>,
    id: Option<String>,
    logic: Option<Logic>,
    owner: Option<String>,
    policies: Option<Vec<String>>,
    resources: Option<Vec<String>>,
    // recources_data: do we need this, if so we need a struct for that
    scopes: Option<Vec<String>>,
    // scopes_data: do we need this, if so we need a struct for that
    policy_type: Option<String>,
}

impl Policy for KeycloakPolicy {
    type R = KeycloakRole;

    fn role_is_authorized(&self, role: Self::R) -> bool {
        unimplemented!();
    }
}

pub struct KeycloakGroup {
    access: Option<HashMap<String, String>>, //REVIEW: not sure what the types are here, use strings for now
    attributes: Option<HashMap<String, String>>, //REVIEW: not sure what the types are here, use strings for now
    client_roles: Option<HashMap<String, String>>, //REVIEW: not sure what the types are here, use strings for now
    id: Option<String>,
    realm_roles: Option<Vec<String>>,
    sub_groups: Option<Vec<KeycloakGroup>>,
}

#[derive(Clone, Deserialize)]
struct KeycloakAuth {
    access_token: String,
    access_token_expires: usize,
    refresh_token: String,
    refresh_token_expires: usize,
    scopes: Vec<String>,
    token_type: String, //maybe enum?
    session_state: String,
}

#[derive(Serialize, Debug)]
struct KeycloakAuthRequest {
    client_id: String,
    username: String,
    password: String,
    grant_type: String,
}

impl SSOConnector<KeycloakAuth> for KeycloakServer {
    type R = KeycloakRole;

    type P = KeycloakPolicy;

    fn authenticate(&mut self) -> Result<KeycloakAuth, SSOError> {
        let client = Client::new();
        let result = executor::block_on(
            client
                .request(
                    Method::POST,
                    format!(
                        "{}/auth/realms/{}/protocol/openid-connect/token",
                        self.url, self.realm
                    ),
                )
                .form(&KeycloakAuthRequest {
                    client_id: self.client_id.clone(),
                    username: self.username.clone(),
                    password: self.password.clone(),
                    grant_type: "password".into(),
                })
                .send(),
        );
        let auth = executor::block_on(result.unwrap().json::<KeycloakAuth>()).unwrap();
        self.auth = Some(auth.clone());
        Ok(auth)
    }

    fn roles(&self) -> Result<Vec<KeycloakRole>, SSOError> {
        let client = Client::new();
        let result = executor::block_on(
            client
                .request(
                    Method::GET,
                    format!(
                        "{}/{}/clients/{}/roles",
                        self.url, self.realm, self.client_id
                    ),
                )
                .bearer_auth(&self.auth.as_ref().unwrap().access_token)
                .send(),
        );
        Ok(executor::block_on(result.unwrap().json::<Vec<KeycloakRole>>()).unwrap())
    }

    fn policies(&self) -> Result<Vec<KeycloakPolicy>, SSOError> {
        unimplemented!();
    }
}
