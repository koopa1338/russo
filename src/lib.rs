mod russoerror;
mod server;

use russoerror::SSOError;

trait Policy {
    type R: Clone;

    fn role_is_authorized(&self, role: Self::R) -> bool;
}

trait SSOConnector<A> {
    type R: Clone;
    type P: Policy + Clone;

    fn authenticate(&self) -> Result<A, SSOError>;

    fn get_roles(&self) -> Vec<Self::R>;

    fn get_policies(&self) -> Vec<Self::P>;
}

struct SSOConfig<S, A>
where
    S: SSOConnector<A>,
{
    server: S,
    roles: Option<Vec<<S as SSOConnector<A>>::R>>,
    policies: Option<Vec<<S as SSOConnector<A>>::P>>,
}

impl<S: Clone, A> SSOConfig<S, A>
where
    S: SSOConnector<A>,
{
    fn new(url: String, server: S) -> Self {
        let roles = &server.get_roles();
        let policies = &server.get_policies();
        SSOConfig {
            server,
            roles: Some(roles.to_vec()),
            policies: Some(policies.to_vec()),
        }
    }
}
