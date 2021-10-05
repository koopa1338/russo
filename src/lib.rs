use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Role {
    id: usize,
    pub name: String,
}

#[derive(Debug, Clone)]
struct Policy {
    id: usize,
    pub name: String,
    pub roles: Arc<Vec<Role>>,
    // maybe save the rule but how does that look like?
}

impl Policy {
    pub fn role_is_authorized(&self, role: Role) -> bool {
        self.roles.contains(&role)  
    }
}
