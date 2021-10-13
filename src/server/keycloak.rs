use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

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
}

#[derive(Clone)]
pub struct KeycloakRole {
    attributes: Option<HashMap<String, String>>, //REVIEW: not sure what the types are here, use strings for now
    client_role: Option<bool>,
    composite: Option<bool>,
    // composites: do we need this, if so we need a struct for that
    description: Option<String>,
    id: Option<Uuid>,
    name: Option<String>,
}

#[derive(Clone)]
pub struct KeycloakPolicy {
    config: Option<HashMap<String, String>>, //REVIEW: not sure what the types are here, use strings for now
    decision_strat: Option<DecisionStrat>,
    description: Option<String>,
    id: Option<Uuid>,
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
    id: Option<Uuid>,
    realm_roles: Option<Vec<String>>,
    sub_groups: Option<Vec<KeycloakGroup>>,
}

impl KeycloakServer {
    pub fn get_roles(&self) {
        unimplemented!();
    }

    pub fn get_roles_by(&self, role_id: Uuid) {
        unimplemented!();
    }

    pub fn get_groups(&self) -> Vec<KeycloakGroup> {
        unimplemented!();
    }
}

struct KeycloakAuth {
    // TODO: check if the token is all we get.
    access_token: String,
    session_token: String,
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

    /* TODO: we have 2 options here:
     * 1. we use async functions inside authenticate and have to poll the result from the future
     * 2. we make all functions async -> async traits seem very complicated and might not be worth it
     */
    fn authenticate(&self) -> Result<KeycloakAuth, SSOError> {
        let result = async {
            let client = Client::new();
            client
                .post(format!(
                    "{}/auth/realms/{}/protocol/openid-connect/token",
                    self.url, self.realm
                ))
                .form(&KeycloakAuthRequest {
                    client_id: self.client_id.clone(),
                    username: self.username.clone(),
                    password: self.password.clone(),
                    grant_type: "password".into(),
                })
                .send()
        };
        // TODO: get KeycloakAuth from the result future
        Ok(KeycloakAuth {
            access_token: "bearer token".into(),
            session_token: "session".into(),
        })
    }

    fn get_roles(&self) -> Vec<Self::R> {
        unimplemented!();
    }

    fn get_policies(&self) -> Vec<Self::P> {
        unimplemented!();
    }
}
