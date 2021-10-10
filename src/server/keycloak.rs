use uuid::Uuid;
use std::collections::HashMap;

use crate::Role;

enum DecisionStrat {
    Affirmative,
    Unanimous,
    Consensus,
}

enum Logic {
    Positive,
    Negative,
}

pub struct KeycloakServer {
    realm: String,
    groups: Option<Vec<String>>,
}

struct KeycloakRole {
    attributes: Option<HashMap<String, String>>, //REVIEW: not sure what the types are here, use strings for now
    client_role: Option<bool>,
    composite: Option<bool>,
    // composites: do we need this, if so we need a struct for that
    description: Option<String>,
    id: Option<Uuid>,
    name: Option<String>,
}

impl Role for KeycloakRole {
    fn name(&self) -> String {
       self.name.clone().unwrap_or("No role name.".to_string())
    }
}

struct KeycloakPolicy {
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

struct KeycloakGroup {
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
