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

    // REVIEW: we use a mut self because we want to store the authentication information like
    // tokens, session id or cookies in the struct we implement the trait for.
    fn authenticate(&mut self) -> Result<A, SSOError>;

    fn roles(&self) -> Result<Vec<Self::R>, SSOError>;

    // REVIEW: do we need policies or should this be implemented on demand for the needed
    // servertype?
    fn policies(&self) -> Result<Vec<Self::P>, SSOError>;
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
    fn new(server: S) -> Self {
        let roles = match &server.roles() {
            Ok(roles) => Some(roles.to_vec()),
            Err(..) => None,
        };
        let policies = match &server.policies() {
            Ok(policies) => Some(policies.to_vec()),
            Err(..) => None,
        };
        SSOConfig {
            server,
            roles,
            policies,
        }
    }
}
