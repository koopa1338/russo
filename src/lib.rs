mod russoerror;
mod server;

use russoerror::SSOError;

/// A SSO connector trait that is generic over a server authentication type A and has a associated
/// type R that represents a role to enable role based permissions
/// The authentication type is a struct that represents a authentication state with data that th
/// server expects. See one of the default implementations.
trait SSOConnector<A> {
    type R: Clone;

    /// Returns the authentication type A if succesful or a SSOError if authentication fails.
    fn authenticate(&mut self) -> Result<A, SSOError>;

    /// Returns a vec of the associated role type or a SSOError if the request fails.
    fn roles(&mut self) -> Result<Vec<Self::R>, SSOError>;
}
