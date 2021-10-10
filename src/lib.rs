mod russoerror;
mod server;

use russoerror::SSOError;

trait Role {
    fn name(&self) -> String;
}

trait Policy {
    type R: Role;

    fn role_is_authorized(&self, role: Self::R) -> bool;
}

trait SSOConnector {
    type R: Role;
    type P: Policy;

    fn connect(&self, user: String, password: String) -> Result<bool, SSOError>;

    fn get_roles(&self) -> Vec<Self::R>;

    fn get_policies(&self) -> Vec<Self::P>;
}

struct SSOConfig<S>
where
    S: SSOConnector,
{
    url: String,
    server: S,
    user: String,
    password: String,
    roles: Option<Vec<<S as SSOConnector>::R>>,
    policies: Option<Vec<<S as SSOConnector>::P>>,
}

impl<S> SSOConfig<S>
where
    S: SSOConnector,
{
    fn new(url: String, server: S, user: String, password: String) -> Self {
        let roles = server.get_roles();
        let policies = server.get_policies();
        SSOConfig {
            url,
            server,
            user,
            password,
            roles: Some(roles),
            policies: Some(policies),
        }
    }
}
