use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Keycloak representation of a role, see the keycloak documentation for more information:
/// https://www.keycloak.org/docs-api/16.0/rest-api/index.html#_rolerepresentation
#[derive(Clone, Deserialize)]
pub struct KeycloakRole {
    pub attributes: Option<HashMap<String, String>>, //REVIEW: not sure what the types are here, use strings for now
    pub client_role: Option<bool>,
    pub composite: Option<bool>,
    // composites: do we need this, if so we need a struct for that
    pub description: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}

/// Keycloak representation of a group, see the keycloak documentation for more information:
/// https://www.keycloak.org/docs-api/16.0/rest-api/index.html#_grouprepresentation
#[derive(Clone, Deserialize)]
pub struct KeycloakGroup {
    pub access: Option<HashMap<String, String>>, //REVIEW: not sure what the types are here, use strings for now
    pub attributes: Option<HashMap<String, String>>, //REVIEW: not sure what the types are here, use strings for now
    pub client_roles: Option<HashMap<String, String>>, //REVIEW: not sure what the types are here, use strings for now
    pub id: Option<String>,
    pub realm_roles: Option<Vec<String>>,
    pub sub_groups: Option<Vec<KeycloakGroup>>,
}

#[derive(Clone, Deserialize)]
pub struct KeycloakAuth {
    pub access_token: String,
    pub access_token_expires: usize,
    pub refresh_token: String,
    pub refresh_token_expires: usize,
    pub scopes: Vec<String>,
    pub token_type: String, //maybe enum?
    pub session_state: String,
}

#[derive(Serialize, Debug)]
struct KeycloakAuthRequest {
    client_id: String,
    username: String,
    password: String,
    grant_type: String,
}
